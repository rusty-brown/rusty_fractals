pub mod engine;
pub mod machine;
pub mod fractal;
pub mod fractal_stats;
pub mod mathematician;
pub mod mem;
pub mod mem_collatz;
pub mod mem_phoenix;
pub mod fractal_path;

fn lib() {
    let cores: usize = num_cpus::get();
}
