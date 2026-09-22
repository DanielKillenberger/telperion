use std::{env, fs, path::PathBuf};
use telperion_jev::{
    caller::{load_key, UreqTransport},
    sha256_hex,
    tuning::{calibration, vision},
};

fn main() {
    if let Err(e) = run() {
        eprintln!("{e}");
        std::process::exit(1);
    }
}

fn run() -> Result<(), String> {
    let args = env::args().skip(1).collect::<Vec<_>>();
    let flag = |name: &str| {
        args.windows(2)
            .find(|a| a[0] == name)
            .map(|a| PathBuf::from(&a[1]))
            .ok_or_else(|| format!("missing {name}"))
    };
    let command = args.first().ok_or(
        "usage: tuning-loop <run|preflight> --config FILE --out DIR [--resume FILE]\n       tuning-loop inventory --config FILE --out DIR\n       tuning-loop freeze-replay --job FILE --out FILE\n       tuning-loop result --run DIR\n       tuning-loop <calibrate|vision-replay> --manifest FILE --out FILE [--adapter FILE]",
    )?;
    if command == "inventory" {
        let config: telperion_jev::tuning::live::Config =
            serde_json::from_slice(&fs::read(flag("--config")?).map_err(|e| e.to_string())?)
                .map_err(|e| e.to_string())?;
        let path = telperion_jev::tuning::inventory::run(&config, &flag("--out")?)?;
        println!("{}", path.display());
        return Ok(());
    }
    if command == "result" {
        let dir = flag("--run")?;
        let run: telperion_jev::tuning::engine::Run =
            serde_json::from_slice(&fs::read(dir.join("run.json")).map_err(|e| e.to_string())?)
                .map_err(|e| e.to_string())?;
        let result = run.end_result();
        fs::write(dir.join("result.json"), serde_json::to_vec_pretty(&result).unwrap())
            .map_err(|e| e.to_string())?;
        let page = telperion_jev::tuning::result::markdown(&result);
        fs::write(dir.join("RESULT.md"), &page).map_err(|e| e.to_string())?;
        print!("{page}");
        return Ok(());
    }
    if command == "freeze-replay" {
        let replay = telperion_jev::tuning::replay::freeze(&flag("--job")?, &flag("--out")?)?;
        println!("{} cases frozen", replay.cases.len());
        return Ok(());
    }
    if command == "run" || command == "preflight" {
        let resume = args
            .windows(2)
            .find(|a| a[0] == "--resume")
            .map(|a| PathBuf::from(&a[1]));
        let (config, out) = (flag("--config")?, flag("--out")?);
        if command == "run" {
            return telperion_jev::tuning::command::run(&config, &out, resume.as_deref());
        }
        let plan = telperion_jev::tuning::preflight::plan(&config, &out, resume.as_deref())?;
        let bytes = serde_json::to_vec_pretty(&plan).unwrap();
        fs::create_dir_all(&out).map_err(|e| e.to_string())?;
        fs::write(out.join("plan.json"), &bytes).map_err(|e| e.to_string())?;
        println!("{}", String::from_utf8_lossy(&bytes));
        return Ok(());
    }
    let manifest = fs::read(flag("--manifest")?).map_err(|e| e.to_string())?;
    let out = flag("--out")?;
    if command == "freeze" {
        let mut prepared: calibration::Manifest =
            serde_json::from_slice(&manifest).map_err(|e| e.to_string())?;
        let dials: Vec<telperion_jev::tuning::actions::Dial> =
            serde_json::from_slice(&fs::read(flag("--table")?).map_err(|e| e.to_string())?)
                .map_err(|e| e.to_string())?;
        prepared.table_sha256 = sha256_hex(&serde_json::to_vec(&dials).unwrap());
        for case in &mut prepared.cases {
            case.questions = calibration::questions(&prepared.kind, &case.state)?;
        }
        calibration::validate_manifest(&prepared)?;
        let bytes = serde_json::to_vec_pretty(&prepared).unwrap();
        use std::io::Write;
        fs::OpenOptions::new()
            .write(true)
            .create_new(true)
            .open(&out)
            .map_err(|e| e.to_string())?
            .write_all(&bytes)
            .map_err(|e| e.to_string())?;
        println!("{} {}", sha256_hex(&bytes), out.display());
        return Ok(());
    }
    let max_tokens = args
        .windows(2)
        .find(|a| a[0] == "--max-tokens")
        .ok_or("missing --max-tokens")?[1]
        .parse::<u64>()
        .map_err(|e| e.to_string())?;
    match command.as_str() {
        "calibrate" => {
            let ledger = flag("--ledger")?;
            let key = load_key().map_err(|e| e.to_string())?;
            let result =
                calibration::run(&manifest, &UreqTransport, &key, &ledger, max_tokens, &out)?;
            println!(
                "{}",
                serde_json::to_string_pretty(&calibration::score(&manifest, &result)?).unwrap()
            );
            let m: calibration::Manifest =
                serde_json::from_slice(&manifest).map_err(|e| e.to_string())?;
            if m.kind == "continuation" {
                println!(
                    "{}",
                    serde_json::to_string_pretty(&calibration::policy_score(&m, &result)?).unwrap()
                );
            }
            calibration::qualified(
                &manifest,
                &result,
                &m.kind,
                &m.question_version,
                &m.table_sha256,
            )?;
        }
        "vision-replay" => {
            let declared: serde_json::Value =
                serde_json::from_slice(&manifest).map_err(|e| e.to_string())?;
            if declared["schema"] == "reference-first-replay-v1" {
                let adapter: vision::Adapter = serde_json::from_slice(
                    &fs::read(flag("--adapter")?).map_err(|e| e.to_string())?,
                )
                .map_err(|e| e.to_string())?;
                let score = telperion_jev::tuning::replay::run(
                    &manifest,
                    &adapter,
                    &flag("--protocol")?,
                    &out,
                    &flag("--result")?,
                    max_tokens,
                )?;
                println!("{}", serde_json::to_string_pretty(&score).unwrap());
                return Ok(());
            }
            let replay: vision::Replay =
                serde_json::from_slice(&manifest).map_err(|e| e.to_string())?;
            let adapter: vision::Adapter =
                serde_json::from_slice(&fs::read(flag("--adapter")?).map_err(|e| e.to_string())?)
                    .map_err(|e| e.to_string())?;
            if replay.model != adapter.model || replay.effort != adapter.effort {
                return Err("wrong replay model".into());
            }
            let mut result = vision::ReplayResult {
                manifest_sha256: sha256_hex(&manifest),
                results: vec![],
            };
            let mut journal = calibration::Journal::create(&out, max_tokens)?;
            for case in &replay.cases {
                journal.reserve(25000)?;
                let answer = adapter.assess(&case.request)?;
                let usage = answer
                    .usage
                    .as_ref()
                    .ok_or("unknown vision usage; reserved spend retained")?;
                let tokens = usage
                    .input_tokens
                    .checked_add(usage.output_tokens)
                    .ok_or("usage overflow")?;
                result.results.push(answer);
                journal.record(serde_json::to_value(&result).unwrap())?;
                journal.settle(tokens)?;
            }
            println!(
                "{}",
                serde_json::to_string_pretty(&vision::replay_score(&manifest, &result)?).unwrap()
            );
        }
        _ => return Err(format!("unknown command {command}")),
    }
    Ok(())
}
