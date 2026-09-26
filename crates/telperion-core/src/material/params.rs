//! The material's value table: every row declared once, with its catalogue entry.
use crate::catalogue::{bounded, input, tuned, Bounds, Site};

crate::catalogue::rows! {
    #[derive(Debug, Clone, Copy, PartialEq)]
    pub struct MaterialParams in "/material" {
        /// The bark's own colour, a linear reflectance per channel. Raising a
        /// channel pushes mature bark toward that colour.
        pub bark_red: f64 = "barkRed" "reflectance" Bounds::closed(0.0, 1.0) => [Draw] {
            wire: 153,
            check: input(Site::Material, 73, "bark red"),
            dial: bounded("material_bark_red", "the red in the bark's own colour", [0.15, 0.3]),
        },
        /// The green channel of `bark_red`.
        pub bark_green: f64 = "barkGreen" "reflectance" Bounds::closed(0.0, 1.0) => [Draw] {
            wire: 154,
            check: input(Site::Material, 74, "bark green"),
            dial: bounded("material_bark_green", "the green in the bark's own colour", [0.15, 0.3]),
        },
        /// The blue channel of `bark_red`.
        pub bark_blue: f64 = "barkBlue" "reflectance" Bounds::closed(0.0, 1.0) => [Draw] {
            wire: 155,
            check: input(Site::Material, 75, "bark blue"),
            dial: bounded("material_bark_blue", "the blue in the bark's own colour", [0.15, 0.3]),
        },
        /// How diffuse the bark is: 0 is a mirror, 1 is chalk.
        pub bark_roughness: f64 = "barkRoughness" "-" Bounds::closed(0.0, 1.0) => [Draw] {
            wire: 156,
            check: input(Site::Material, 76, "bark roughness"),
            dial: bounded("material_bark_roughness", "how diffuse the bark is; nought is a \
                mirror and one is chalk", [0.15, 0.3]),
        },
        /// The young wood's own colour, before its bark has formed. Wood thinner
        /// than `shoot_radius` takes it, and gives it up to the bark colour on a
        /// smoothstep of its radius by twice that; zero means no wood is young.
        pub shoot_red: f64 = "shootRed" "reflectance" Bounds::closed(0.0, 1.0) => [Draw] {
            wire: 157,
            check: input(Site::Material, 77, "young shoot red"),
            applies: "`shootRadius` zero",
            dial: bounded("material_shoot_red", "the red in young wood's colour, before its bark \
                has formed", [0.15, 0.3]),
        },
        /// The green channel of `shoot_red`.
        pub shoot_green: f64 = "shootGreen" "reflectance" Bounds::closed(0.0, 1.0) => [Draw] {
            wire: 158,
            check: input(Site::Material, 78, "young shoot green"),
            applies: "`shootRadius` zero",
            dial: bounded("material_shoot_green", "the green in young wood's colour, before its \
                bark has formed", [0.15, 0.3]),
        },
        /// The blue channel of `shoot_red`.
        pub shoot_blue: f64 = "shootBlue" "reflectance" Bounds::closed(0.0, 1.0) => [Draw] {
            wire: 159,
            check: input(Site::Material, 79, "young shoot blue"),
            applies: "`shootRadius` zero",
            dial: bounded("material_shoot_blue", "the blue in young wood's colour, before its \
                bark has formed", [0.15, 0.3]),
        },
        /// Metres. The radius below which wood is young.
        pub shoot_radius: f64 = "shootRadius" "m" Bounds::closed(0.0, 0.1) => [Draw] {
            wire: 160,
            check: input(Site::Material, 80, "young shoot radius"),
            dial: bounded("material_shoot_radius", "the metres of radius below which wood counts \
                as young", [0.015, 0.03]),
        },
        /// The colour of a leaf's upper face, a linear reflectance per
        /// channel. Raising a channel pushes the sunlit face toward it.
        pub leaf_front_red: f64 = "leafFrontRed" "reflectance" Bounds::closed(0.0, 1.0) => [Draw] {
            wire: 161,
            check: input(Site::Material, 81, "leaf front red"),
            dial: bounded("material_leaf_front_red", "the red in the colour of a leaf's upper \
                face", [0.02, 0.04]).span([0.0, 0.141]),
        },
        /// The green channel of `leaf_front_red`.
        pub leaf_front_green: f64 = "leafFrontGreen" "reflectance"
            Bounds::closed(0.0, 1.0) => [Draw] {
            wire: 162,
            check: input(Site::Material, 82, "leaf front green"),
            dial: bounded("material_leaf_front_green", "the green in the colour of a leaf's \
                upper face", [0.15, 0.3]),
        },
        /// The blue channel of `leaf_front_red`.
        pub leaf_front_blue: f64 = "leafFrontBlue" "reflectance"
            Bounds::closed(0.0, 1.0) => [Draw] {
            wire: 163,
            check: input(Site::Material, 83, "leaf front blue"),
            dial: bounded("material_leaf_front_blue", "the blue in the colour of a leaf's upper \
                face", [0.015, 0.03]).span([0.0, 0.082]),
        },
        /// The colour of a leaf's underside, shown wherever the eye sees the
        /// back of a blade. Raising a channel pushes that face toward it.
        pub leaf_back_red: f64 = "leafBackRed" "reflectance" Bounds::closed(0.0, 1.0) => [Draw] {
            wire: 164,
            check: input(Site::Material, 84, "leaf back red"),
            dial: bounded("material_leaf_back_red", "the red in the colour of a leaf's underside",
                [0.15, 0.3]),
        },
        /// The green channel of `leaf_back_red`.
        pub leaf_back_green: f64 = "leafBackGreen" "reflectance"
            Bounds::closed(0.0, 1.0) => [Draw] {
            wire: 165,
            check: input(Site::Material, 85, "leaf back green"),
            dial: bounded("material_leaf_back_green", "the green in the colour of a leaf's \
                underside", [0.025, 0.05]).span([0.1525, 0.3225]),
        },
        /// The blue channel of `leaf_back_red`.
        pub leaf_back_blue: f64 = "leafBackBlue" "reflectance" Bounds::closed(0.0, 1.0) => [Draw] {
            wire: 166,
            check: input(Site::Material, 86, "leaf back blue"),
            dial: bounded("material_leaf_back_blue", "the blue in the colour of a leaf's \
                underside", [0.025, 0.05]).span([0.03, 0.19]),
        },
        /// The colour a dead frond has aged to, both faces alike, shown only on
        /// the fronds a rosette keeps below its living crown. Raising a channel
        /// pushes the skirt toward it.
        pub leaf_dead_red: f64 = "leafDeadRed" "reflectance" Bounds::closed(0.0, 1.0) => [Draw] {
            wire: 167,
            check: input(Site::Material, 87, "leaf dead red"),
            applies: "no dead frond (`canopy.skirtFronds` zero)",
            dial: bounded("material_leaf_dead_red", "the red in the colour a dead frond has aged \
                to", [0.05, 0.1]),
        },
        /// The green channel of `leaf_dead_red`.
        pub leaf_dead_green: f64 = "leafDeadGreen" "reflectance"
            Bounds::closed(0.0, 1.0) => [Draw] {
            wire: 168,
            check: input(Site::Material, 88, "leaf dead green"),
            applies: "no dead frond (`canopy.skirtFronds` zero)",
            dial: bounded("material_leaf_dead_green", "the green in the colour a dead frond has \
                aged to", [0.05, 0.1]),
        },
        /// The blue channel of `leaf_dead_red`.
        pub leaf_dead_blue: f64 = "leafDeadBlue" "reflectance" Bounds::closed(0.0, 1.0) => [Draw] {
            wire: 169,
            check: input(Site::Material, 89, "leaf dead blue"),
            applies: "no dead frond (`canopy.skirtFronds` zero)",
            dial: bounded("material_leaf_dead_blue", "the blue in the colour a dead frond has \
                aged to", [0.05, 0.1]),
        },
        /// The hue offsets one leaf may take, as a fraction of the colour circle.
        pub hue_range_low: f64 = "hueRangeLow" "-" Bounds::closed(-0.5, 0.5) => [Draw] {
            wire: 170,
            check: input(Site::Material, 90, "leaf hue range low"),
            note: "Refused above `hueRangeHigh` (`leaf hue range`).",
            dial: tuned("material_hue_range_low", "the lowest hue offset one leaf may take, as a \
                share of the colour circle",
                [-0.5, 0.0], [0.0025, 0.005], "validated bound").span([-0.035, -0.015]),
        },
        /// The upper end of the hue offsets `hue_range_low` opens.
        pub hue_range_high: f64 = "hueRangeHigh" "-" Bounds::closed(-0.5, 0.5) => [Draw] {
            wire: 171,
            check: input(Site::Material, 91, "leaf hue range high"),
            note: "Refused below `hueRangeLow` (`leaf hue range`).",
            dial: tuned("material_hue_range_high", "the highest hue offset one leaf may take, as \
                a share of the colour circle",
                [0.0, 0.5], [0.0025, 0.005], "validated bound").span([0.015, 0.035]),
        },
        /// The brightness offsets one leaf may take, about no change at all.
        pub brightness_range_low: f64 = "brightnessRangeLow" "-"
            Bounds::closed(-1.0, 1.0) => [Draw] {
            wire: 172,
            check: input(Site::Material, 92, "leaf brightness range low"),
            note: "Refused above `brightnessRangeHigh` (`leaf brightness range`).",
            dial: tuned("material_brightness_range_low", "the lowest brightness offset one leaf \
                may take", [-1.0, 0.0], [0.015, 0.03], "validated bound").span([-0.175, -0.075]),
        },
        /// The upper end of the brightness offsets `brightness_range_low` opens.
        pub brightness_range_high: f64 = "brightnessRangeHigh" "-"
            Bounds::closed(-1.0, 1.0) => [Draw] {
            wire: 173,
            check: input(Site::Material, 93, "leaf brightness range high"),
            note: "Refused below `brightnessRangeLow` (`leaf brightness range`).",
            dial: tuned("material_brightness_range_high", "the highest brightness offset one \
                leaf may take", [0.0, 1.0], [0.015, 0.03], "validated bound").span([0.075, 0.175]),
        },
        /// How far a leaf deep inside the crown is darkened towards a shaded mass.
        pub interior_darkening: f64 = "interiorDarkening" "-" Bounds::closed(0.0, 1.0) => [Draw] {
            wire: 174,
            check: input(Site::Material, 94, "leaf interior darkening"),
            dial: bounded("material_interior_darkening", "how far a leaf deep inside the crown \
                is darkened toward a shaded mass", [0.15, 0.3]),
        },
        /// Circumferential ridge spacing in metres; zero disables relief.
        pub ridge_scale: f64 = "ridgeScale" "m" Bounds::closed(0.0, 1.0) => [Draw] {
            wire: 175,
            check: input(Site::Material, 62, "bark ridge scale"),
            dial: bounded("material_ridge_scale", "the metres between the bark's circumferential \
                ridges; zero leaves no relief", [0.02, 0.04]).span([0.0, 0.12]),
        },
        /// Axial scale control in metres; spacing is bounded to 1.5–2 ridge widths.
        /// Larger ratios lengthen and deepen furrows; zero omits breaks.
        pub plate_scale: f64 = "plateScale" "m" Bounds::closed(0.0, 1.0) => [Draw] {
            wire: 176,
            check: input(Site::Material, 63, "bark plate scale"),
            applies: "`ridgeScale` zero",
            dial: bounded("material_plate_scale", "the axial scale of the bark's plates, in \
                metres; larger lengthens and deepens the furrows", [0.15, 0.3]),
        },
        /// Furrow width and depth together; zero leaves tightly packed scales.
        pub furrow_strength: f64 = "furrowStrength" "-" Bounds::closed(0.0, 1.0) => [Draw] {
            wire: 177,
            check: input(Site::Material, 64, "bark furrow strength"),
            applies: "`ridgeScale` zero",
            dial: bounded("material_furrow_strength", "the width and depth of the bark's furrows \
                together; at zero no furrow is cut at all, and any rise starts them", [0.15, 0.3]),
        },
        /// Roughness variation about the base value, clamped to 0..1.
        pub roughness_detail: f64 = "roughnessDetail" "-" Bounds::closed(0.0, 1.0) => [Draw] {
            wire: 178,
            check: input(Site::Material, 65, "bark roughness detail"),
            dial: bounded("material_roughness_detail", "how far roughness varies about the \
                bark's base value", [0.15, 0.3]),
        },
        /// Secondary vein pairs per blade, continuously interpolated.
        pub vein_scale: f64 = "veinScale" "-" Bounds::closed(0.0, 32.0) => [Draw] {
            wire: 179,
            check: input(Site::Material, 66, "leaf vein scale"),
            dial: bounded("material_vein_scale", "secondary vein pairs per blade",
                [0.5, 1.0]).span([5.0, 9.0]),
        },
        /// How far the veins are lightened and the blade between them
        /// darkened. Raising it makes the venation read more sharply.
        pub vein_contrast: f64 = "veinContrast" "-" Bounds::closed(0.0, 1.0) => [Draw] {
            wire: 180,
            check: input(Site::Material, 67, "leaf vein contrast"),
            dial: bounded("material_vein_contrast", "how far the veins are lightened against the \
                blade between them", [0.15, 0.3]),
        },
        /// How brightly a backlit leaf glows with the light that came through
        /// it. Raising it lifts that glow.
        pub transmission_strength: f64 = "transmissionStrength" "-"
            Bounds::closed(0.0, 1.0) => [Draw] {
            wire: 181,
            check: input(Site::Material, 68, "leaf transmission strength"),
            dial: bounded("material_transmission_strength", "how brightly a backlit leaf glows \
                with the light that came through it", [0.15, 0.3]),
        },
        /// Linear transmission tint, in 0..1 per channel.
        pub transmission_red: f64 = "transmissionRed" "reflectance"
            Bounds::closed(0.0, 1.0) => [Draw] {
            wire: 182,
            check: input(Site::Material, 69, "leaf transmission red"),
            applies: "`transmissionStrength` zero",
            dial: bounded("material_transmission_red", "the red in the tint of the light that \
                comes through a leaf", [0.15, 0.3]),
        },
        /// The green channel of `transmission_red`.
        pub transmission_green: f64 = "transmissionGreen" "reflectance"
            Bounds::closed(0.0, 1.0) => [Draw] {
            wire: 183,
            check: input(Site::Material, 70, "leaf transmission green"),
            applies: "`transmissionStrength` zero",
            dial: bounded("material_transmission_green", "the green in the tint of the light \
                that comes through a leaf", [0.15, 0.3]),
        },
        /// The blue channel of `transmission_red`.
        pub transmission_blue: f64 = "transmissionBlue" "reflectance"
            Bounds::closed(0.0, 1.0) => [Draw] {
            wire: 184,
            check: input(Site::Material, 71, "leaf transmission blue"),
            applies: "`transmissionStrength` zero",
            dial: bounded("material_transmission_blue", "the blue in the tint of the light that \
                comes through a leaf", [0.01, 0.02]).span([0.055, 0.115]),
        },
        /// Optical thickness: attenuation is exp(-thickness).
        pub thickness: f64 = "thickness" "-" Bounds::closed(0.0, 8.0) => [Draw] {
            wire: 185,
            check: input(Site::Material, 72, "leaf thickness"),
            applies: "`transmissionStrength` zero",
            dial: bounded("material_thickness", "the blade's optical thickness; what comes \
                through falls off with it", [1.0, 2.0]),
        },
        /// The tint carried by the floors of the bark's furrows, as an offset
        /// per channel, and how far it is laid over the bark colour there.
        pub fissure_red: f64 = "fissureRed" "offset" Bounds::closed(-1.0, 1.0) => [Draw] {
            wire: 186,
            check: input(Site::Material, 0, "bark fissure red"),
            applies: "`fissureStrength` zero",
            dial: bounded("material_fissure_red", "the red in the tint carried by the floors of \
                the bark's furrows", [0.25, 0.5]),
        },
        /// The green channel of `fissure_red`.
        pub fissure_green: f64 = "fissureGreen" "offset" Bounds::closed(-1.0, 1.0) => [Draw] {
            wire: 187,
            check: input(Site::Material, 1, "bark fissure green"),
            applies: "`fissureStrength` zero",
            dial: bounded("material_fissure_green", "the green in the tint carried by the floors \
                of the bark's furrows", [0.25, 0.5]),
        },
        /// The blue channel of `fissure_red`.
        pub fissure_blue: f64 = "fissureBlue" "offset" Bounds::closed(-1.0, 1.0) => [Draw] {
            wire: 188,
            check: input(Site::Material, 2, "bark fissure blue"),
            applies: "`fissureStrength` zero",
            dial: bounded("material_fissure_blue", "the blue in the tint carried by the floors \
                of the bark's furrows", [0.25, 0.5]),
        },
        /// How far `fissure_red`'s tint is laid over the bark colour in the furrows.
        pub fissure_strength: f64 = "fissureStrength" "-" Bounds::closed(0.0, 1.0) => [Draw] {
            wire: 189,
            check: input(Site::Material, 3, "bark fissure strength"),
            dial: bounded("material_fissure_strength", "how far the furrow tint is laid over the \
                bark colour; at zero the furrow tint is not laid on at all, and any rise starts \
                it", [0.15, 0.3]),
        },
        /// The tint carried by the crests of the bark's ridges, as an offset
        /// per channel, and how far it is laid over the bark colour there.
        pub crest_red: f64 = "crestRed" "offset" Bounds::closed(-1.0, 1.0) => [Draw] {
            wire: 190,
            check: input(Site::Material, 4, "bark crest red"),
            applies: "`crestStrength` zero",
            dial: bounded("material_crest_red", "the red in the tint carried by the crests of \
                the bark's ridges", [0.025, 0.05]).span([-0.05, 0.15]),
        },
        /// The green channel of `crest_red`.
        pub crest_green: f64 = "crestGreen" "offset" Bounds::closed(-1.0, 1.0) => [Draw] {
            wire: 191,
            check: input(Site::Material, 5, "bark crest green"),
            applies: "`crestStrength` zero",
            dial: bounded("material_crest_green", "the green in the tint carried by the crests \
                of the bark's ridges", [0.025, 0.05]).span([-0.05, 0.15]),
        },
        /// The blue channel of `crest_red`.
        pub crest_blue: f64 = "crestBlue" "offset" Bounds::closed(-1.0, 1.0) => [Draw] {
            wire: 192,
            check: input(Site::Material, 6, "bark crest blue"),
            applies: "`crestStrength` zero",
            dial: bounded("material_crest_blue", "the blue in the tint carried by the crests of \
                the bark's ridges", [0.025, 0.05]).span([-0.05, 0.15]),
        },
        /// How far `crest_red`'s tint is laid over the bark colour on the crests.
        pub crest_strength: f64 = "crestStrength" "-" Bounds::closed(0.0, 1.0) => [Draw] {
            wire: 193,
            check: input(Site::Material, 7, "bark crest strength"),
            dial: bounded("material_crest_strength", "how far the crest tint is laid over the \
                bark colour; at zero the crest tint is not laid on at all, and any rise starts \
                it", [0.15, 0.3]),
        },
        /// The size of the blotches in the bark's mottling, and how far they
        /// lighten and darken its colour. A scale of zero leaves none.
        pub bark_mottle_scale: f64 = "barkMottleScale" "-" Bounds::closed(0.0, 8.0) => [Draw] {
            wire: 194,
            check: input(Site::Material, 8, "bark mottle scale"),
            applies: "`barkMottleStrength` zero",
            dial: bounded("material_bark_mottle_scale", "the size of the blotches in the bark's \
                mottling; zero leaves none", [1.0, 2.0]),
        },
        /// How far the blotches `bark_mottle_scale` sizes lighten and darken the bark.
        pub bark_mottle_strength: f64 = "barkMottleStrength" "-"
            Bounds::closed(0.0, 1.0) => [Draw] {
            wire: 195,
            check: input(Site::Material, 9, "bark mottle strength"),
            applies: "`barkMottleScale` zero",
            dial: bounded("material_bark_mottle_strength", "how far that mottling lightens and \
                darkens the bark", [0.15, 0.3]),
        },
        /// How far the hollows of the bark - furrow floors and the sockets
        /// where a limb joins - are darkened. Raising it sinks them deeper.
        pub cavity_strength: f64 = "cavityStrength" "-" Bounds::closed(0.0, 1.0) => [Draw] {
            wire: 196,
            check: input(Site::Material, 10, "bark cavity strength"),
            dial: bounded("material_cavity_strength", "how far the bark's hollows and the \
                sockets at a fork are darkened", [0.15, 0.3]),
        },
        /// The size of the blotches in a leaf's mottling, and how far they
        /// vary its colour. A scale of zero leaves none.
        pub blade_mottle_scale: f64 = "bladeMottleScale" "-" Bounds::closed(0.0, 32.0) => [Draw] {
            wire: 197,
            check: input(Site::Material, 11, "leaf blade mottle scale"),
            applies: "`bladeMottleStrength` zero",
            dial: bounded("material_blade_mottle_scale", "the size of the blotches in a leaf's \
                mottling; zero leaves none", [2.0, 4.0]).span([0.0, 12.0]),
        },
        /// How far the blotches `blade_mottle_scale` sizes vary a leaf's colour.
        pub blade_mottle_strength: f64 = "bladeMottleStrength" "-"
            Bounds::closed(0.0, 1.0) => [Draw] {
            wire: 198,
            check: input(Site::Material, 12, "leaf blade mottle strength"),
            applies: "`bladeMottleScale` zero",
            dial: bounded("material_blade_mottle_strength", "how far that mottling varies the \
                blade's colour", [0.15, 0.3]),
        },
        /// How wide a band along the leaf's edge takes the margin colour, and
        /// what that colour is as an offset per channel. Raising the width
        /// broadens the rim around every leaf.
        pub margin_width: f64 = "marginWidth" "-" Bounds::closed(0.0, 0.5) => [Draw] {
            wire: 199,
            check: input(Site::Material, 13, "leaf margin width"),
            dial: bounded("material_margin_width", "how wide a band along the leaf's edge takes \
                the margin tint", [0.1, 0.2]),
        },
        /// The margin colour's red channel, as an offset; `margin_width` sets its band.
        pub margin_red: f64 = "marginRed" "offset" Bounds::closed(-1.0, 1.0) => [Draw] {
            wire: 200,
            check: input(Site::Material, 14, "leaf margin red"),
            applies: "`marginWidth` zero",
            dial: bounded("material_margin_red", "the red in the tint along a leaf's edge",
                [0.01, 0.02]).span([-0.015, 0.045]),
        },
        /// The margin colour's green channel, as an offset; `margin_width` sets its band.
        pub margin_green: f64 = "marginGreen" "offset" Bounds::closed(-1.0, 1.0) => [Draw] {
            wire: 201,
            check: input(Site::Material, 15, "leaf margin green"),
            applies: "`marginWidth` zero",
            dial: bounded("material_margin_green", "the green in the tint along a leaf's edge",
                [0.015, 0.03]).span([-0.025, 0.075]),
        },
        /// The margin colour's blue channel, as an offset; `margin_width` sets its band.
        pub margin_blue: f64 = "marginBlue" "offset" Bounds::closed(-1.0, 1.0) => [Draw] {
            wire: 202,
            check: input(Site::Material, 16, "leaf margin blue"),
            applies: "`marginWidth` zero",
            dial: bounded("material_margin_blue", "the blue in the tint along a leaf's edge",
                [0.0025, 0.005]).span([-0.005, 0.015]),
        },
        /// How tight the highlight on a leaf's upper face is. Raising it draws
        /// the glint into a smaller, glossier spot.
        pub cuticle_gloss: f64 = "cuticleGloss" "-" Bounds::closed(0.0, 1.0) => [Draw] {
            wire: 203,
            check: input(Site::Material, 17, "leaf cuticle gloss"),
            dial: bounded("material_cuticle_gloss", "how tight the highlight on a leaf's upper \
                face is", [0.15, 0.3]),
        },
        /// How much of the sky is withheld from bark and leaves the deeper
        /// they sit in the crown. Raising it darkens the crown's interior.
        pub sky_occlusion_strength: f64 = "skyOcclusionStrength" "-"
            Bounds::closed(0.0, 1.0) => [Draw] {
            wire: 204,
            check: input(Site::Material, 18, "sky occlusion strength"),
            dial: bounded("material_sky_occlusion_strength", "how much of the sky is withheld \
                from bark and leaves deep in the crown", [0.15, 0.3]),
        },
        /// Circumferential size of one bark plate in metres, before girth scales
        /// it. Zero leaves the field the ridges it has always been.
        pub plate_cell_scale: f64 = "plateCellScale" "m" Bounds::closed(0.0, 1.0) => [Draw] {
            wire: 205,
            check: input(Site::Material, 19, "bark plate cell scale"),
            applies: "`ridgeScale` zero",
            dial: bounded("material_plate_cell_scale", "the metres across one bark plate, before \
                girth scales it; at zero the bark has no plates at all, and any rise switches \
                plates on", [0.02, 0.04]).span([0.0, 0.126]),
        },
        /// How much longer a plate runs than it is wide: nought is as long as it
        /// is wide, one is twice as long.
        pub plate_elongation: f64 = "plateElongation" "-" Bounds::closed(0.0, 16.0) => [Draw] {
            wire: 206,
            check: input(Site::Material, 20, "bark plate elongation"),
            applies: "`plateCellScale` or `ridgeScale` zero",
            dial: bounded("material_plate_elongation", "how much longer a plate runs than it is \
                wide", [2.5, 5.0]),
        },
        /// How far a plate's face rises from its own edge towards its middle. At
        /// zero a plate's face lies flat, and any rise starts the doming.
        pub plate_dome: f64 = "plateDome" "-" Bounds::closed(0.0, 1.0) => [Draw] {
            wire: 207,
            check: input(Site::Material, 21, "bark plate dome"),
            applies: "`plateCellScale` or `ridgeScale` zero",
            dial: bounded("material_plate_dome", "how far a plate's face rises from its edge \
                toward its middle; at zero a plate's face lies flat, and any rise starts the \
                doming", [0.15, 0.3]),
        },
        /// How far a plate's rim stands off the furrow it borders: the scale that
        /// lifts rather than the plate that sits flat. At zero a plate's rim lies
        /// flush with its furrow, and any rise starts the lift.
        pub plate_edge_lift: f64 = "plateEdgeLift" "-" Bounds::closed(0.0, 1.0) => [Draw] {
            wire: 208,
            check: input(Site::Material, 22, "bark plate edge lift"),
            applies: "`plateCellScale` or `ridgeScale` zero",
            dial: bounded("material_plate_edge_lift", "how far a plate's rim stands off the \
                furrow it borders; at zero a plate's rim lies flush with its furrow, and any \
                rise starts the lift", [0.15, 0.3]),
        },
        /// How wide the flat floor of a furrow is cut, as a fraction of a plate's
        /// own width, so a bigger plate carries a wider furrow off one row. Zero
        /// leaves the hairline the network has always cut between two faces.
        pub plate_furrow_width: f64 = "plateFurrowWidth" "-" Bounds::closed(0.0, 1.0) => [Draw] {
            wire: 209,
            check: input(Site::Material, 23, "bark plate furrow width"),
            applies: "`plateCellScale` or `ridgeScale` zero",
            dial: bounded("material_plate_furrow_width", "how wide the flat floor of a furrow is \
                cut, as a share of a plate's width", [0.15, 0.3]),
        },
        /// Blend from rounded plate edges to narrow chipped scales; independent of lichen/lenticel/peel.
        pub plate_edge_shape: f64 = "plateEdgeShape" "-" Bounds::closed(0.0, 1.0) => [Draw] {
            wire: 210,
            check: input(Site::Material, 24, "bark plate edge shape"),
            dial: bounded("material_plate_edge_shape", "the blend from rounded plate edges to \
                narrow chipped scales; at zero every plate edge is rounded, and any rise starts \
                the chipping", [0.15, 0.3]),
        },
        /// How much of its own a plate keeps: how proud it stands, how it leans,
        /// and the value and cast it holds against its neighbours. At zero every
        /// plate stands and reads exactly like its neighbours, and any rise
        /// starts the difference.
        pub plate_identity: f64 = "plateIdentity" "-" Bounds::closed(0.0, 1.0) => [Draw] {
            wire: 211,
            check: input(Site::Material, 25, "bark plate identity"),
            applies: "`plateCellScale` or `ridgeScale` zero",
            dial: bounded("material_plate_identity", "how much of its own a plate keeps against \
                its neighbours; at zero every plate stands and reads exactly like its \
                neighbours, and any rise starts the difference", [0.15, 0.3]),
        },
        /// How far a weathered face is greyed and tinted against a fresh furrow.
        /// At zero no face is weathered at all, and any rise starts the greying.
        pub weathering_strength: f64 = "weatheringStrength" "-" Bounds::closed(0.0, 1.0) => [Draw] {
            wire: 212,
            check: input(Site::Material, 26, "bark weathering strength"),
            dial: bounded("material_weathering_strength", "how far a weathered face is greyed \
                against a fresh furrow; at zero no face is weathered at all, and any rise starts \
                the greying", [0.15, 0.3]),
        },
        /// The tint a weathered face takes, as an offset per channel; how far
        /// it is laid on is `weathering_strength` above.
        pub weathering_red: f64 = "weatheringRed" "offset" Bounds::closed(-1.0, 1.0) => [Draw] {
            wire: 213,
            check: input(Site::Material, 27, "bark weathering red"),
            applies: "`weatheringStrength` zero",
            dial: bounded("material_weathering_red", "the red in the tint a weathered face takes",
                [0.005, 0.01]).span([-0.01125, 0.03375]),
        },
        /// The green channel of `weathering_red`.
        pub weathering_green: f64 = "weatheringGreen" "offset" Bounds::closed(-1.0, 1.0) => [Draw] {
            wire: 214,
            check: input(Site::Material, 28, "bark weathering green"),
            applies: "`weatheringStrength` zero",
            dial: bounded("material_weathering_green", "the green in the tint a weathered face \
                takes", [0.01, 0.02]).span([-0.0175, 0.0525]),
        },
        /// The blue channel of `weathering_red`.
        pub weathering_blue: f64 = "weatheringBlue" "offset" Bounds::closed(-1.0, 1.0) => [Draw] {
            wire: 215,
            check: input(Site::Material, 29, "bark weathering blue"),
            applies: "`weatheringStrength` zero",
            dial: bounded("material_weathering_blue", "the blue in the tint a weathered face \
                takes", [0.01, 0.02]).span([-0.014875, 0.044625]),
        },
        /// How far the side away from the sun and the foot of the trunk take a
        /// colour of their own - what damp growth would look like, not what it is.
        /// At zero the shaded side and the foot take no colour of their own, and
        /// any rise starts it.
        pub orientation_strength: f64 = "orientationStrength" "-"
            Bounds::closed(0.0, 1.0) => [Draw] {
            wire: 216,
            check: input(Site::Material, 30, "bark orientation strength"),
            dial: bounded("material_orientation_strength", "how far the shaded side and the foot \
                of the trunk take a colour of their own; at zero the shaded side and the foot \
                take no colour of their own, and any rise starts it", [0.15, 0.3]),
        },
        /// The tint that damp side takes, as an offset per channel; how far it
        /// is laid on is `orientation_strength` above.
        pub orientation_red: f64 = "orientationRed" "offset" Bounds::closed(-1.0, 1.0) => [Draw] {
            wire: 217,
            check: input(Site::Material, 31, "bark orientation red"),
            applies: "`orientationStrength` zero",
            dial: bounded("material_orientation_red", "the red in the tint the shaded side and \
                the foot of the trunk take", [0.02, 0.04]).span([-0.093, 0.031]),
        },
        /// The green channel of `orientation_red`.
        pub orientation_green: f64 = "orientationGreen" "offset"
            Bounds::closed(-1.0, 1.0) => [Draw] {
            wire: 218,
            check: input(Site::Material, 32, "bark orientation green"),
            applies: "`orientationStrength` zero",
            dial: bounded("material_orientation_green", "the green in the tint the shaded side \
                and the foot of the trunk take", [0.01, 0.02]).span([-0.013, 0.039]),
        },
        /// The blue channel of `orientation_red`.
        pub orientation_blue: f64 = "orientationBlue" "offset" Bounds::closed(-1.0, 1.0) => [Draw] {
            wire: 219,
            check: input(Site::Material, 33, "bark orientation blue"),
            applies: "`orientationStrength` zero",
            dial: bounded("material_orientation_blue", "the blue in the tint the shaded side and \
                the foot of the trunk take", [0.05, 0.1]).span([-0.23025, 0.07675]),
        },
        /// How far a furrow floor is darkened by its own crest standing between it
        /// and the sun. Zero leaves the sun on both sides of every furrow alike.
        pub directional_occlusion: f64 = "directionalOcclusion" "-"
            Bounds::closed(0.0, 1.0) => [Draw] {
            wire: 220,
            check: input(Site::Material, 34, "bark directional occlusion"),
            dial: bounded("material_directional_occlusion", "how far a furrow floor is darkened \
                by its own crest; at zero the sun falls on both sides of every furrow alike, and \
                any rise starts the shading", [0.15, 0.3]),
        },
        /// How far the relief is given depth beyond the shaded normal. At zero
        /// the relief is given no depth beyond that normal, and any rise starts
        /// it.
        pub depth_strength: f64 = "depthStrength" "-" Bounds::closed(0.0, 1.0) => [Draw] {
            wire: 221,
            check: input(Site::Material, 35, "bark depth strength"),
            dial: bounded("material_depth_strength", "how far the bark's relief is given depth \
                beyond the shaded normal; at zero the relief is given no depth beyond the shaded \
                normal, and any rise starts it", [0.15, 0.3]),
        },
        /// How far a leaf is lit as part of its crown rather than as a lone card:
        /// its lighting normal bends from the blade's toward the crown's outward
        /// direction at its placement, so the sunward shell of the mass is lit
        /// whichever way its blades turn. Zero lights the blade alone.
        pub canopy_normal: f64 = "canopyNormal" "-" Bounds::closed(0.0, 1.0) => [Draw] {
            wire: 222,
            check: input(Site::Material, 36, "leaf canopy normal"),
            dial: bounded("material_canopy_normal", "how far a leaf is lit as part of its crown \
                rather than as a lone card", [0.15, 0.3]),
        },
        /// How far the leaf's sunlight wraps past the terminator, as a fraction
        /// of a right angle's cosine; a face square to the sun takes what it
        /// always took. Zero is the plain cosine.
        pub light_wrap: f64 = "lightWrap" "-" Bounds::closed(0.0, 1.0) => [Draw] {
            wire: 223,
            check: input(Site::Material, 37, "leaf light wrap"),
            dial: bounded("material_light_wrap", "how far a leaf's sunlight wraps past the \
                terminator", [0.15, 0.3]),
        },
        /// The share of the blade's transmission that leaves it diffusely, as a
        /// thin leaf's does, rather than on the forward lobe toward the sun; the
        /// diffuse share also carries the sky through the blade. Zero is the lobe.
        pub diffuse_transmission: f64 = "diffuseTransmission" "-"
            Bounds::closed(0.0, 1.0) => [Draw] {
            wire: 224,
            check: input(Site::Material, 38, "leaf diffuse transmission"),
            dial: bounded("material_diffuse_transmission", "the share of what comes through a \
                blade that leaves it diffusely", [0.15, 0.3]),
        },
        /// The cuticle's reflectance of the sky at normal incidence, rising to
        /// the whole sky at grazing by Schlick's Fresnel; zero reflects no sky.
        pub leaf_sheen: f64 = "leafSheen" "-" Bounds::closed(0.0, 0.5) => [Draw] {
            wire: 225,
            check: input(Site::Material, 39, "leaf sheen"),
            dial: bounded("material_leaf_sheen", "what the leaf's cuticle reflects of the sky \
                head-on", [0.1, 0.2]),
        },
        /// How much of the sky one crown radius of leaves takes from a leaf that
        /// reads it through the mass - the sky over it, behind it and in its
        /// sheen - so the underside of a crown falls into its own shade. Zero
        /// sees the sky through the mass.
        pub crown_shade: f64 = "crownShade" "-" Bounds::closed(0.0, 1.0) => [Draw] {
            wire: 226,
            check: input(Site::Material, 40, "leaf crown shade"),
            dial: bounded("material_crown_shade", "how much of the sky one crown radius of \
                leaves takes from a leaf", [0.15, 0.3]),
        },
        /// Smooth bark's lichen: the size in metres of the cells its patches are
        /// scattered over. Zero leaves no patch anywhere.
        pub lichen_scale: f64 = "lichenScale" "m" Bounds::closed(0.0, 1.0) => [Draw] {
            wire: 227,
            check: input(Site::Material, 41, "bark lichen scale"),
            applies: "`lichenStrength` zero",
            dial: bounded("material_lichen_scale", "the metres across the cells lichen patches \
                are scattered over", [0.01, 0.02]).span([0.0, 0.06]),
        },
        /// The share of those cells that hold a patch.
        pub lichen_coverage: f64 = "lichenCoverage" "-" Bounds::closed(0.0, 1.0) => [Draw] {
            wire: 228,
            check: input(Site::Material, 42, "bark lichen coverage"),
            applies: "`lichenStrength` or `lichenScale` zero",
            dial: bounded("material_lichen_coverage", "the share of those cells that hold a \
                patch", [0.15, 0.3]),
        },
        /// A patch's own colour, a linear reflectance like the bark's.
        pub lichen_red: f64 = "lichenRed" "reflectance" Bounds::closed(0.0, 1.0) => [Draw] {
            wire: 229,
            check: input(Site::Material, 43, "bark lichen red"),
            applies: "`lichenStrength` or `lichenScale` zero",
            dial: bounded("material_lichen_red", "the red in a lichen patch's own colour",
                [0.15, 0.3]),
        },
        /// The green channel of `lichen_red`.
        pub lichen_green: f64 = "lichenGreen" "reflectance" Bounds::closed(0.0, 1.0) => [Draw] {
            wire: 230,
            check: input(Site::Material, 44, "bark lichen green"),
            applies: "`lichenStrength` or `lichenScale` zero",
            dial: bounded("material_lichen_green", "the green in a lichen patch's own colour",
                [0.15, 0.3]),
        },
        /// The blue channel of `lichen_red`.
        pub lichen_blue: f64 = "lichenBlue" "reflectance" Bounds::closed(0.0, 1.0) => [Draw] {
            wire: 231,
            check: input(Site::Material, 45, "bark lichen blue"),
            applies: "`lichenStrength` or `lichenScale` zero",
            dial: bounded("material_lichen_blue", "the blue in a lichen patch's own colour",
                [0.15, 0.3]),
        },
        /// How far a patch covers the bark with that colour.
        pub lichen_strength: f64 = "lichenStrength" "-" Bounds::closed(0.0, 1.0) => [Draw] {
            wire: 232,
            check: input(Site::Material, 46, "bark lichen strength"),
            applies: "`lichenScale` zero",
            note: "Above zero, with `lichenScale`, it compiles the smooth-bark wood pipeline \
                (`telperion-render` `wood.rs`).",
            dial: bounded("material_lichen_strength", "how far a patch covers the bark with its \
                colour", [0.15, 0.3]),
        },
        /// Rows of lenticel dashes per metre along the wood.
        pub lenticel_density: f64 = "lenticelDensity" "-" Bounds::closed(0.0, 400.0) => [Draw] {
            wire: 233,
            check: input(Site::Material, 47, "bark lenticel density"),
            applies: "`lenticelStrength` or `lenticelLength` zero",
            dial: bounded("material_lenticel_density", "rows of lenticel dashes per metre along \
                the wood", [5.0, 10.0]).span([0.0, 27.0]),
        },
        /// The longest dash across the wood, in metres; the shortest is under half.
        pub lenticel_length: f64 = "lenticelLength" "m" Bounds::closed(0.0, 0.5) => [Draw] {
            wire: 234,
            check: input(Site::Material, 48, "bark lenticel length"),
            applies: "`lenticelStrength` zero",
            dial: bounded("material_lenticel_length", "the longest lenticel dash across the \
                wood, in metres", [0.1, 0.2]),
        },
        /// How far a dash shows: its tint, and the shallow groove it cuts.
        pub lenticel_strength: f64 = "lenticelStrength" "-" Bounds::closed(0.0, 1.0) => [Draw] {
            wire: 235,
            check: input(Site::Material, 49, "bark lenticel strength"),
            applies: "`lenticelLength` zero",
            note: "Above zero, with `lenticelLength`, it compiles the smooth-bark wood pipeline \
                (`telperion-render` `wood.rs`).",
            dial: bounded("material_lenticel_strength", "how far a lenticel dash shows",
                [0.15, 0.3]),
        },
        /// A dash's value against the bark it marks: -1 is black, 0 no change.
        pub lenticel_tint: f64 = "lenticelTint" "-" Bounds::closed(-1.0, 1.0) => [Draw] {
            wire: 236,
            check: input(Site::Material, 50, "bark lenticel tint"),
            applies: "`lenticelStrength` or `lenticelLength` zero",
            dial: bounded("material_lenticel_tint", "a lenticel dash's value against the bark it \
                marks", [0.25, 0.5]),
        },
        /// How far the plate network's strips curl away: they stretch across the
        /// wood into bands, lift at their lower edge, and this share of them has
        /// peeled off to show the inner bark. At zero no strip has peeled at all,
        /// and any rise switches the peel on.
        pub peel_curl: f64 = "peelCurl" "-" Bounds::closed(0.0, 1.0) => [Draw] {
            wire: 237,
            check: input(Site::Material, 51, "bark peel curl"),
            applies: "`ridgeScale` zero",
            note: "Above zero it compiles the smooth-bark wood pipeline (`telperion-render` \
                `wood.rs`).",
            dial: bounded("material_peel_curl", "the share of the bark's strips that have peeled \
                away; at zero no strip has peeled at all, and any rise switches the peel on",
                [0.15, 0.3]),
        },
        /// The inner bark a peeled strip leaves showing.
        pub peel_red: f64 = "peelRed" "reflectance" Bounds::closed(0.0, 1.0) => [Draw] {
            wire: 238,
            check: input(Site::Material, 52, "bark peel red"),
            applies: "`peelCurl` or `ridgeScale` zero",
            dial: bounded("material_peel_red", "the red in the inner bark a peeled strip shows",
                [0.15, 0.3]),
        },
        /// The green channel of `peel_red`.
        pub peel_green: f64 = "peelGreen" "reflectance" Bounds::closed(0.0, 1.0) => [Draw] {
            wire: 239,
            check: input(Site::Material, 53, "bark peel green"),
            applies: "`peelCurl` or `ridgeScale` zero",
            dial: bounded("material_peel_green", "the green in the inner bark a peeled strip \
                shows", [0.15, 0.3]),
        },
        /// The blue channel of `peel_red`.
        pub peel_blue: f64 = "peelBlue" "reflectance" Bounds::closed(0.0, 1.0) => [Draw] {
            wire: 240,
            check: input(Site::Material, 54, "bark peel blue"),
            applies: "`peelCurl` or `ridgeScale` zero",
            dial: bounded("material_peel_blue", "the blue in the inner bark a peeled strip shows",
                [0.15, 0.3]),
        },
        /// How much of the sky and of what passes through the blade a leaf loses
        /// to the leaves of its own lobe standing over it, read from the crown's
        /// own placements rather than from one smooth ellipsoid: a lobe's face is
        /// lit and what hangs under it falls into its shade. Zero sees none of it.
        pub lobe_shade: f64 = "lobeShade" "-" Bounds::closed(0.0, 1.0) => [Draw] {
            wire: 241,
            check: input(Site::Material, 55, "leaf lobe shade"),
            dial: bounded("material_lobe_shade", "how much a leaf loses to the leaves of its own \
                lobe standing over it", [0.15, 0.3]),
        },
        /// What the bark mirrors of the sun at normal incidence, the foot of its
        /// one highlight by Schlick's Fresnel: 0.04 is a dielectric. What the
        /// highlight mirrors is taken from the diffuse; zero mirrors none.
        pub bark_reflectance: f64 = "barkReflectance" "-" Bounds::closed(0.0, 1.0) => [Draw] {
            wire: 242,
            check: input(Site::Material, 56, "bark reflectance"),
            dial: bounded("material_bark_reflectance", "what the bark mirrors of the sun head-on",
                [0.15, 0.3]),
        },
        /// The same for the cuticle on the blade's front face; the highlight's
        /// width follows `cuticle_gloss`.
        pub leaf_reflectance: f64 = "leafReflectance" "-" Bounds::closed(0.0, 1.0) => [Draw] {
            wire: 243,
            check: input(Site::Material, 57, "leaf reflectance"),
            dial: bounded("material_leaf_reflectance", "what a leaf's cuticle mirrors of the sun \
                head-on", [0.15, 0.3]),
        },
        /// A grain below the relief: the size in metres of its cells. Zero
        /// leaves the bark smooth between the relief's features.
        pub bark_grain_scale: f64 = "barkGrainScale" "m" Bounds::closed(0.0, 0.05) => [Draw] {
            wire: 244,
            check: input(Site::Material, 58, "bark grain scale"),
            applies: "`barkGrainStrength` zero",
            dial: bounded("material_bark_grain_scale", "the metres across the cells of the grain \
                below the bark's relief", [0.0005, 0.001]).span([0.0, 0.003]),
        },
        /// How far the grain varies the bark's colour and tilts its normal.
        pub bark_grain_strength: f64 = "barkGrainStrength" "-" Bounds::closed(0.0, 1.0) => [Draw] {
            wire: 245,
            check: input(Site::Material, 59, "bark grain strength"),
            applies: "`barkGrainScale` zero",
            dial: bounded("material_bark_grain_strength", "how far that grain varies the bark's \
                colour and tilts its normal", [0.15, 0.3]),
        },
        /// The blade's cell grain, in cells per blade length; zero is none.
        pub blade_grain_scale: f64 = "bladeGrainScale" "-" Bounds::closed(0.0, 256.0) => [Draw] {
            wire: 246,
            check: input(Site::Material, 60, "leaf blade grain scale"),
            applies: "`bladeGrainStrength` zero",
            dial: bounded("material_blade_grain_scale", "the blade's cell grain, in cells per \
                blade length", [20.0, 40.0]).span([0.0, 135.0]),
        },
        /// How far that grain varies the blade's colour and tilts its normal.
        pub blade_grain_strength: f64 = "bladeGrainStrength" "-"
            Bounds::closed(0.0, 1.0) => [Draw] {
            wire: 247,
            check: input(Site::Material, 61, "leaf blade grain strength"),
            applies: "`bladeGrainScale` zero",
            dial: bounded("material_blade_grain_strength", "how far that grain varies the \
                blade's colour and tilts its normal", [0.15, 0.3]),
        },
    }
}
impl Default for MaterialParams {
    /// A mid-brown bark under a mid-green blade, paler underneath, varying a
    /// little. Neither a species nor a look: the row every family starts from.
    fn default() -> Self {
        Self {
            bark_red: 0.147,
            bark_green: 0.105,
            bark_blue: 0.068,
            bark_roughness: 0.8,
            // Young wood the colour of the bark, and none of it young.
            shoot_red: 0.147,
            shoot_green: 0.105,
            shoot_blue: 0.068,
            shoot_radius: 0.0,
            leaf_front_red: 0.068,
            leaf_front_green: 0.195,
            leaf_front_blue: 0.036,
            leaf_back_red: 0.105,
            leaf_back_green: 0.240,
            leaf_back_blue: 0.070,
            // A dry grey-brown, drawn only where a canopy keeps a skirt.
            leaf_dead_red: 0.30,
            leaf_dead_green: 0.25,
            leaf_dead_blue: 0.18,
            hue_range_low: -0.03,
            hue_range_high: 0.03,
            brightness_range_low: -0.12,
            brightness_range_high: 0.12,
            interior_darkening: 0.5,
            ridge_scale: 0.0,
            plate_scale: 0.0,
            furrow_strength: 1.0,
            roughness_detail: 0.0,
            vein_scale: 8.0,
            vein_contrast: 0.0,
            transmission_strength: 0.0,
            transmission_red: 0.3,
            transmission_green: 0.6,
            transmission_blue: 0.1,
            thickness: 1.0,
            fissure_red: 0.0,
            fissure_green: 0.0,
            fissure_blue: 0.0,
            fissure_strength: 0.0,
            crest_red: 0.0,
            crest_green: 0.0,
            crest_blue: 0.0,
            crest_strength: 0.0,
            bark_mottle_scale: 0.0,
            bark_mottle_strength: 0.0,
            cavity_strength: 0.0,
            blade_mottle_scale: 0.0,
            blade_mottle_strength: 0.0,
            margin_width: 0.0,
            margin_red: 0.0,
            margin_green: 0.0,
            margin_blue: 0.0,
            cuticle_gloss: 0.0,
            sky_occlusion_strength: 0.0,
            plate_cell_scale: 0.0,
            plate_elongation: 0.0,
            plate_dome: 0.0,
            plate_edge_lift: 0.0,
            plate_furrow_width: 0.0,
            plate_edge_shape: 0.0,
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
            canopy_normal: 0.0,
            light_wrap: 0.0,
            diffuse_transmission: 0.0,
            leaf_sheen: 0.0,
            crown_shade: 0.0,
            lichen_scale: 0.0,
            lichen_coverage: 0.0,
            lichen_red: 0.0,
            lichen_green: 0.0,
            lichen_blue: 0.0,
            lichen_strength: 0.0,
            lenticel_density: 0.0,
            lenticel_length: 0.0,
            lenticel_strength: 0.0,
            lenticel_tint: 0.0,
            peel_curl: 0.0,
            peel_red: 0.0,
            peel_green: 0.0,
            peel_blue: 0.0,
            lobe_shade: 0.0,
            bark_reflectance: 0.04,
            leaf_reflectance: 0.04,
            bark_grain_scale: 0.0,
            bark_grain_strength: 0.0,
            blade_grain_scale: 0.0,
            blade_grain_strength: 0.0,
        }
    }
}
