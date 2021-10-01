#![feature(rustc_private)]
extern crate rustc_driver;

struct Cb;

impl rustc_driver::Callbacks for Cb {}

fn main() {
    rustc_driver::init_rustc_env_logger();
    rustc_driver::RunCompiler::new(&std::env::args().collect::<Vec<_>>(), &mut Cb)
        .run()
        .unwrap();
}
