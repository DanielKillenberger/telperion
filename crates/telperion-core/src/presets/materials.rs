//! Material value tables kept beside the geometry presets.
use crate::material::MaterialParams;

pub(super) fn oak() -> MaterialParams {
    MaterialParams {
        bark_red: 0.185,
        bark_green: 0.175,
        bark_blue: 0.158,
        bark_roughness: 0.85,
        bark_grain_scale: 0.0025,
        bark_grain_strength: 0.2,
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
        // Neutral: a card lit alone, until this table states its canopy.
        canopy_normal: 0.0,
        light_wrap: 0.0,
        diffuse_transmission: 0.0,
        leaf_sheen: 0.0,
        crown_shade: 0.0,
        // Plated bark: no lichen, lenticel or peel term of fn-40's.
        ..MaterialParams::default()
    }
}

pub(super) fn spruce() -> MaterialParams {
    MaterialParams {
        bark_red: 0.20,
        bark_green: 0.175,
        bark_blue: 0.148,
        bark_roughness: 0.9,
        bark_grain_scale: 0.0018,
        bark_grain_strength: 0.15,
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
        // Neutral: a card lit alone, until this table states its canopy.
        canopy_normal: 0.0,
        light_wrap: 0.0,
        diffuse_transmission: 0.0,
        leaf_sheen: 0.0,
        crown_shade: 0.0,
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
        // Rough, not glossy, in the oak's and the spruce's range after the
        // owner read the close-ups as plastic. At B-BASE's own light the old
        // 0.4 highlight was already all but unseen; 0.95 leaves a sheen of
        // three code values, where 0.9 left five.
        bark_roughness: 0.95,
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
        // B-WHOLE's own leaf pixels, its centre crop, read linear
        // 1:1.78:1.06, a grey green. The blade is set greyer than the old
        // 1:4.8:0.8 front and lighter on both faces: an olive front and a
        // pale grey-green back. The render lands greener than either face,
        // 1:2.17:1.04, because what passes through the leaf adds its own
        // green: with both faces black the centre still reads 38/57/40.
        leaf_front_red: 0.08,
        leaf_front_green: 0.15,
        leaf_front_blue: 0.05,
        leaf_back_red: 0.21,
        leaf_back_green: 0.28,
        leaf_back_blue: 0.15,
        hue_range_low: -0.02,
        hue_range_high: 0.02,
        brightness_range_low: -0.1,
        brightness_range_high: 0.1,
        interior_darkening: 0.35,
        // Smooth bark: the ridge field only as a 2 mm grain with no furrow and
        // no tint. At a centimetre and more it drew vertical wavy ridges with
        // bright rims on B-BASE; tinted at 2 mm, a crackle glaze.
        ridge_scale: 0.002,
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
        fissure_strength: 0.0,
        crest_red: 0.06,
        crest_green: 0.055,
        crest_blue: 0.045,
        crest_strength: 0.0,
        bark_mottle_scale: 0.12,
        bark_mottle_strength: 0.1,
        cavity_strength: 0.35,
        blade_mottle_scale: 5.0,
        blade_mottle_strength: 0.1,
        margin_width: 0.04,
        margin_red: 0.02,
        margin_green: 0.04,
        margin_blue: 0.008,
        // A matte cuticle's glint: at 0.48, the glossiest leaf in the
        // catalogue, the sun's white highlight read as plastic (owner,
        // fn-54); the oak's is 0.35 and the birch's 0.28.
        cuticle_gloss: 0.18,
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
        // A dense crown lit as one mass (fn-52): the lighting normal bends
        // most of the way to the crown's outward direction and the sun wraps
        // a little past the terminator, so the sky-lit top and the sunward
        // shell carry the crown. A thin blade transmits close to evenly
        // (Jacquemoud & Ustin 2019, Leaf Optical Properties), sky and sun
        // alike. The glossy cuticle's sheen sits above a smooth wax's 0.04:
        // B-WHOLE's brightest centre pixels are a pale, sky-lit green,
        // 149/180/152, not sky; at 0.1 the whole crown greyed. Set against
        // B-WHOLE's centre, sRGB 71/94/74 under overcast, where this row
        // drew 61/84/65 before the short shoots.
        canopy_normal: 0.8,
        light_wrap: 0.4,
        diffuse_transmission: 1.0,
        leaf_sheen: 0.08,
        // The crown over a leaf takes its sky, so the dome's underside falls
        // into its own shade: B-WHOLE's leaf mass falls from 125 at the top
        // sixth to 52 at the bottom, and 0.15 a radius drew 91 to 55, where
        // no shade drew the bottom brighter than the middle. Under the short
        // shoots' denser mass, 0.1 lifts the centre by two and a half points
        // over 0.15 and still falls from 126 at the top sixth to 76.
        crown_shade: 0.1,
        // Each clump of leaves takes the sky from what hangs under it (fn-54):
        // B-WHOLE's crown reads as lit billows with shade pockets under and
        // between them, which one smooth ellipsoid cannot draw. The whole of
        // the row, so a leaf under a full clump sees none of the sky its
        // own mass holds off.
        lobe_shade: 0.7,
        // B-BASE: lichen in few patches of widely varied size, the small ones
        // bright white and the broad ones thin grey-green, and a few faint
        // horizontal lines. No strip peels.
        lichen_scale: 0.04,
        lichen_coverage: 0.5,
        lichen_red: 0.8,
        lichen_green: 0.82,
        lichen_blue: 0.76,
        lichen_strength: 1.0,
        lenticel_density: 6.0,
        lenticel_length: 0.08,
        lenticel_strength: 0.25,
        lenticel_tint: -0.3,
        peel_curl: 0.0,
        peel_red: 0.0,
        peel_green: 0.0,
        peel_blue: 0.0,
        // fn-55: a dielectric's foot under the one highlight, and the blade's
        // cells between its veins at ninety a blade. fn-71 sets the bark's
        // grain at 2 mm cells, three pixels at B-BASE's 1440, where the
        // close-up reads 2.37 against the resolution contract's 3.0; 1 mm
        // cells sat on the half-size still's Nyquist edge and read 2.76.
        bark_reflectance: 0.04,
        leaf_reflectance: 0.04,
        bark_grain_scale: 0.002,
        bark_grain_strength: 0.3,
        blade_grain_scale: 90.0,
        blade_grain_strength: 0.3,
    }
}

pub(super) fn birch() -> MaterialParams {
    MaterialParams {
        bark_red: 0.78,
        bark_green: 0.76,
        bark_blue: 0.7,
        // Rough, not glossy, in the oak's and the spruce's range after the
        // owner read the close-ups as plastic.
        bark_roughness: 0.95,
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
        vein_contrast: 0.8,
        transmission_strength: 0.6,
        transmission_red: 0.28,
        transmission_green: 0.55,
        transmission_blue: 0.1,
        thickness: 0.45,
        fissure_red: -0.44,
        fissure_green: -0.44,
        fissure_blue: -0.41,
        fissure_strength: 1.0,
        crest_red: 0.1,
        crest_green: 0.1,
        crest_blue: 0.1,
        crest_strength: 0.4,
        bark_mottle_scale: 0.15,
        bark_mottle_strength: 0.12,
        cavity_strength: 0.3,
        blade_mottle_scale: 5.5,
        blade_mottle_strength: 0.2,
        margin_width: 0.05,
        margin_red: 0.03,
        margin_green: 0.05,
        margin_blue: 0.01,
        cuticle_gloss: 0.28,
        sky_occlusion_strength: 0.4,
        // The plate network as peeling strips: 1.5 cm plates at the base's
        // girth, stretched across by the curl into bands three times wider.
        plate_cell_scale: 0.05,
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
        // S-BARK: a chalk-white stem with a fine grey grain, banded with dark
        // lenticel dashes and small, ragged grey marks over a darker base.
        // The marks are the strips that have peeled, grey inner bark: most at
        // the flare, fewer as the wood thins, each one whole, none where the
        // relief ends. The lichen is only the grain. The owner at round 22:
        // "too much contrast. The texture in the reference isn't black." At
        // 0.65 of the strips peeled and a near-black inner bark, 40% of the
        // stem's pixels read near-black against the photograph's 11%; a
        // third peeled, a grey inner bark and lighter dashes and fissures
        // read 6%, and the stem is mostly white as the photograph's is.
        lichen_scale: 0.01,
        lichen_coverage: 1.0,
        lichen_red: 0.5,
        lichen_green: 0.5,
        lichen_blue: 0.48,
        lichen_strength: 0.45,
        lenticel_density: 18.0,
        lenticel_length: 0.06,
        lenticel_strength: 0.72,
        lenticel_tint: -0.62,
        peel_curl: 0.3,
        peel_red: 0.33,
        peel_green: 0.315,
        peel_blue: 0.29,
        // The same canopy as the beech's (fn-52), with a softer terminator
        // for the airy hanging crown and a less glossy cuticle. Set against
        // S-WHOLE, whose centre pixels sit above half brightness a fifth of
        // the time: this row puts 20% of the leaf pixels there, where the
        // card put 2%, and the centre's leaf pixels read 81 against the
        // photograph's 83.
        canopy_normal: 0.8,
        light_wrap: 0.5,
        diffuse_transmission: 1.0,
        leaf_sheen: 0.06,
        // S-WHOLE's crown falls from 153 at the top sixth to 38 at the
        // bottom; 0.2 a radius draws 156 to 45.
        crown_shade: 0.2,
        lobe_shade: 0.0,
        // fn-55: as the beech's. S-BARK's chalk white is grained at the pixel
        // in the photograph: 2 mm cells, two and a bit pixels at S-BARK's
        // 1440, at 0.15 (fn-71). The close-up reads 2.90 against the
        // resolution contract's 3.0 with the grain box-averaged over the
        // pixel, 2.75 with none; 0.2 read 2.999 and 0.3 read 3.24, the
        // grain's tilt of the normal being what the reduction cannot match.
        bark_reflectance: 0.04,
        leaf_reflectance: 0.04,
        bark_grain_scale: 0.002,
        bark_grain_strength: 0.15,
        blade_grain_scale: 90.0,
        blade_grain_strength: 0.3,
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
