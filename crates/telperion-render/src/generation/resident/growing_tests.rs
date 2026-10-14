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
