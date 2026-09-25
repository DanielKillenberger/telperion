//! The pipeline's build identity (fn-131): a digest of the crate's code and
//! data, baked in by the build script. Every stage's key carries it, so a
//! changed stage reruns instead of reading as current; the crate version
//! alone left three stages current after fn-128's fix.

include!("tree_digest.rs");

/// `tree_digest` of this crate as it was compiled.
pub const BUILD_ID: &str = env!("TELPERION_JEV_BUILD");
/// The tool name a stage header records the build under.
pub const BUILD_TOOL: &str = "species-pipeline-build";
