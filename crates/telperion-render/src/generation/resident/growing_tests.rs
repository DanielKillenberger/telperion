//! fn-270 R6, R7: a tree grown in through the generator's growth entry,
//! as the page grows it (a year at a time, drawn within each year and put
//! on the canvas), ends as the tree built at once, leaf for leaf; a
//! request the tree cannot meet is refused by name.
use super::super::{Delivery, Generator};
use crate::Renderer;
use pollster::block_on;

#[test]
fn a_tree_grown_in_year_by_year_is_the_tree_built_at_once() {
    let Some(gpu) = crate::generation::test_gpu() else {
        return;
    };
    let mut renderer = Renderer::new(gpu, crate::STILL_FORMAT);
    let gen = Generator::new(&renderer).unwrap();
    block_on(gen.prepare_programs()).unwrap();
    // The oak on twig stations, the palm on its rosette's year (R5b).
    for (id, age) in [("oregon-white-oak", 8.0), ("date-palm", 6.0)] {
        let mut family = telperion_core::presets::by_identity(id).unwrap();
        family.age = age;
        let once = gen.prepare(&family, Delivery::Resident).unwrap();
        let (want, bounds) = (gen.read_instances(&once).unwrap().leaves, once.bounds());
        let mut tree = block_on(gen.begin_growth(&family)).unwrap();
        assert_eq!((tree.committed(), tree.horizon()), (Some(0), age as u32));
        for year in 1..=age as u32 {
            // The seedling stands only from its first budburst, near the
            // end of its first year: before it there is no wood to draw.
            let at = f64::from(year) - if year == 1 { 0.0 } else { 0.6 };
            block_on(gen.grow_to(&mut tree, at)).unwrap();
            assert_eq!(tree.committed(), Some(year), "{id}: grown to {at}");
            let drawn = block_on(gen.settle(&tree, at)).unwrap();
            assert!((0.0..=1.0).contains(&drawn.colouring), "{id} at {at}: colouring {}", drawn.colouring);
            renderer.submit_prepared(drawn).unwrap();
        }
        let whole = block_on(gen.settle(&tree, age)).unwrap();
        assert_eq!(whole.colouring, 1.0);
        assert_eq!(whole.bounds(), bounds, "{id}: bounds");
        assert!(gen.read_instances(&whole).unwrap().leaves == want, "{id}: the leaves grown in are not the leaves built at once");
        for (refused, name) in [
            (block_on(gen.grow_to(&mut tree, age + 1.0)).err(), "age.horizon"),
            (block_on(gen.grow_to(&mut tree, f64::NAN)).err(), "age"),
            (block_on(gen.settle(&tree, age - 1.5)).err(), "at"),
        ] {
            let refused = refused.expect("refused").to_string();
            assert!(refused.contains(name), "{id}: {refused}");
        }
        assert_eq!(tree.committed(), Some(age as u32), "a refusal moved nothing");
    }
}

/// R2b: a draw whose placement fails after the device drew it spends the
/// tree, and every later call refuses it by name.
#[test]
fn a_draw_that_fails_to_place_spends_the_tree() {
    let Some(gpu) = crate::generation::test_gpu() else {
        return;
    };
    let renderer = Renderer::new(gpu, crate::STILL_FORMAT);
    let gen = Generator::new(&renderer).unwrap();
    let mut family = telperion_core::presets::by_identity("oregon-white-oak").unwrap();
    (family.age, family.canopy.max_instances) = (6.0, 1);
    let mut tree = block_on(gen.begin_growth(&family)).unwrap();
    block_on(gen.grow_to(&mut tree, 6.0)).unwrap();
    let refused = block_on(gen.settle(&tree, 6.0)).err().expect("over the instance budget");
    assert!(refused.to_string().contains("instance"), "{refused}");
    assert_eq!(tree.committed(), None);
    let again = block_on(gen.grow_to(&mut tree, 6.0)).err().expect("spent").to_string();
    assert!(again.contains("state.failed"), "{again}");
}

/// Host ruling (2026-10-14, the owner's spruce at 80): a grow-in's draw
/// keeps the budget's one rule. Beside the drawn tree where it fits; where
/// it fits only alone, the drawn tree is let go first and the draw is up;
/// where it fits not even alone, refused by name with the drawn tree kept.
#[test]
fn a_grow_in_draw_lets_the_drawn_tree_go_only_where_it_fits_alone() {
    use super::super::Admitted;
    let Some(gpu) = crate::generation::test_gpu() else {
        return;
    };
    let mut renderer = Renderer::new(gpu, crate::STILL_FORMAT);
    let gen = Generator::new(&renderer).unwrap();
    block_on(gen.prepare_programs()).unwrap();
    let mut family = telperion_core::presets::by_identity("oregon-white-oak").unwrap();
    family.age = 8.0;
    let mut tree = block_on(gen.begin_growth(&family)).unwrap();
    for year in 1..=7 {
        block_on(gen.grow_to(&mut tree, f64::from(year))).unwrap();
        let admitted = Generator::draw_admission(&tree, &renderer, u64::MAX).unwrap();
        assert_eq!(admitted, Admitted::Beside, "year {year}");
        renderer.submit_prepared(block_on(gen.settle(&tree, f64::from(year))).unwrap()).unwrap();
    }
    block_on(gen.grow_to(&mut tree, 8.0)).unwrap();
    block_on(crate::generation::io::complete(&gen.gpu)).unwrap();
    let (held, drawn, next) = (gen.gpu.ledger.live(), Generator::tree_buffer_bytes(&renderer), tree.expected_draw());
    assert!(drawn > 0 && next > 0, "drawn {drawn}, expected {next}");
    let budget = |b: u64| Generator::draw_admission(&tree, &renderer, b);
    assert_eq!(budget(held + next).unwrap(), Admitted::Beside);
    assert_eq!(budget(held + next - 1).unwrap(), Admitted::MakeRoom);
    let refused = budget(held - drawn + next - 1).unwrap_err().to_string();
    assert!(refused.contains("even with the drawn tree let go"), "{refused}");
    assert_eq!(Generator::tree_buffer_bytes(&renderer), drawn, "a refusal keeps the drawn tree");
    // A draw refused by name is refused before anything is let go.
    for at in [f64::NAN, 6.5] {
        assert!(gen.drawable(&tree, at).is_err(), "at {at}");
    }
    gen.drawable(&tree, 8.0).unwrap();
    // Made room: the drawn tree let go, the draw put up alone, and its
    // bytes measured without the tree let go (as the page lets it go,
    // with no wait between).
    let before = tree.last_draw();
    renderer.release_tree();
    let up = renderer.submit_prepared(block_on(gen.settle(&tree, 8.0)).unwrap());
    assert!(up.is_ok() && Generator::tree_buffer_bytes(&renderer) > 0);
    assert!(tree.last_draw() >= before, "the draw at 8 measured {} after {before} at 7", tree.last_draw());
}
