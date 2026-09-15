//! Material value tables kept beside the geometry presets.
use crate::material::MaterialParams;

pub(super) fn oak() -> MaterialParams {
    MaterialParams {
        bark_red: 0.225,
        bark_green: 0.218,
        bark_blue: 0.198,
        bark_roughness: 0.85,
        // Neutral: no young wood until the oak's own table states it.
        shoot_red: 0.225,
        shoot_green: 0.218,
        shoot_blue: 0.198,
        shoot_radius: 0.0,
        leaf_front_red: 0.028,
        leaf_front_green: 0.102,
        leaf_front_blue: 0.016,
        leaf_back_red: 0.153,
        leaf_back_green: 0.254,
        leaf_back_blue: 0.112,
        hue_range_low: -0.03,
        hue_range_high: 0.03,
        brightness_range_low: -0.15,
        brightness_range_high: 0.15,
        interior_darkening: 0.55,
        ridge_scale: 0.032,
        plate_scale: 0.055,
        furrow_strength: 1.0,
        roughness_detail: 0.12,
        vein_scale: 7.0,
        vein_contrast: 0.45,
        transmission_strength: 0.55,
        transmission_red: 0.24,
        transmission_green: 0.52,
        transmission_blue: 0.07,
        thickness: 0.65,
        fissure_red: -0.1,
        fissure_green: -0.05,
        fissure_blue: 0.005,
        fissure_strength: 0.65,
        crest_red: 0.1,
        crest_green: 0.085,
        crest_blue: 0.055,
        crest_strength: 0.5,
        bark_mottle_scale: 0.8,
        bark_mottle_strength: 0.2,
        cavity_strength: 0.6,
        blade_mottle_scale: 6.0,
        blade_mottle_strength: 0.15,
        margin_width: 0.08,
        margin_red: 0.025,
        margin_green: 0.035,
        margin_blue: 0.006,
        cuticle_gloss: 0.35,
        sky_occlusion_strength: 0.5,
        plate_cell_scale: 0.084,
        plate_elongation: 1.8,
        plate_dome: 0.41875,
        plate_edge_lift: 0.27,
        plate_furrow_width: 0.0,
        plate_identity: 0.5,
        weathering_strength: 0.425,
        weathering_red: 0.0225,
        weathering_green: 0.035,
        weathering_blue: 0.00575,
        orientation_strength: 0.40625,
        orientation_red: -0.062,
        orientation_green: 0.009,
        orientation_blue: -0.1535,
        directional_occlusion: 1.0,
        depth_strength: 0.5375,
        // Plated bark: no lichen, lenticel or peel term of fn-40's.
        ..MaterialParams::default()
    }
}

pub(super) fn spruce() -> MaterialParams {
    MaterialParams {
        bark_red: 0.147,
        bark_green: 0.078,
        bark_blue: 0.045,
        bark_roughness: 0.9,
        // Neutral: no young wood until the spruce's own table states it.
        shoot_red: 0.147,
        shoot_green: 0.078,
        shoot_blue: 0.045,
        shoot_radius: 0.0,
        leaf_front_red: 0.018,
        leaf_front_green: 0.056,
        leaf_front_blue: 0.028,
        leaf_back_red: 0.109,
        leaf_back_green: 0.195,
        leaf_back_blue: 0.138,
        hue_range_low: -0.02,
        hue_range_high: 0.02,
        brightness_range_low: -0.10,
        brightness_range_high: 0.10,
        interior_darkening: 0.7,
        ridge_scale: 0.02,
        plate_scale: 0.03,
        furrow_strength: 0.025,
        roughness_detail: 0.16,
        vein_scale: 8.0,
        vein_contrast: 0.0,
        transmission_strength: 0.01,
        transmission_red: 0.12,
        transmission_green: 0.24,
        transmission_blue: 0.08,
        thickness: 3.5,
        fissure_red: -0.035,
        fissure_green: -0.03,
        fissure_blue: -0.02,
        fissure_strength: 0.4,
        crest_red: 0.05,
        crest_green: 0.025,
        crest_blue: 0.01,
        crest_strength: 0.3,
        bark_mottle_scale: 0.4,
        bark_mottle_strength: 0.2,
        cavity_strength: 0.6,
        blade_mottle_scale: 8.0,
        blade_mottle_strength: 0.0,
        margin_width: 0.0,
        margin_red: 0.01,
        margin_green: 0.015,
        margin_blue: 0.002,
        cuticle_gloss: 0.05,
        sky_occlusion_strength: 0.6,
        plate_cell_scale: 0.028,
        plate_elongation: 0.2,
        plate_dome: 0.39375,
        plate_edge_lift: 0.5,
        plate_furrow_width: 0.0,
        plate_identity: 0.375,
        weathering_strength: 0.6,
        weathering_red: 0.015,
        weathering_green: 0.03,
        weathering_blue: 0.02975,
        orientation_strength: 0.44375,
        orientation_red: -0.014,
        orientation_green: 0.026,
        orientation_blue: -0.0155,
        directional_occlusion: 0.30625,
        depth_strength: 0.45625,
        // Plated bark: no lichen, lenticel or peel term of fn-40's.
        ..MaterialParams::default()
    }
}

pub(super) fn beech() -> MaterialParams {
    MaterialParams {
        // fn-40: B-BASE's smooth grey reads 132/133/138 at the centre of its
        // close-up; the old 0.36/0.335/0.295 drew it at 96/101/106 under the
        // shot's sun and sky, and this row draws it at 127/133/138.
        bark_red: 0.54,
        bark_green: 0.5,
        bark_blue: 0.44,
        bark_roughness: 0.4,
        // Young shoots light olive- to grey-brown (VT Dendrology, Fagus
        // sylvatica: "slender, zigzag, light brown"; OSU: stems olive-brown),
        // grey by the second or third year. B-BARE's winter crown haze reads
        // linear 0.064/0.055/0.033 to 0.149/0.136/0.097 on its own exposure,
        // R:G:B 1:0.86:0.52 in its darker band. The birch's matched still
        // drew a row redder by 0.85 in green and 0.78 in blue, so the row is
        // set greener than the band; unrendered until the beech's table lands.
        // Smooth grey from about 1.6 cm across.
        shoot_red: 0.14,
        shoot_green: 0.13,
        shoot_blue: 0.085,
        shoot_radius: 0.004,
        leaf_front_red: 0.022,
        leaf_front_green: 0.105,
        leaf_front_blue: 0.018,
        leaf_back_red: 0.14,
        leaf_back_green: 0.23,
        leaf_back_blue: 0.1,
        hue_range_low: -0.02,
        hue_range_high: 0.02,
        brightness_range_low: -0.1,
        brightness_range_high: 0.1,
        interior_darkening: 0.35,
        // Smooth bark: no ridge field at all. The close-up drew it as vertical
        // wavy ridges with bright rims at every setting the field has, down to
        // a crackle at 2 mm; the beech's fissure, crest and plate rows draw
        // nothing without it, and what the eye reads is the colour below.
        ridge_scale: 0.0,
        plate_scale: 0.04,
        furrow_strength: 0.0,
        roughness_detail: 0.08,
        vein_scale: 7.0,
        vein_contrast: 0.4,
        transmission_strength: 0.5,
        transmission_red: 0.22,
        transmission_green: 0.5,
        transmission_blue: 0.08,
        thickness: 0.6,
        fissure_red: -0.04,
        fissure_green: -0.03,
        fissure_blue: -0.02,
        fissure_strength: 0.2,
        crest_red: 0.06,
        crest_green: 0.055,
        crest_blue: 0.045,
        crest_strength: 0.25,
        bark_mottle_scale: 0.12,
        bark_mottle_strength: 0.12,
        cavity_strength: 0.35,
        blade_mottle_scale: 5.0,
        blade_mottle_strength: 0.1,
        margin_width: 0.04,
        margin_red: 0.02,
        margin_green: 0.04,
        margin_blue: 0.008,
        cuticle_gloss: 0.48,
        sky_occlusion_strength: 0.35,
        // No plate network: smooth bark has none to draw.
        plate_cell_scale: 0.0,
        plate_elongation: 0.0,
        plate_dome: 0.0,
        plate_edge_lift: 0.0,
        plate_furrow_width: 0.0,
        plate_identity: 0.0,
        weathering_strength: 0.0,
        weathering_red: 0.0,
        weathering_green: 0.0,
        weathering_blue: 0.0,
        orientation_strength: 0.0,
        orientation_red: 0.0,
        orientation_green: 0.0,
        orientation_blue: 0.0,
        directional_occlusion: 0.0,
        depth_strength: 0.0,
        // B-BASE: pale lichen spots, many small and some broad, and a few
        // faint horizontal lines. No strip peels.
        lichen_scale: 0.015,
        lichen_coverage: 0.4,
        lichen_red: 0.76,
        lichen_green: 0.78,
        lichen_blue: 0.7,
        lichen_strength: 0.9,
        lenticel_density: 6.0,
        lenticel_length: 0.08,
        lenticel_strength: 0.6,
        lenticel_tint: -0.3,
        peel_curl: 0.0,
        peel_red: 0.0,
        peel_green: 0.0,
        peel_blue: 0.0,
    }
}

pub(super) fn birch() -> MaterialParams {
    MaterialParams {
        bark_red: 0.78,
        bark_green: 0.76,
        bark_blue: 0.7,
        bark_roughness: 0.48,
        // Young shoots dark red-brown, glossy, with pale resin warts; the
        // bark whitens only once it has thickened (VT Dendrology, Betula
        // pendula: twigs "slender, reddish brown"; bark "reddish brown ...
        // when very young, later turning white"; Atkinson 1992). S-BARE's
        // winter crown haze reads linear 0.061/0.042/0.030 to 0.149/0.093/
        // 0.066 on the exposure its trunk reads 0.78/0.78/0.71 on, R:G:B
        // 1:0.69:0.49 in its darker band. The matched still reddens a row:
        // 0.10/0.06/0.045 drew that band at 1:0.51:0.35, so the row is the
        // greyer red-brown that draws it near the photograph's. White from
        // about 4 cm across.
        shoot_red: 0.095,
        shoot_green: 0.07,
        shoot_blue: 0.055,
        shoot_radius: 0.01,
        // A light, yellow-green blade, paler beneath. S-WHOLE's crown reads
        // R:G:B 1:1.38:0.79 linear in its middle band (sRGB 92/108/82); the
        // older 0.055/0.165/0.035 drew that band at 1:1.93:0.79, a bluer and
        // darker green, and the rows below draw it at 83/105/73.
        leaf_front_red: 0.1,
        leaf_front_green: 0.19,
        leaf_front_blue: 0.06,
        leaf_back_red: 0.2,
        leaf_back_green: 0.28,
        leaf_back_blue: 0.14,
        hue_range_low: -0.025,
        hue_range_high: 0.025,
        brightness_range_low: -0.12,
        brightness_range_high: 0.12,
        interior_darkening: 0.15,
        // fn-40: the old base is dark and fissured and the stem above it
        // smooth. Relief comes in with maturity, from a radius of one ridge
        // width to two and a half, so at 8 cm it holds the root flare (20 cm
        // at a quarter metre, 13 cm at one metre, 10 cm by four at seed 1)
        // and leaves the stem and every limb smooth. Long, deep furrows.
        ridge_scale: 0.08,
        plate_scale: 0.4,
        furrow_strength: 1.0,
        roughness_detail: 0.1,
        vein_scale: 6.0,
        vein_contrast: 0.35,
        transmission_strength: 0.6,
        transmission_red: 0.28,
        transmission_green: 0.55,
        transmission_blue: 0.1,
        thickness: 0.45,
        fissure_red: -0.7,
        fissure_green: -0.7,
        fissure_blue: -0.66,
        fissure_strength: 1.0,
        crest_red: 0.1,
        crest_green: 0.1,
        crest_blue: 0.1,
        crest_strength: 0.4,
        bark_mottle_scale: 0.9,
        bark_mottle_strength: 0.18,
        cavity_strength: 0.3,
        blade_mottle_scale: 5.5,
        blade_mottle_strength: 0.12,
        margin_width: 0.05,
        margin_red: 0.03,
        margin_green: 0.05,
        margin_blue: 0.01,
        cuticle_gloss: 0.28,
        sky_occlusion_strength: 0.4,
        // The plate network as peeling strips: 3.6 cm plates at the base's
        // girth, stretched across by the curl into bands three times wider.
        plate_cell_scale: 0.12,
        plate_elongation: 0.0,
        plate_dome: 0.3,
        plate_edge_lift: 0.3,
        plate_furrow_width: 0.0,
        plate_identity: 0.3,
        weathering_strength: 0.0,
        weathering_red: 0.0,
        weathering_green: 0.0,
        weathering_blue: 0.0,
        orientation_strength: 0.0,
        orientation_red: 0.0,
        orientation_green: 0.0,
        orientation_blue: 0.0,
        directional_occlusion: 0.6,
        depth_strength: 0.4,
        // S-BARK: a chalk-white stem banded with dark lenticel dashes and a
        // few black blotches over a dark base. The dark there is the strips
        // that have peeled: seven in ten at the flare, fewer as the wood
        // thins, each one whole, down to none where the relief ends.
        lichen_scale: 0.06,
        lichen_coverage: 0.12,
        lichen_red: 0.07,
        lichen_green: 0.07,
        lichen_blue: 0.07,
        lichen_strength: 0.75,
        lenticel_density: 18.0,
        lenticel_length: 0.06,
        lenticel_strength: 0.9,
        lenticel_tint: -0.9,
        peel_curl: 0.7,
        peel_red: 0.1,
        peel_green: 0.085,
        peel_blue: 0.075,
    }
}

pub(super) fn radiant(silver: bool) -> MaterialParams {
    MaterialParams {
        fissure_strength: 0.12,
        crest_strength: 0.1,
        bark_mottle_strength: 0.05,
        blade_mottle_strength: 0.04,
        cavity_strength: 0.2,
        cuticle_gloss: if silver { 0.25 } else { 0.3 },
        sky_occlusion_strength: 0.2,
        ..MaterialParams::default()
    }
}
