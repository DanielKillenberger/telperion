"""One evidence-only native-image Grok comparison; --execute is explicit."""
import argparse
import base64
import importlib.util
import json
import mimetypes
import os
from pathlib import Path
import subprocess
import tempfile
import time

spec=importlib.util.spec_from_file_location("shared",Path(__file__).with_name("claude-comparison.py"))
shared=importlib.util.module_from_spec(spec);spec.loader.exec_module(shared)


def parse_result(raw):
    result=json.loads(raw)
    if result.get("stopReason")!="end_turn" or result.get("structuredOutputError"):
        raise ValueError("unsuccessful/unknown result; retain reservation")
    usage=result.get("usage",{})
    keys=("input_tokens","cache_read_input_tokens","cache_creation_input_tokens","output_tokens","total_tokens")
    if any(type(usage.get(k)) is not int or usage[k]<0 for k in keys):raise ValueError("unknown usage")
    if sum(usage[k] for k in keys[:-1])!=usage["total_tokens"]:raise ValueError("usage total mismatch")
    models=result.get("modelUsage",{})
    if list(models)!=["grok-4.6"] or models["grok-4.6"].get("modelCalls")!=1 or result.get("num_turns")!=1:
        raise ValueError("unexpected model/extra model calls; no captioning substitution")
    answer=result.get("structuredOutput")
    if not isinstance(answer,dict):raise ValueError("missing structured output")
    return {"answer":answer,"usage":usage,"actual_tokens":usage["total_tokens"],"actual_model":"grok-4.6","model_usage":models}


def main():
    parser=argparse.ArgumentParser();parser.add_argument("--execute",action="store_true");args=parser.parse_args()
    request,payload=shared.frozen_payload()
    assert shared.digest(payload["prompt"].encode())=="61d328c368f40283b4ea1ddcc1e23ac03f620257515befa2874b5289477e7b70"
    prior_path=shared.ROOT/"joint-blind-opus-journal.json";prior_bytes=prior_path.read_bytes();prior=json.loads(prior_bytes)
    assert prior["status"]=="settled" and prior["cumulative_actual_tokens"]==430972
    assert prior["actual_model"].startswith("claude-opus-")
    journal={"authority":"Owner YES to520k/Fable Opus Grok3x40k no retries/new renders; host releases ONE Grok4.6 high after known Claude success",
             "status":"prepared_no_call","prior_journal":str(prior_path),"prior_journal_sha256":shared.digest(prior_bytes),
             "prior_actual_tokens":430972,"reserved_tokens":40000,"cumulative_cap":520000,"attempted_visual_passes":17,
             "request_sha256":shared.REQUEST_HASH,"protocol_sha256":shared.PROTOCOL_HASH,
             "user_prompt_sha256":shared.digest(payload["prompt"].encode()),"model":"grok-4.6","effort":"high",
             "images":[{"sha256":shared.digest(Path(p).read_bytes()),"path":p} for p in payload["paths"]],
             "system_context":"Native provider differs; neutral system override, scratchcwd, no-memory flag disables legacy/v2, no-plan/subagents/web, explicit filtered builtin plus deny rules; same user prompt/schema/image bytes"}
    if not args.execute:print(json.dumps(journal));return
    assert 430972+40000<=520000
    runtime=shared.WORKTREE/".flow/tmp/fn68-pilot-run/run.json";journal["original_runtime_sha256"]=shared.digest(runtime.read_bytes())
    journal["status"]="reserved_before_dispatch";jp=shared.ROOT/"joint-blind-grok-journal.json"
    with jp.open("x") as f:json.dump(journal,f,indent=2)
    blocks=[{"type":"text","text":payload["prompt"]}]
    for p in payload["paths"]:
        media=mimetypes.guess_type(p)[0];assert media in ("image/png","image/jpeg")
        blocks.append({"type":"image","data":base64.b64encode(Path(p).read_bytes()).decode(),"mimeType":media})
    started=time.monotonic()
    with tempfile.TemporaryDirectory(prefix="fn68-grok-blind-") as scratch:
        prompt=Path(scratch)/"input.json";prompt.write_text(json.dumps(blocks))
        command=["grok","--prompt-file",str(prompt),"--verbatim","--model","grok-4.6","--reasoning-effort","high",
                 "--json-schema",json.dumps(payload["schema"]),"--output-format","json","--max-turns","1",
                 "--no-memory","--no-subagents","--no-plan","--disable-web-search","--no-auto-update",
                 "--tools","read_file","--disallowed-tools","read_file,Agent",
                 "--system-prompt-override","You are a visual evidence reviewer. Follow the supplied user instructions. Do not use tools."]
        for rule in ("Bash","Read","Write","Edit","Grep","WebFetch","MCPTool"):command.extend(["--deny",rule])
        env=dict(os.environ);env["GROK_MEMORY"]="0"
        run=subprocess.run(command,cwd=scratch,env=env,stdin=subprocess.DEVNULL,capture_output=True,text=True,timeout=300)
    raw=shared.ROOT/"local/joint-blind-grok-stdout.json"
    with raw.open("x") as f:f.write(run.stdout)
    (shared.ROOT/"local/joint-blind-grok-stderr.txt").write_text(run.stderr)
    if run.returncode:raise RuntimeError("Grok failed; retain reservation, no retry")
    parsed=parse_result(run.stdout)
    answer=parsed["answer"];assert len(answer["passes"])==len(request["required"])
    ids={i["id"] for i in request["joint"]["inputs"]}
    for f in answer["findings"]:assert f["evidence_ids"] and set(f["evidence_ids"])<=ids and len(f["evidence_ids"])==len(set(f["evidence_ids"]))
    assert shared.digest(runtime.read_bytes())==journal["original_runtime_sha256"]
    journal.update(parsed,elapsed_seconds=time.monotonic()-started,cumulative_actual_tokens=430972+parsed["actual_tokens"],raw_response_sha256=shared.digest(run.stdout.encode()),status="settled" if parsed["actual_tokens"]<=40000 else "over_reservation_stop")
    jp.write_text(json.dumps(journal,indent=2));print(json.dumps({k:journal[k] for k in ("actual_model","usage","actual_tokens","cumulative_actual_tokens","status")}))
    assert parsed["actual_tokens"]<=40000


if __name__=="__main__":main()
