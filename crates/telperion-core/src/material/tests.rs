//! Every material row is refused off its range by its own name.
use super::*;

#[test]
fn the_row_every_family_starts_from_is_one_a_leaf_can_be_drawn_from() {
    assert_eq!(MaterialParams::default().validate(), Ok(()));
}

#[test]
fn a_value_off_its_range_is_refused_by_its_own_name() {
    // One field at a time, so the name in the refusal is the only name it
    // could have come from. Every field is covered: a row that grew a
    // field without a bound would fail the count below.
    // One field put off its range, and the name the refusal must carry.
    type Refusal = (fn(&mut MaterialParams), &'static str);
    let refusals: [Refusal; 81] = [
        (|m| m.lobe_shade = -0.1, "leaf lobe shade"),
        (|m| m.bark_reflectance = 1.1, "bark reflectance"),
        (|m| m.leaf_reflectance = -0.1, "leaf reflectance"),
        (|m| m.bark_grain_scale = 0.06, "bark grain scale"),
        (|m| m.bark_grain_strength = 1.5, "bark grain strength"),
        (|m| m.blade_grain_scale = 257.0, "leaf blade grain scale"),
        (
            |m| m.blade_grain_strength = f64::NAN,
            "leaf blade grain strength",
        ),
        (|m| m.lichen_scale = 1.5, "bark lichen scale"),
        (|m| m.lichen_coverage = -0.1, "bark lichen coverage"),
        (|m| m.lichen_red = 1.1, "bark lichen red"),
        (|m| m.lichen_green = -0.1, "bark lichen green"),
        (|m| m.lichen_blue = f64::NAN, "bark lichen blue"),
        (|m| m.lichen_strength = 2.0, "bark lichen strength"),
        (|m| m.lenticel_density = 401.0, "bark lenticel density"),
        (|m| m.lenticel_length = 0.6, "bark lenticel length"),
        (|m| m.lenticel_strength = -0.5, "bark lenticel strength"),
        (|m| m.lenticel_tint = -1.5, "bark lenticel tint"),
        (|m| m.peel_curl = 1.5, "bark peel curl"),
        (|m| m.peel_red = 2.0, "bark peel red"),
        (|m| m.peel_green = -1.0, "bark peel green"),
        (|m| m.peel_blue = 1.2, "bark peel blue"),
        (|m| m.plate_cell_scale = 2.0, "bark plate cell scale"),
        (|m| m.plate_furrow_width = 1.5, "bark plate furrow width"),
        (|m| m.plate_edge_shape = 1.5, "bark plate edge shape"),
        (|m| m.plate_elongation = 17.0, "bark plate elongation"),
        (|m| m.plate_dome = 2.0, "bark plate dome"),
        (|m| m.plate_edge_lift = 2.0, "bark plate edge lift"),
        (|m| m.plate_identity = 2.0, "bark plate identity"),
        (|m| m.weathering_strength = 2.0, "bark weathering strength"),
        (|m| m.weathering_red = 2.0, "bark weathering red"),
        (|m| m.weathering_green = -2.0, "bark weathering green"),
        (|m| m.weathering_blue = 2.0, "bark weathering blue"),
        (
            |m| m.orientation_strength = 2.0,
            "bark orientation strength",
        ),
        (|m| m.orientation_red = 2.0, "bark orientation red"),
        (|m| m.orientation_green = 2.0, "bark orientation green"),
        (|m| m.orientation_blue = -2.0, "bark orientation blue"),
        (
            |m| m.directional_occlusion = 2.0,
            "bark directional occlusion",
        ),
        (|m| m.depth_strength = 2.0, "bark depth strength"),
        (|m| m.fissure_red = 2.0, "bark fissure red"),
        (|m| m.fissure_green = 2.0, "bark fissure green"),
        (|m| m.fissure_blue = 2.0, "bark fissure blue"),
        (|m| m.fissure_strength = 2.0, "bark fissure strength"),
        (|m| m.crest_red = 2.0, "bark crest red"),
        (|m| m.crest_green = 2.0, "bark crest green"),
        (|m| m.crest_blue = 2.0, "bark crest blue"),
        (|m| m.crest_strength = 2.0, "bark crest strength"),
        (|m| m.bark_mottle_scale = 9.0, "bark mottle scale"),
        (|m| m.bark_mottle_strength = 2.0, "bark mottle strength"),
        (|m| m.cavity_strength = 2.0, "bark cavity strength"),
        (|m| m.blade_mottle_scale = 33.0, "leaf blade mottle scale"),
        (
            |m| m.blade_mottle_strength = 2.0,
            "leaf blade mottle strength",
        ),
        (|m| m.margin_width = 1.5, "leaf margin width"),
        (|m| m.margin_red = 2.0, "leaf margin red"),
        (|m| m.margin_green = 2.0, "leaf margin green"),
        (|m| m.margin_blue = 2.0, "leaf margin blue"),
        (|m| m.cuticle_gloss = 2.0, "leaf cuticle gloss"),
        (|m| m.sky_occlusion_strength = 2.0, "sky occlusion strength"),
        (|m| m.bark_red = 1.5, "bark red"),
        (|m| m.bark_green = -0.1, "bark green"),
        (|m| m.bark_blue = f64::NAN, "bark blue"),
        (|m| m.bark_roughness = 2.0, "bark roughness"),
        (|m| m.shoot_red = -0.1, "young shoot red"),
        (|m| m.shoot_green = 1.1, "young shoot green"),
        (|m| m.shoot_blue = f64::NAN, "young shoot blue"),
        (|m| m.shoot_radius = 0.101, "young shoot radius"),
        (|m| m.leaf_front_red = -1.0, "leaf front red"),
        (|m| m.leaf_front_green = 1.2, "leaf front green"),
        (|m| m.leaf_front_blue = f64::INFINITY, "leaf front blue"),
        (|m| m.leaf_back_red = 3.0, "leaf back red"),
        (|m| m.leaf_back_green = -0.2, "leaf back green"),
        (|m| m.leaf_back_blue = 1.000_1, "leaf back blue"),
        (|m| m.hue_range_low = -0.7, "leaf hue range low"),
        (|m| m.hue_range_high = 0.7, "leaf hue range high"),
        (
            |m| m.brightness_range_low = -1.5,
            "leaf brightness range low",
        ),
        (
            |m| m.brightness_range_high = 1.5,
            "leaf brightness range high",
        ),
        (|m| m.interior_darkening = 1.1, "leaf interior darkening"),
        (|m| m.canopy_normal = 1.1, "leaf canopy normal"),
        (|m| m.light_wrap = -0.1, "leaf light wrap"),
        (
            |m| m.diffuse_transmission = f64::NAN,
            "leaf diffuse transmission",
        ),
        (|m| m.leaf_sheen = 0.51, "leaf sheen"),
        (|m| m.crown_shade = 1.5, "leaf crown shade"),
    ];
    for (break_it, name) in refusals {
        let mut row = MaterialParams::default();
        break_it(&mut row);
        assert_eq!(row.validate(), Err(Error::InvalidInput(name)));
    }
}

#[test]
fn a_range_that_runs_backwards_is_refused_by_the_pairs_name() {
    let mut hue = MaterialParams::default();
    (hue.hue_range_low, hue.hue_range_high) = (0.2, -0.2);
    assert_eq!(hue.validate(), Err(Error::InvalidInput("leaf hue range")));
    let mut brightness = MaterialParams::default();
    (
        brightness.brightness_range_low,
        brightness.brightness_range_high,
    ) = (0.3, 0.1);
    assert_eq!(
        brightness.validate(),
        Err(Error::InvalidInput("leaf brightness range"))
    );
    // A range of no width is a leaf that varies not at all, which is a
    // family with one leaf colour and not an error.
    let mut flat = MaterialParams::default();
    (flat.hue_range_low, flat.hue_range_high) = (0.0, 0.0);
    assert_eq!(flat.validate(), Ok(()));
}
