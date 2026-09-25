//! The render tools a run measures and draws with, built from the checkout
//! the runner runs in, so a revision never draws with a binary from an older
//! commit. Cargo leaves a current binary alone; the runner builds every time.
use std::path::{Path, PathBuf};
use std::process::Command;

pub struct Tools {
    pub species_measure: PathBuf,
    pub geometry_benchmark: PathBuf,
    pub headless: PathBuf,
}

impl Tools {
    /// The release examples under `<root>/target`.
    pub fn at(root: &Path) -> Self {
        let example = |name: &str| root.join("target/release/examples").join(name);
        Self {
            species_measure: example("species_measure"),
            geometry_benchmark: example("geometry_benchmark"),
            headless: example("headless"),
        }
    }

    /// Every tool's bytes key the stages that draw or measure with it.
    pub fn files(&self) -> Vec<PathBuf> {
        vec![
            self.species_measure.clone(),
            self.geometry_benchmark.clone(),
            self.headless.clone(),
        ]
    }

    /// Builds the three examples in release.
    pub fn build(root: &Path) -> Result<Self, String> {
        for (package, examples) in [
            (
                "telperion-core",
                &["species_measure", "geometry_benchmark"][..],
            ),
            ("telperion-render", &["headless"][..]),
        ] {
            let mut command = Command::new("cargo");
            command
                .current_dir(root)
                .args(["build", "--release", "-p", package]);
            for example in examples {
                command.args(["--example", example]);
            }
            let status = command.status().map_err(|e| format!("cargo: {e}"))?;
            if !status.success() {
                return Err(format!("cargo build {package} examples exited {status}"));
            }
        }
        Ok(Self::at(root))
    }
}
