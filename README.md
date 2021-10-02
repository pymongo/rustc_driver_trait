```rust
// rustc_driver::args::arg_expand_all(at_args)
// https://doc.rust-lang.org/cargo/reference/config.html#buildrustc-wrapper
// The first argument passed to the wrapper is the path to the actual rustc
// we must remove extra arg "rustc" and "-"
// mozilla/sccache is a RUSTC_WRAPPER example to learn
// sccahe 只是个编译器缓存中间件而已，所以 RUSTC_WRAPPER 并不是替换 rustc 只是中间件
// 真正修改 rustc 路径的是 RUSTC 环境变量

/*
clippy 核心代码:
- src/driver.rs: impl rustc_driver::Callbacks for ClippyCallbacks {
- clippy_lints::register_plugins
*/
```