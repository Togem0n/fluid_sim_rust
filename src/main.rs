pub mod fluid_cube_mod;
pub mod lib;

fn main() {
    pollster::block_on(lib::run());
}