import contextlib
import hashlib
import importlib.util
import io
import json
from pathlib import Path
import tempfile
import unittest
from unittest.mock import patch

spec=importlib.util.spec_from_file_location("adapter",Path(__file__).with_name("reference-first-codex.py"))
adapter=importlib.util.module_from_spec(spec);spec.loader.exec_module(adapter)

class ReferenceFirstAdapter(unittest.TestCase):
    def test_candidate_blind_dispatch_and_receipt(self):
        with tempfile.TemporaryDirectory() as temp:
            image=Path(temp)/"reference.png";image.write_bytes(b"reference")
            request={"protocol":"reference-first-v1","target_species":"Example species","specimen_relationship":"unknown","references":[{"id":"reference-0","image":{"path":str(image),"sha256":hashlib.sha256(b"reference").hexdigest(),"view":"whole","seed":0}}]}
            envelope={"stage":"inventory","request":request,"request_sha256":"bound-by-rust","prompt":"Neutral inventory instruction","prompt_sha256":hashlib.sha256(b"Neutral inventory instruction").hexdigest()}
            def run(command,**kwargs):
                self.assertEqual(command.count("--image"),1)
                self.assertNotIn("OWNER_SECRET",command[-1])
                self.assertIn('"specimen_relationship": "unknown"',command[-1])
                self.assertIn("--ephemeral",command)
                Path(command[command.index("-o")+1]).write_text(json.dumps({"traits":[],"observations":["literal"]}))
                return type("Done",(),{"returncode":0,"stdout":b'{"type":"turn.completed","usage":{"input_tokens":2,"output_tokens":3}}\n',"stderr":b""})()
            out=io.StringIO()
            with patch("sys.argv",["adapter","--model","mock","--effort","medium"]),patch("sys.stdin",io.StringIO(json.dumps(envelope))),patch.object(adapter.subprocess,"run",side_effect=run) as call,contextlib.redirect_stdout(out):
                adapter.main()
            self.assertEqual(call.call_count,1)
            self.assertEqual(json.loads(out.getvalue())["answer"]["observations"],["literal"])
            for code, tool in [(0, True), (1, False)]:
                def failed(command, **kwargs):
                    Path(command[command.index("-o")+1]).write_text('{"traits":[],"observations":[]}')
                    events = ('{"type":"item.completed","item":{"type":"command_execution"}}\n' if tool else '') + '{"type":"turn.completed","usage":{"input_tokens":2,"output_tokens":3}}\n'
                    return type("Done",(),{"returncode":code,"stdout":events.encode(),"stderr":b"failure"})()
                captured=io.StringIO()
                with patch("sys.argv",["adapter","--model","mock","--effort","medium"]),patch("sys.stdin",io.StringIO(json.dumps(envelope))),patch.object(adapter.subprocess,"run",side_effect=failed),contextlib.redirect_stdout(captured):
                    adapter.main()
                receipt=json.loads(captured.getvalue())
                self.assertEqual(receipt["status"],"failed_or_tools_or_unknown_usage_or_cardinality")
                self.assertEqual(receipt["usage"]["input_tokens"],2)
            _,inventory_schema,_=adapter.prepare(envelope)
            self.assertEqual(inventory_schema["properties"]["traits"]["maxItems"],16)
            self.assertEqual(inventory_schema["properties"]["observations"]["maxItems"],16)
            self.assertEqual(inventory_schema["properties"]["traits"]["items"]["properties"]["id"]["maxLength"],64)
            comparison=dict(envelope,stage="comparison",request={"comparison":{"required":[{"view":"whole"},{"view":"bare"}],"images":[],"references":[request["references"][0]["image"]],"quality_anchors":[]}})
            _,schema,prompt=adapter.prepare(comparison)
            self.assertEqual(schema["properties"]["passes"]["minItems"],2)
            self.assertEqual(schema["properties"]["passes"]["maxItems"],2)
            self.assertEqual(schema["properties"]["coverage"]["maxItems"],16)
            self.assertEqual(schema["properties"]["findings"]["maxItems"],16)
            self.assertIn("exact order",prompt)
            def wrong_count(command,**kwargs):
                Path(command[command.index("-o")+1]).write_text('{"passes":["fail"]}')
                return type("Done",(),{"returncode":0,"stdout":b'{"type":"turn.completed","usage":{"input_tokens":2,"output_tokens":3}}\n',"stderr":b""})()
            captured=io.StringIO()
            with patch("sys.argv",["adapter","--model","mock","--effort","medium"]),patch("sys.stdin",io.StringIO(json.dumps(comparison))),patch.object(adapter.subprocess,"run",side_effect=wrong_count),contextlib.redirect_stdout(captured):
                adapter.main()
            self.assertNotEqual(json.loads(captured.getvalue())["status"],"ok")
            request["candidate"]="OWNER_SECRET"
            with self.assertRaises(ValueError):adapter.prepare(envelope)
            del request["candidate"]
            image.write_bytes(b"changed")
            with self.assertRaises(ValueError):adapter.prepare(envelope)

if __name__=="__main__":unittest.main()
