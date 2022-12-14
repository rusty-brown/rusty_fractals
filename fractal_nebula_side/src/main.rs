use rusty_fractals_result::palette::Palette;
use rusty_fractals_core::mem::Mem;
use rusty_fractals_core::fractal::{FractalConfig, FractalDefinition, Math};
use rusty_fractals_domain::resolution_multiplier;
use rusty_fractals_result::palettes::palette_blue_to_white;
use resolution_multiplier::ResolutionMultiplier;
use resolution_multiplier::ResolutionMultiplier::SquareAlter;
use log::{info};

const NAME: &str = "Nebula side";
const ITERATION_MAX: u32 = 14800;
const ITERATION_MIN: u32 = 42;
const AREA_SIZE: f64 = 7.0;
const TARGET_RE: f64 = -0.10675625916322415;
const TARGET_IM: f64 = -0.8914368889277283;
const RESOLUTION_WIDTH: u32 = 1280;
const RESOLUTION_HEIGHT: u32 = 720;
const RESOLUTION_MULTIPLIER: ResolutionMultiplier = SquareAlter;
const REPEAT: bool = true;
const SAVE_IMAGES: bool = false;
const PALETTE: Palette = palette_blue_to_white();

struct NebulaSide {
    pub name: String,
}

impl Math for NebulaSide {
    fn math(&self, m: &mut Mem, origin_re: f64, origin_im: f64) {
        m.square();
        m.plus(origin_re, origin_im);
    }
}

fn main() {
    info!("Started");

    let nebula = NebulaSide { name: NAME.to_string() };
    let definition = FractalDefinition {
        iteration_min: ITERATION_MIN,
        iteration_max: ITERATION_MAX,
        area_size: AREA_SIZE,
        target_re: TARGET_RE,
        target_im: TARGET_IM,
        resolution_width: RESOLUTION_WIDTH,
        resolution_height: RESOLUTION_HEIGHT,
        resolution_multiplier: RESOLUTION_MULTIPLIER,
        repeat: REPEAT,
        save_images: SAVE_IMAGES,
        palette: PALETTE
    };

    info!("Fractal {}", nebula.name);

    let mut m = Mem { re: 0.0, im: 0.0 };
    nebula.math(&mut m, 1.0, 0.1);

    info!("Finished.");
}


#[test]
fn test_math() {
    let nebula = NebulaSide { name: NAME.to_string() };
    let mut m = Mem { re: 0.0, im: 0.0 };
    nebula.math(&mut m, 1.0, 0.1);
    assert_eq!(m.re, 1.0);
    assert_eq!(m.im, 0.1);
}
