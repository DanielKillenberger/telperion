//! What a segment cost (fn-272 R2): its frames, the time building or
//! growing and settling its trees, the time drawing and writing them, and
//! the peak memory: the process's resident peak (reset at the segment's
//! start) and the device memory `nvidia-smi` sees this process hold,
//! sampled four times a second where the tool is present.
use std::sync::atomic::{AtomicBool, AtomicU64, Ordering};
use std::sync::Arc;

#[derive(Default)]
pub struct Cost {
    pub frames: usize,
    pub trees: usize,
    pub build_ms: f64,
    pub draw_ms: f64,
}

pub struct Peaks {
    stop: Arc<AtomicBool>,
    device: Arc<AtomicU64>,
    sampler: Option<std::thread::JoinHandle<()>>,
}

impl Peaks {
    pub fn start() -> Peaks {
        let (stop, device) = (
            Arc::new(AtomicBool::new(false)),
            Arc::new(AtomicU64::new(0)),
        );
        let (s, d) = (stop.clone(), device.clone());
        let pid = std::process::id().to_string();
        let sampler = std::thread::spawn(move || {
            while !s.load(Ordering::Relaxed) {
                let query = [
                    "--query-compute-apps=pid,used_memory",
                    "--format=csv,noheader,nounits",
                ];
                let Ok(out) = std::process::Command::new("nvidia-smi")
                    .args(query)
                    .output()
                else {
                    return;
                };
                for line in String::from_utf8_lossy(&out.stdout).lines() {
                    if let Some((p, mb)) = line.split_once(',') {
                        if p.trim() == pid {
                            d.fetch_max(mb.trim().parse().unwrap_or(0), Ordering::Relaxed);
                        }
                    }
                }
                std::thread::sleep(std::time::Duration::from_millis(250));
            }
        });
        Peaks {
            stop,
            device,
            sampler: Some(sampler),
        }
    }

    /// Both peaks from here on.
    pub fn reset(&self) {
        self.device.store(0, Ordering::Relaxed);
        // Linux resets the resident peak on "5" (proc(5), clear_refs).
        let _ = std::fs::write("/proc/self/clear_refs", "5");
    }

    /// The resident and device peaks since the reset, MB.
    pub fn take(&self) -> (u64, u64) {
        let status = std::fs::read_to_string("/proc/self/status").unwrap_or_default();
        let kb = status
            .lines()
            .find_map(|l| l.strip_prefix("VmHWM:"))
            .and_then(|v| v.trim().trim_end_matches(" kB").trim().parse::<u64>().ok());
        (kb.unwrap_or(0) / 1024, self.device.load(Ordering::Relaxed))
    }
}

impl Drop for Peaks {
    fn drop(&mut self) {
        self.stop.store(true, Ordering::Relaxed);
        if let Some(s) = self.sampler.take() {
            let _ = s.join();
        }
    }
}
