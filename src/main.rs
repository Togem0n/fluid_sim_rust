pub mod fluid_cube_mod;
pub mod lib;

fn main() {
    let instances = wgpu::Instance::new(wgpu::Backends::all());
    for adapter in instances.enumerate_adapters(wgpu::Backends::all()) {
        println!("{:?}", adapter.get_info());
    }
    lib::run();
}