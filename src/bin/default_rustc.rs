//! the origin rustc executable

#![feature(rustc_private)]
extern crate rustc_driver;

struct DefaultCallback;

impl rustc_driver::Callbacks for DefaultCallback {}

fn main() {
    // rustc_driver::init_rustc_env_logger();
    rustc_driver::RunCompiler::new(&std::env::args().collect::<Vec<_>>(), &mut DefaultCallback)
        .run()
        .unwrap();
}
