use rayon::iter::{IntoParallelIterator, ParallelIterator};
use rusty_fractals_result::{perfect_color_distribution, result_pixels};
use rusty_fractals_result::result_data::ResultData;
use rusty_fractals_result::result_pixels::ResultPixels;
use rusty_fractals_common::area::Area;
use rusty_fractals_common::constants::CALCULATION_BOUNDARY;
use rusty_fractals_domain::domain::Domain;
use rusty_fractals_domain::domain_element::DomainElement;
use rusty_fractals_domain::pixel_states::DomainElementState;
use crate::fractal::{AppConfig, CalculationConfig, Math, ResultConfig};
use crate::mem::Mem;

// to calculate single image
pub struct Machine<'lif> {
    pub area: &'lif Area,
    pub domain: &'lif Domain<'lif>,
    pub calculation_config: CalculationConfig,
    pub app_config: AppConfig,
    pub result_config: ResultConfig,
}

impl Machine<'_> {
    pub fn calculate(&mut self, fractal_math: &impl Math<Mem>) {
        println!("calculate()");
        let coordinates_xy = self.domain.shuffled_calculation_coordinates();

        let mut result_data = ResultData {
            paths: Vec::new()
        };

        // Calculate independently and in parallel each domain chunks
        // TODO coordinates_xy.into_par_iter().for_each(
        coordinates_xy.iter().for_each(
            |xy| self.chunk_calculation(&xy, fractal_math, &mut result_data)
        );

        let mut result_pixels = result_pixels::init(self.area.width_x, self.area.height_y);

        result_pixels.translate_paths_to_pixel_grid(result_data.paths, &self.area);

        let domain_image = self.domain.domain_element_states_to_image();

        let result_image = perfect_color_distribution::perfectly_color_result_values(&result_pixels, &self.result_config.palette);

        // TODO Application.repaint_mandelbrot_window();
    }

    // in sequence (cpu_num) executes as 20x20 parallel for each domain chunk
    pub fn chunk_calculation(&self, xy: &[u32; 2], fractal_math: &impl Math<Mem>, result: &mut ResultData) {
        let chunk_size_x = (self.domain.width / 20) as u32;
        let chunk_size_y = (self.domain.height / 20) as u32;

        let x_from = (xy[0] * chunk_size_x) as usize;
        let x_to = ((xy[0] + 1) * chunk_size_x) as usize;
        let y_from = (xy[1] * chunk_size_y) as usize;
        let y_to = ((xy[1] + 1) * chunk_size_y) as usize;
        for x in x_from..x_to {
            for y in y_from..y_to {
                let core_element: &DomainElement = self.domain.domain_elements[x]
                    .get(y)
                    .expect("domain_elements problem");
                if core_element.is_active_new() {
                    self.calculate_path_finite(core_element, fractal_math, result);
                }
            }
        }
    }

    pub fn calculate_path_finite(&self, el: &DomainElement, fractal_math: &impl Math<Mem>, result: &mut ResultData) -> DomainElementState {
        let max = self.calculation_config.iteration_max;
        let min = self.calculation_config.iteration_min;
        let cb = CALCULATION_BOUNDARY as f64;
        let mut iterator = 0;
        let mut length = 0;
        let mut m = Mem::new(el.origin_re, el.origin_im);
        while m.quad() < cb && iterator < max {

            // Investigate if this is a good calculation path
            // Don't create path data yet. Too many origins don't produce good data
            // Most of the long and expensive calculations end up inside Mandelbrot set, useless
            // It is 1.68x faster to calculate path twice, and recording exclusively the good paths

            fractal_math.math(&mut m, el.origin_re, el.origin_im);
            if self.area.contains(m.re, m.im) {
                length += 1;
            }
            iterator += 1;
        }
        let el_state = Domain::state_from_path_length(iterator, max, min);

        if length > min && iterator < max {

            // This origin produced good data, record calculation path

            let mut m = Mem::new(el.origin_re, el.origin_im);
            // TODO el.good_path();

            let mut path: Vec<[f64; 2]> = Vec::new();
            for _ in 0..iterator {
                fractal_math.math(&mut m, el.origin_re, el.origin_im);
                if self.area.contains(m.re, m.im) {
                    path.push([m.re, m.im]);
                }
            }
            result.add_calculation_path(path);
            // stats.paths_new_points_amount += path.size();
        }

        el_state
    }
}
