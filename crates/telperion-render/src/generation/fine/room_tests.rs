//! fn-235 host decision, third: the budget counts the drawn tree, so a
//! build that fits only alone lets the drawn tree go first, and one that
//! does not fit even alone is refused by name with the drawn tree kept.
use super::super::{Admitted, Delivery, Generator};
use crate::Renderer;

fn family(id: &str, age: u32) -> telperion_core::presets::Family {
    let mut family = telperion_core::presets::by_identity(id).unwrap();
    family.age = f64::from(age);
    family
}

/// The predicted peak of `family`'s build, in bytes.
fn peak(family: &telperion_core::presets::Family) -> u64 {
    let species = family.space.species.as_ref().unwrap();
    let kept = telperion_space::expected_kept(species, family.age as u32);
    super::budget::peak(kept.unwrap().unwrap())
}

/// A renderer with the beech at 12 drawn, its generator, and the bytes
/// held with the tree's share of them.
fn drawn() -> Option<(Renderer, Generator, u64, u64)> {
    let gpu = crate::generation::test_gpu()?;
    let mut renderer = Renderer::new(gpu, crate::STILL_FORMAT);
    let gen = Generator::new(&renderer).unwrap();
    let prepared = gen.prepare(&family("european-beech", 12), Delivery::Resident);
    renderer.submit_prepared(prepared.unwrap()).unwrap();
    let held = renderer.gpu().ledger.live();
    let tree = Generator::tree_buffer_bytes(&renderer);
    Some((renderer, gen, held, tree))
}

/// A still at one pose, the beech's room whether a tree is drawn or not.
fn still(renderer: &mut Renderer) -> crate::Still {
    use telperion_core::{math::Vec3, surface::Bounds};
    let room = Bounds {
        min: Vec3::new(-5.0, 0.0, -5.0),
        max: Vec3::new(5.0, 12.0, 5.0),
    };
    let camera = crate::hero_pose(room, 4.0 / 3.0, crate::GROUND_REACH);
    crate::render(renderer, &camera, 320, 240).unwrap()
}

#[test]
fn a_tree_that_fits_only_alone_lets_the_drawn_tree_go_first() {
    let Some((mut renderer, gen, held, tree)) = drawn() else {
        return;
    };
    let next = family("european-beech", 30);
    // Room for the next tree once the drawn one is let go, and half the
    // drawn tree's bytes to spare, not its whole.
    let budget = held - tree / 2 + peak(&next);
    assert!(tree > 0 && held + peak(&next) > budget, "{tree} of {held}");
    let drawn = still(&mut renderer).stats.instances;
    let admitted = Generator::admit(&next, &renderer, budget).unwrap();
    assert_eq!(admitted, Admitted::MakeRoom);
    renderer.release_tree();
    assert!(
        renderer.gpu().ledger.live() <= held - tree,
        "the tree's buffers let go"
    );
    // The screen is empty of the tree while the next builds.
    assert!(drawn > 0 && renderer.bounds().is_none());
    assert_eq!(still(&mut renderer).stats.instances, 0);
    gen.budget.set(Some(budget));
    let built = gen.prepare(&next, Delivery::Resident).unwrap();
    renderer.submit_prepared(built).unwrap();
    assert!(still(&mut renderer).stats.instances > 0);
}

#[test]
fn a_tree_that_does_not_fit_even_alone_is_refused_and_the_drawn_tree_kept() {
    let Some((mut renderer, _gen, held, tree)) = drawn() else {
        return;
    };
    let next = family("european-beech", 30);
    let before = still(&mut renderer);
    let budget = held - tree + peak(&next) - 1;
    let refused = Generator::admit(&next, &renderer, budget).unwrap_err();
    assert!(
        refused
            .to_string()
            .contains("even with the drawn tree let go"),
        "{refused}"
    );
    let after = still(&mut renderer);
    assert!(after.has_subject());
    assert_eq!(after.stats.instances, before.stats.instances);
}
