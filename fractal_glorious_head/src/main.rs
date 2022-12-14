use rusty_fractals_core::fractal::{FractalConfig, FractalDefinition, Math, MathPhoenix};
use rusty_fractals_core::mem::Mem;
use rusty_fractals_core::mem_phoenix::MemPhoenix;
use rusty_fractals_domain::resolution_multiplier;
use rusty_fractals_domain::resolution_multiplier::ResolutionMultiplier::Square5;
use resolution_multiplier::ResolutionMultiplier;
use rusty_fractals_result::palettes::palette_blue_to_white;

use log::{info};
const NAME: &str = "Glorious Head";
const PHOENIX_INIT_C: f64 = 0.35;
const PHOENIX_INIT_P: f64 = -0.25;
const PHOENIX_INIT_PHOENIX_INITIALIZER: f64 = 1.0;
const ITERATION_MAX: u32 = 2500;
const ITERATION_MIN: u32 = 8;
const AREA_SIZE: f64 = 4.5;
const TARGET_RE: f64 = -0.16884290496519;
const TARGET_IM: f64 = -0.37573460559804;
const RESOLUTION_WIDTH: u32 = 1280;
const RESOLUTION_HEIGHT: u32 = 720;
const RESOLUTION_MULTIPLIER: ResolutionMultiplier = Square5;
const REPEAT: bool = true;
const SAVE_IMAGES: bool = false;

struct GloriousHead {
    pub name: String,
}

impl MathPhoenix for GloriousHead {
    fn math(&self, mp: &mut MemPhoenix, origin_re: f64, origin_im: f64) {
        mp.m.square();

        mp.m.re += mp.c;
        mp.m.re += mp.p * mp.prev_prev_re;
        mp.m.im += mp.p * mp.prev_prev_im;

        // previous iteration
        mp.prev_prev_re = mp.prev_re;
        mp.prev_prev_im = mp.prev_im;
        mp.prev_re = mp.m.re;
        mp.prev_im = mp.m.im;

        mp.m.plus(origin_re, origin_im);
    }
}

fn main() {
    info!("Started");


    let glorious_head = GloriousHead { name: NAME.to_string() };
    let definition = FractalDefinition { iteration_min: ITERATION_MIN, iteration_max: ITERATION_MAX, area_size: AREA_SIZE, target_re: TARGET_RE, target_im: TARGET_IM };
    let config = FractalConfig { resolution_width: RESOLUTION_WIDTH, resolution_height: RESOLUTION_HEIGHT, resolution_multiplier: RESOLUTION_MULTIPLIER, repeat: REPEAT, save_images: SAVE_IMAGES, palette: PALETTE };

    info!("Fractal {}", glorious_head.name);

    let m = Mem { re: 0.0, im: 0.0 };
    let mut mp = MemPhoenix { m, c: PHOENIX_INIT_C, p: PHOENIX_INIT_P, prev_prev_re: PHOENIX_INIT_PHOENIX_INITIALIZER, prev_prev_im: PHOENIX_INIT_PHOENIX_INITIALIZER, prev_re: PHOENIX_INIT_PHOENIX_INITIALIZER, prev_im: PHOENIX_INIT_PHOENIX_INITIALIZER };

    glorious_head.math(&mut mp, 1.0, 0.1);

    info!("Finished.");
}

#[test]
fn test_math() {
    let glorious_head = GloriousHead { name: NAME.to_string() };
    let m = Mem { re: 0.0, im: 0.0 };
    let mut mp = MemPhoenix { m, c: PHOENIX_INIT_C, p: PHOENIX_INIT_P, prev_prev_re: PHOENIX_INIT_PHOENIX_INITIALIZER, prev_prev_im: PHOENIX_INIT_PHOENIX_INITIALIZER, prev_re: PHOENIX_INIT_PHOENIX_INITIALIZER, prev_im: PHOENIX_INIT_PHOENIX_INITIALIZER };

    glorious_head.math(&mut mp, 1.0, 0.1);

    assert_eq!(mp.re(), 1.1);
    assert_eq!(mp.im(), -0.15);
}
