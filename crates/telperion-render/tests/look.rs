//! What the frame looks like once the rows are read: the clay room exactly as
//! it always was, a lit frame whose leaves take their colours from the material
//! row, and a sky that deepens with height. Skips with a reason where there is
//! no GPU to ask; it never fails for lack of one.
use telperion_core::{
    material::MaterialParams,
    mesh::{self, Detail},
    presets::Preset,
};
use telperion_render::{
    hero_pose, render, Camera, Gpu, Renderer, Still, View, GROUND_REACH, STILL_FORMAT,
};

mod common;
use common::gpu;

/// The still fn-24 judged the clay room by, and the pose and size it was drawn
/// at: `headless --preset ordinary --seed 7 --size 1600x1000`.
const PINNED: &str = concat!(
    env!("CARGO_MANIFEST_DIR"),
    "/../../.flow/evidence/fn24/ordinary-hero.png"
);
const PINNED_SIZE: (u32, u32) = (1600, 1000);
const SEED: u32 = 7;

/// One tree on one device, framed once, so a test that wants three stills of
/// it pays for the tree and the pose once and for the frames three times.
struct Stage {
    renderer: Renderer,
    camera: Camera,
    size: (u32, u32),
}

impl Stage {
    fn new(gpu: Gpu, view: View, size: (u32, u32)) -> Self {
        let mut family = Preset::Ordinary.parameters();
        family.skeleton.seed = SEED;
        let tree = mesh::build(&family, Detail::Full).expect("the core built the tree");
        let mut renderer = Renderer::new(gpu, STILL_FORMAT);
        renderer.submit(&tree).expect("the tree fits the device");
        renderer.set_material(family.material);
        renderer.set_view(view);
        let bounds = renderer.bounds().expect("a tree is up");
        let aspect = f64::from(size.0) / f64::from(size.1);
        let camera = hero_pose(bounds, aspect, GROUND_REACH);
        Self {
            renderer,
            camera,
            size,
        }
    }

    fn draw(&mut self) -> Still {
        render(&mut self.renderer, &self.camera, self.size.0, self.size.1)
            .expect("the frame was drawn")
    }

    fn under(&mut self, material: MaterialParams) -> Still {
        self.renderer.set_material(material);
        self.draw()
    }
}

fn decode(path: &str) -> (u32, u32, Vec<u8>) {
    let file = std::fs::File::open(path).unwrap_or_else(|error| panic!("{path}: {error}"));
    let mut reader = png::Decoder::new(std::io::BufReader::new(file))
        .read_info()
        .expect("a readable PNG");
    let size = reader.output_buffer_size().expect("a PNG of a stated size");
    let mut pixels = vec![0; size];
    let info = reader.next_frame(&mut pixels).expect("the pixels");
    pixels.truncate(info.buffer_size());
    (info.width, info.height, pixels)
}

#[test]
fn the_clay_view_draws_the_still_the_room_always_drew() {
    let Some(gpu) = gpu() else { return };
    let drawn = Stage::new(gpu, View::Clay, PINNED_SIZE).draw();
    let (width, height, pinned) = decode(PINNED);
    assert_eq!(
        (width, height),
        PINNED_SIZE,
        "the pinned still changed size"
    );
    assert_eq!(drawn.rgba.len(), pinned.len());

    // A tolerance rather than an equality: the room must survive the
    // multisampling that lands on top of it, which moves edge pixels and
    // nothing else. What it may not survive is a material, a sun or a tone map
    // leaking into the one view that exists to have none of them.
    let (sum, far) = drawn
        .rgba
        .iter()
        .zip(&pinned)
        .map(|(a, b)| u64::from(a.abs_diff(*b)))
        .fold((0u64, 0u64), |(sum, far), d| {
            (sum + d, far + u64::from(d > 24))
        });
    let channels = pinned.len() as f64;
    let (mean, share) = (sum as f64 / channels, far as f64 / channels);
    assert!(
        mean < 1.0,
        "the clay room drifted: mean channel error {mean}"
    );
    assert!(
        share < 0.005,
        "{:.3}% of channels are far out",
        share * 100.0
    );
}

#[test]
fn every_leaf_takes_its_own_offset_inside_the_row_s_ranges() {
    let Some(gpu) = gpu() else { return };
    let varied = MaterialParams::default();
    assert!(
        varied.hue_range_high > varied.hue_range_low
            && varied.brightness_range_high > varied.brightness_range_low,
        "the row every family starts from varies nothing"
    );
    // The same crown under a row that lets no leaf differ from its neighbour.
    let flat = MaterialParams {
        hue_range_low: 0.0,
        hue_range_high: 0.0,
        brightness_range_low: 0.0,
        brightness_range_high: 0.0,
        ..varied
    };
    let mut stage = Stage::new(gpu, View::Whole, (400, 300));
    let open = stage.under(varied);
    let closed = stage.under(flat);
    let moved = open
        .rgba
        .iter()
        .zip(&closed.rgba)
        .filter(|(a, b)| a.abs_diff(**b) > 4)
        .count();
    assert!(
        moved > open.rgba.len() / 500,
        "only {moved} channels moved when the row's ranges opened: no leaf is \
         taking an offset from them"
    );

    // The offset is the leaf's own identity and not the draw's, so one tree
    // under one row is one picture however often it is asked for.
    assert_eq!(closed.rgba, stage.draw().rgba, "one tree drew two crowns");
}

#[test]
fn the_lit_frame_has_a_sky_that_deepens_with_height_and_is_not_blown_out() {
    let Some(gpu) = gpu() else { return };
    let size = (400, 300);
    let drawn = Stage::new(gpu, View::Whole, size).draw();
    let pixel = |x: u32, y: u32| {
        let i = ((y * size.0 + x) * 4) as usize;
        [drawn.rgba[i], drawn.rgba[i + 1], drawn.rgba[i + 2]]
    };
    // Down the left edge, clear of the subject, until the ground begins: the
    // sky is the only thing up there that is bluer than it is green.
    let horizon = (0..size.1)
        .find(|&y| pixel(1, y)[1] > pixel(1, y)[2])
        .expect("the ground is somewhere under the sky");
    assert!(horizon > 2, "the sky is a couple of rows of pixels");
    let (top, low) = (pixel(1, 0), pixel(1, horizon - 1));
    let blueness = |p: [u8; 3]| i32::from(p[2]) - i32::from(p[0]);
    assert!(
        blueness(top) > blueness(low),
        "the sky does not deepen with height: {top:?} over {low:?}"
    );

    // The tone map is what stands between a sun of radiance three and a sheet
    // of white paper. A hundredth of the frame at full white is already a lot.
    let white = drawn
        .rgba
        .as_chunks::<4>()
        .0
        .iter()
        .filter(|p| p[0] == 255 && p[1] == 255 && p[2] == 255)
        .count();
    let pixels = (size.0 * size.1) as usize;
    assert!(
        white * 100 < pixels,
        "{white} of {pixels} pixels are pure white: the frame is blown out"
    );
}
