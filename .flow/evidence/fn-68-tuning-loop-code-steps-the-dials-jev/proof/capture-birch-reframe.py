"""One-shot historical-geometry birch camera repair. Reserve first. No retry."""
import hashlib, json, subprocess, time
from pathlib import Path

PROOF = Path(__file__).resolve().parent
ROOT = PROOF.parent
WORKTREE = ROOT.parents[2]
HIST = ROOT / "local/historical-birch"
BINARY = HIST / "target/release/examples/headless"
FIT = PROOF / "birch-framefit-result.json"
DIRECTORY = ROOT / "local/birch-reframed"
JOURNAL = PROOF / "birch-reframe-journal.json"
RUNTIME = WORKTREE / ".flow/tmp/fn68-pilot-run/run.json"
ORIGINAL = "d93259cd3e6980a19b09c3a1644d9f6114012ad7345bb0c02670941f151e885e"
CLIPPED = "2745f4c37a4410236f2adfb0d6b0c3903c7312bd8a15872ef8e58d4c35293d1e"


def sha(p):
    return hashlib.sha256(Path(p).read_bytes()).hexdigest()


def main():
    assert sha(RUNTIME) == ORIGINAL
    fit = json.loads(FIT.read_text())
    shot = fit["shot"]
    assert shot["new_max_corner_ndc"] <= 0.95
    camera = shot["camera"]
    light = shot["light"]
    overcast = light["overcast"]
    dim = 1 - 0.8 * overcast
    binary_sha = sha(BINARY)
    commands = []
    for twin in (False, True):
        scene = {
            "sunAzimuth": (camera["azimuth"] + 180) % 360 if twin else light["sunAzimuth"],
            "sunElevation": 5 if twin else light["sunElevation"],
            "sunRed": 3 * dim,
            "sunGreen": 2.85 * dim,
            "sunBlue": 2.6 * dim,
            "skyZenithRed": 0.18 + 0.37 * overcast,
            "skyZenithGreen": 0.30 + 0.36 * overcast,
            "skyZenithBlue": 0.62 + 0.18 * overcast,
        }
        out = DIRECTORY / ("S-WHOLE-twin.png" if twin else "S-WHOLE.png")
        commands.append(
            [
                str(BINARY),
                "--preset",
                "silver-birch",
                "--seed",
                "1",
                "--view",
                "whole",
                "--size",
                "x".join(map(str, shot["size"])),
                "--out",
                str(out),
                "--camera",
                json.dumps(camera),
                "--scene",
                json.dumps(scene),
                "--no-figure",
            ]
        )
    journal = {
        "authority": "Owner ok approved: in-scope historical-geometry camera repair; 2 stills; no generator change; no new candidate; no sweep; no recapture fishing",
        "grant_quotation": "ok approved",
        "prior_image_reservations": 28,
        "image_reservations": 30,
        "image_cap": 52,
        "remaining_after_reserve": 22,
        "prior_actual_images": 24,
        "actual_images": 24,
        "attempted_captures": 0,
        "runtime_sha256": ORIGINAL,
        "clipped_owner_accepted_sha256": CLIPPED,
        "new_raster_owner_accepted": False,
        "headless_sha256": binary_sha,
        "framefit_sha256": sha(FIT),
        "historical_revision": "0f2c2f6a",
        "fill": camera["fill"],
        "commands": commands,
        "captures": [],
        "status": "reserved_before_capture",
    }
    DIRECTORY.mkdir(exist_ok=False)
    with JOURNAL.open("x") as f:
        json.dump(journal, f, indent=2)
        f.write("\n")
    for command in commands:
        assert sha(BINARY) == binary_sha
        journal["attempted_captures"] += 1
        JOURNAL.write_text(json.dumps(journal, indent=2) + "\n")
        started = time.monotonic()
        result = subprocess.run(command, cwd=str(HIST), capture_output=True, text=True, timeout=300)
        out = Path(command[command.index("--out") + 1])
        (DIRECTORY / (out.stem + ".log")).write_text(result.stdout + result.stderr)
        assert result.returncode == 0, "capture failed; no retry"
        journal["captures"].append(
            {"path": str(out), "sha256": sha(out), "seconds": time.monotonic() - started}
        )
        journal["actual_images"] += 1
        JOURNAL.write_text(json.dumps(journal, indent=2) + "\n")
    assert sha(RUNTIME) == ORIGINAL
    journal["status"] = "two_captures_complete"
    JOURNAL.write_text(json.dumps(journal, indent=2) + "\n")
    print(json.dumps(journal["captures"]))


if __name__ == "__main__":
    main()
