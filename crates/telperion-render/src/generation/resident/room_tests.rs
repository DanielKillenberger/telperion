//! fn-235 host decision, third: the budget counts the drawn tree, so a
//! build that fits only alone lets the drawn tree go first, and one that
//! does not fit even alone is refused by name with the drawn tree kept.
use super::super::{Admitted, Generator};
use crate::Renderer;

fn family(id: &str, age: u32) -> telperion_core::presets::Family {
    let mut family = telperion_core::presets::by_identity(id).unwrap();
    family.age = f64::from(age);
    unlit(family)
}

/// `family` as it grew unlit, before fn-264: the predicted admission this
/// tests serves only a species that reads no light (a lit one is held to
/// the budget as it grows, tested beside it).
fn unlit(mut family: telperion_core::presets::Family) -> telperion_core::presets::Family {
    for s in &mut family.space.species.as_mut().unwrap().states {
        (s.shade_hazard, s.shade_size, s.balance_hazard, s.leaf_girth) = (0.0, 0.0, 0.0, 0.0);
    }
    family
}

/// The predicted peak of `family`'s build, in bytes.
fn peak(family: &telperion_core::presets::Family) -> u64 {
    let species = family.space.species.as_ref().unwrap();
    let kept = telperion_space::expected_kept(species, family.age as u32);
    super::budget::peak(kept.unwrap().unwrap())
}

/// A renderer with `id` at 12 drawn, its generator, and the bytes the
/// device holds, with the tree's share of them.
fn drawn(id: &str) -> Option<(Renderer, Generator, u64, u64)> {
    let gpu = crate::generation::test_gpu()?;
    let mut renderer = Renderer::new(gpu, crate::STILL_FORMAT);
    let gen = Generator::new(&renderer).unwrap();
    let prepared = gen.prepare(&family(id, 12));
    renderer.submit_prepared(prepared.unwrap()).unwrap();
    let held = gen.gpu.ledger.live();
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
    let Some((mut renderer, gen, held, tree)) = drawn("european-beech") else {
        return;
    };
    let next = family("european-beech", 30);
    // Room for the next tree once the drawn one is let go, and half the
    // drawn tree's bytes to spare, not its whole.
    let budget = held - tree / 2 + peak(&next);
    assert!(tree > 0 && held + peak(&next) > budget, "{tree} of {held}");
    let drawn = still(&mut renderer).stats.instances;
    // The still's buffers, let go, are charged until the device is done.
    pollster::block_on(crate::generation::io::complete(&gen.gpu)).unwrap();
    let admitted = Generator::admit(&next, &renderer, budget).unwrap();
    assert_eq!(admitted, Admitted::MakeRoom);
    renderer.release_tree();
    // A buffer let go is charged until the device is done with it.
    pollster::block_on(crate::generation::io::complete(&gen.gpu)).unwrap();
    assert!(
        gen.gpu.ledger.live() <= held - tree,
        "the tree's buffers let go"
    );
    // The screen is empty of the tree while the next builds.
    assert!(drawn > 0 && renderer.bounds().is_none());
    assert_eq!(still(&mut renderer).stats.instances, 0);
    gen.budget.set(Some(budget));
    let built = gen.prepare(&next).unwrap();
    renderer.submit_prepared(built).unwrap();
    assert!(still(&mut renderer).stats.instances > 0);
}

#[test]
fn a_tree_that_does_not_fit_even_alone_is_refused_and_the_drawn_tree_kept() {
    let Some((mut renderer, _gen, held, tree)) = drawn("european-beech") else {
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

/// The owner's walk the reviewers named (fn-235 review): a spruce at 12
/// drawn, then the spruce at 80, under the page's own 6 GiB: it builds,
/// beside or alone.
#[test]
fn the_spruce_at_80_follows_the_spruce_at_12_under_the_page_budget() {
    let Some((renderer, _gen, held, tree)) = drawn("norway-spruce") else {
        return;
    };
    let next = family("norway-spruce", 80);
    let admitted = Generator::admit(&next, &renderer, super::budget::BROWSER);
    let said = format!("held {} MiB, tree {} MiB", held >> 20, tree >> 20);
    assert!(admitted.is_ok(), "{said}: {admitted:?}");
    println!("{said}: {:?}", admitted.unwrap());
}

/// fn-258 R2: a build's growth scratch and geometry scratch are let go
/// when it is done. Once the drawn tree is released too, the device holds
/// what it held before the build.
#[test]
fn a_build_keeps_nothing_on_the_device_once_it_is_done() {
    let Some(gpu) = crate::generation::test_gpu() else {
        return;
    };
    let mut renderer = Renderer::new(gpu, crate::STILL_FORMAT);
    let gen = Generator::new(&renderer).unwrap();
    pollster::block_on(gen.prepare_programs()).unwrap();
    let before = gen.gpu.ledger.live();
    let prepared = gen.prepare(&family("norway-spruce", 12));
    renderer.submit_prepared(prepared.unwrap()).unwrap();
    renderer.release_tree();
    // A buffer let go is charged until the device is done with it.
    pollster::block_on(crate::generation::io::complete(&gen.gpu)).unwrap();
    let after = gen.gpu.ledger.live();
    assert!(
        after <= before,
        "the build kept {} KiB on the device",
        (after - before) >> 10
    );
}

/// fn-294 R2: a flowering tree's memory counts its flowers. The bytes
/// reported are the bytes releasing the tree hands back, flowers among
/// them, and a replacement that fits once they are let go is admitted.
#[test]
fn a_flowering_tree_s_memory_counts_its_flowers() {
    let Some((mut renderer, gen, _, tree)) = drawn("laurelin") else {
        return;
    };
    let organ = renderer.organ.allocated_bytes();
    assert!(organ > 0, "the laurelin at 12 flowers");
    let next = family("laurelin", 12);
    pollster::block_on(crate::generation::io::complete(&gen.gpu)).unwrap();
    let held = gen.gpu.ledger.live();
    // Room for the next tree once all three layers are let go, no more.
    let layers = renderer.wood.allocated_bytes() + renderer.foliage.allocated_bytes() + organ;
    let budget = held - layers + peak(&next);
    let admitted = Generator::admit(&next, &renderer, budget).unwrap();
    assert_eq!(admitted, Admitted::MakeRoom);
    renderer.release_tree();
    pollster::block_on(crate::generation::io::complete(&gen.gpu)).unwrap();
    // Releasing hands back the three layers and a few bytes besides (the
    // existing room tests hold the same `<=`): never less than reported, and
    // never a whole layer of flowers more.
    let freed = held - gen.gpu.ledger.live();
    assert!(
        freed >= tree && freed - tree < organ,
        "{freed} freed, {tree} reported"
    );
}

/// fn-297 R5: the ledger is what is held. Laurelin replaced by a tree
/// without flowers lets her flowers go, and releasing that tree hands back
/// exactly the bytes it reports, the curve's two demand readbacks among
/// them.
#[test]
fn a_tree_without_flowers_lets_the_last_one_s_flowers_go() {
    let Some((mut renderer, gen, _, _)) = drawn("laurelin") else {
        return;
    };
    assert!(renderer.organ.allocated_bytes() > 0, "the laurelin at 12 flowers");
    let oak = gen.prepare(&family("oregon-white-oak", 12)).unwrap();
    renderer.submit_prepared(oak).unwrap();
    assert_eq!(renderer.organ.allocated_bytes(), 0, "the oak holds no flowers");
    let tree = Generator::tree_buffer_bytes(&renderer);
    pollster::block_on(crate::generation::io::complete(&gen.gpu)).unwrap();
    let held = gen.gpu.ledger.live();
    renderer.release_tree();
    pollster::block_on(crate::generation::io::complete(&gen.gpu)).unwrap();
    let freed = held - gen.gpu.ledger.live();
    assert_eq!(freed, tree, "{freed} freed, {tree} reported");
}
