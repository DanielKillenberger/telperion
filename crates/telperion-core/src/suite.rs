//! Tests of the stages' implementation. They name the crate as its consumers do
//! (`telperion_core::...`), through the stage contracts, which under `cfg(test)`
//! carry the whole stage; they compile only into the crate's own test binary.
//! Helpers the tests share are one module each.
#[path = "suite/catalogue/pins.rs"]
mod catalogue;
#[path = "suite/colonization.rs"]
mod colonization;
#[path = "suite/crown_reference.rs"]
mod crown_reference;
#[path = "suite/crown_retention.rs"]
mod crown_retention;
#[path = "suite/drop.rs"]
mod drop;
#[path = "suite/field.rs"]
mod field;
#[path = "suite/field_fronds.rs"]
mod field_fronds;
#[path = "suite/field_plan.rs"]
mod field_plan;
#[path = "suite/field_still.rs"]
mod field_still;
#[path = "suite/foliage.rs"]
mod foliage;
#[path = "suite/foliage_reference.rs"]
mod foliage_reference;
#[path = "suite/generation_limits.rs"]
mod generation_limits;
#[path = "suite/geometry_benchmark.rs"]
mod geometry_benchmark;
#[path = "suite/growth.rs"]
mod growth;
#[path = "suite/growth_reference.rs"]
mod growth_reference;
#[path = "suite/identity.rs"]
mod identity;
#[path = "suite/leaf_base_lattice.rs"]
mod leaf_base_lattice;
#[path = "suite/leaf_bases.rs"]
mod leaf_bases;
#[path = "suite/limb_clumping.rs"]
mod limb_clumping;
#[path = "suite/mesh.rs"]
mod mesh;
#[path = "suite/outline.rs"]
mod outline;
#[path = "suite/packed_leaf.rs"]
mod packed_leaf;
#[path = "suite/pendulous.rs"]
mod pendulous;
#[path = "suite/rosette.rs"]
mod rosette;
#[path = "suite/sag.rs"]
mod sag;
#[path = "suite/short_shoots.rs"]
mod short_shoots;
#[path = "suite/skirt.rs"]
mod skirt;
#[path = "suite/species.rs"]
mod species;
#[path = "suite/species_metrics.rs"]
mod species_metrics;
#[path = "suite/specimen_cache.rs"]
mod specimen_cache;
#[path = "suite/specimen_handle.rs"]
mod specimen_handle;
#[path = "suite/specimen_view.rs"]
mod specimen_view;
#[path = "suite/specimens/mod.rs"]
mod specimens;
#[path = "suite/stem_fork_height.rs"]
mod stem_fork_height;
#[path = "suite/stem_lean_spread.rs"]
mod stem_lean_spread;
#[path = "suite/stems.rs"]
mod stems;
#[path = "suite/strands.rs"]
mod strands;
#[path = "suite/surface.rs"]
mod surface;
#[path = "suite/surface_attachments.rs"]
mod surface_attachments;
#[path = "suite/surface_collapse.rs"]
mod surface_collapse;
#[path = "suite/surface_prepared.rs"]
mod surface_prepared;
#[path = "suite/surface_reference.rs"]
mod surface_reference;
#[path = "suite/sweep.rs"]
mod sweep;
#[path = "suite/terminal_taper.rs"]
mod terminal_taper;
#[path = "suite/twig_generations.rs"]
mod twig_generations;
