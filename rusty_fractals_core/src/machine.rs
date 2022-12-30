use rayon::iter::{IntoParallelIterator, IntoParallelRefIterator, ParallelIterator};
use rusty_fractals_result::fractal_result::ResultData;
use rusty_fractals_result::result_data::ResultData;
use rusty_fractals_result::result_pixels::ResultPixels;
use rusty_fractals_domain::domain;
use rusty_fractals_common::area::Area;
use rusty_fractals_common::constants::CALCULATION_BOUNDARY;
use rusty_fractals_domain::domain::Domain;
use rusty_fractals_domain::domain_element::DomainElement;
use crate::{fractal, fractal_path};
use crate::fractal::{CALCULATION_BOUNDARY, CalculationConfig, Fractal, Math, ResultConfig};
use crate::mem::Mem;

// to calculate single image
pub struct Machine {
    pub area: Area,
    pub domain: Domain,
    pub calculation_config: CalculationConfig,
    pub result_config: ResultConfig,
}

impl Machine {
    pub fn calculate(&mut self, fractal_math: &impl Math<T>) {
        let coordinates_xy = self.domain.shuffled_calculation_coordinates();

        let mut result_data = ResultData {
            paths: Vec::new(),
            area_result: area,
        };

        // Calculate independently and in parallel each domain chunks
        coordinates_xy.into_par_iter().for_each(
            |xy| self.chunk_calculation(xy, fractal_math, &mut result_data)
        );

        let mut result_pixels = ResultPixels {
            width: self.area.width_x,
            height: self.area.height_y,
            pixels: vec![]
        };

        result_data.translate_paths_to_pixel_grid(&result_pixels);

        self.domain.mask_full_update();

        fractal.perfectly_color_values();
        Application.repaint_mandelbrot_window();
    }

    // in sequence (cpu_num) executes as 20x20 parallel for each domain chunk
    pub fn chunk_calculation(&mut self, xy: [u32; 2], fractal_math: &impl Math<T>, result: &mut ResultData) {
        let chunk_size_x = self.domain.width / 20;
        let chunk_size_y = self.domain.height / 20;

        let wrapped_chunk = self.domain.make_chunk(
            (xy[0] * chunk_size_x) as usize, ((xy[0] + 1) * chunk_size_x) as usize,
            (xy[1] * chunk_size_y) as usize, ((xy[1] + 1) * chunk_size_y) as usize,
        );
        for el in wrapped_chunk {
            self.calculate_path_finite(&el, fractal_math, result);
        }
    }

    pub fn calculate_path_finite(&mut self, el: &DomainElement, fractal_math: &impl Math<Mem>, result: &mut ResultData) {
        let max = self.calculation_config.iteration_max;
        let min = self.calculation_config.iteration_min;
        let mut iterator = 0;
        let mut length = 0;
        let m = Mem::new(el.origin_re, el.origin_im);
        while m.quadrance() < CALCULATION_BOUNDARY && iterator < max {

            // Investigate if this is a good calculation path
            // Don't create path data yet. Too many origins don't produce good data
            // Most of the long and expensive calculations end up inside Mandelbrot set, useless
            // It is 1.68x faster to calculate path twice, and recording exclusively the good paths

            fractal_math.math(m, el.originRe, el.originIm);
            if AreaFinebrot.contains(m) {
                length += 1;
            }
            iterator += 1;
        }
        el.setFinishedState(iterator, length);

        if length > min && iterator < max {

            // This origin produced good data, record calculation path

            m.reset(el.originRe, el.originIm);
            el.goodPath();

            let mut path: Vec<[f64; 2]> = Vec::new();
            for _ in 0..iterator {
                fractal_math.math(m, el.originRe, el.originIm);
                if AreaFinebrot.contains(m) {
                    path.push([m.re, m.im]);
                }
            }
            result.add_escape_path_long(path);
            // stats.paths_new_points_amount += path.size();
        }
    }
}
