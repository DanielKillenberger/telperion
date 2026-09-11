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

    // A tolerance rather than an equality, and it is spent where the samples
    // land. Multisampling resolves the silhouette of every twig and leaf and
    // touches nothing that is not a silhouette, so the room is judged in two
    // halves: the paint has to be the paint fn-24 drew, and the picture as a
    // whole only has to stay near it.
    let channels = pinned.len() as f64;
    let sum: u64 = drawn
        .rgba
        .iter()
        .zip(&pinned)
        .map(|(a, b)| u64::from(a.abs_diff(*b)))
        .sum();
    let moved = drawn
        .rgba
        .iter()
        .zip(&pinned)
        .filter(|(a, b)| a != b)
        .count() as f64;
    let flat = flat_drift(&drawn.rgba, &pinned, PINNED_SIZE);

    // The flat picture: every colour channel the pinned still paints the same
    // as its neighbours, where no sample of an edge ever lands. A material, a
    // sun or a tone map leaking into the one view that exists to have none of
    // them would move exactly this, and antialiasing cannot.
    assert!(
        flat.mean() < 0.1,
        "the clay room's paint drifted: mean channel error {} over {} flat channels",
        flat.mean(),
        flat.channels
    );
    assert!(
        flat.far_share() < 0.002,
        "{:.4}% of the room's flat channels moved more than four",
        flat.far_share() * 100.0
    );
    // And the silhouette, which may move and only so far: an edge is a small
    // part of a picture however finely a tree is drawn.
    assert!(
        sum as f64 / channels < 2.0,
        "the clay room drifted: mean channel error {}",
        sum as f64 / channels
    );
    assert!(
        moved / channels < 0.08,
        "{:.2}% of channels moved: more of the room than its edges changed",
        moved / channels * 100.0
    );
}

/// How far the flat paint of a pinned still moved. Flat is the pinned still's
/// own word: a colour channel whose eight neighbours are all within two of it,
/// which is paint rather than a silhouette.
struct Drift {
    channels: u64,
    sum: u64,
    far: u64,
}

impl Drift {
    fn mean(&self) -> f64 {
        self.sum as f64 / self.channels as f64
    }

    fn far_share(&self) -> f64 {
        self.far as f64 / self.channels as f64
    }
}

fn flat_drift(drawn: &[u8], pinned: &[u8], (width, height): (u32, u32)) -> Drift {
    let (w, h) = (width as usize, height as usize);
    let channel = |pixels: &[u8], x: usize, y: usize, c: usize| pixels[(y * w + x) * 4 + c];
    let mut drift = Drift {
        channels: 0,
        sum: 0,
        far: 0,
    };
    for y in 1..h - 1 {
        for x in 1..w - 1 {
            for c in 0..3 {
                let here = i32::from(channel(pinned, x, y, c));
                let flat = (-1..=1).all(|dy: i32| {
                    (-1..=1).all(|dx: i32| {
                        let (nx, ny) = ((x as i32 + dx) as usize, (y as i32 + dy) as usize);
                        (i32::from(channel(pinned, nx, ny, c)) - here).abs() <= 2
                    })
                });
                if !flat {
                    continue;
                }
                let moved = u64::from(channel(drawn, x, y, c).abs_diff(channel(pinned, x, y, c)));
                drift.channels += 1;
                drift.sum += moved;
                drift.far += u64::from(moved > 4);
            }
        }
    }
    assert!(
        drift.channels > (w * h * 3) as u64 / 2,
        "only {} of {} channels of the pinned still are flat paint: the mask is wrong",
        drift.channels,
        w * h * 3
    );
    drift
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
    // Two rather than four: a leaf of the crown is a fraction of a pixel
    // across at this size, and multisampling resolves its own offset together
    // with whatever it stands in front of. Both frames are drawn the same way
    // from the same seed, so every difference left is a leaf's own and none of
    // it is noise.
    let moved = open
        .rgba
        .iter()
        .zip(&closed.rgba)
        .filter(|(a, b)| a.abs_diff(**b) > 2)
        .count();
    assert!(
        moved > open.rgba.len() / 500,
        "only {moved} channels moved when the row's ranges opened: no leaf is \
         taking an offset from them"
    );

    // The offset is the leaf's own identity and not the draw's, so one tree
    // under one row is one picture however often it is asked for. Near enough
    // to identical rather than identical: the crown's placements come out of
    // the selection pass in whatever order its atomics gave them, and where
    // two leaves tie on depth, one sample of four may land either way. What a
    // per-draw offset would look like is the comparison above, which moved
    // thousands of channels; this moves a handful of edge pixels by a few.
    let again = stage.draw();
    let (redrawn, worst) = closed
        .rgba
        .iter()
        .zip(&again.rgba)
        .map(|(a, b)| u32::from(a.abs_diff(*b)))
        .fold((0u64, 0u32), |(count, worst), d| {
            (count + u64::from(d > 0), worst.max(d))
        });
    assert!(
        redrawn * 10_000 < closed.rgba.len() as u64 && worst <= 16,
        "one tree drew two crowns: {redrawn} of {} channels moved, worst {worst}",
        closed.rgba.len()
    );
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
