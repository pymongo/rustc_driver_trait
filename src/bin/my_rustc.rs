#![feature(rustc_private)]
extern crate rustc_driver;
extern crate rustc_interface;
extern crate rustc_lint;
extern crate rustc_lint_defs;
extern crate rustc_ast;
extern crate rustc_span;

struct DefaultCallback;

/*
clippy 核心代码:
- src/driver.rs: impl rustc_driver::Callbacks for ClippyCallbacks {
- clippy_lints::register_plugins
*/
impl rustc_driver::Callbacks for DefaultCallback {
    fn config(&mut self, config: &mut rustc_interface::interface::Config) {
        config.register_lints = Some(Box::new(move |_session, lint_store| {
            lint_store.register_early_pass(|| Box::new(LintFnNameIsFoo));
        }));
    }
}

// 
/**
or use rustc_session::declare_lint! macro to define lint
虽然所有 lint 的定义都在 extern crate rustc_lint_defs 但是实现分散在各处，有的实现都不在 compiler 模块内(例如检测死递归的)
*/
struct LintFnNameIsFoo;

impl rustc_lint::LintPass for LintFnNameIsFoo {
    fn name(&self) -> &'static str {
        "my_lint_fn_name_is_foo"
    }
}

impl LintFnNameIsFoo {
    const fn lint() -> rustc_lint_defs::Lint {
        let mut lint = rustc_lint_defs::Lint::default_fields_for_macro();
        lint.name = "my_lint_fn_name_is_foo";
        lint.default_level = rustc_lint::Level::Warn;
        lint
    }
}

const FN_NAME_IS_FOO_LINT: rustc_lint::Lint = LintFnNameIsFoo::lint();

impl rustc_lint::EarlyLintPass for LintFnNameIsFoo {
    fn check_fn(
        &mut self,
        cx: &rustc_lint::EarlyContext<'_>,
        fn_kind: rustc_ast::visit::FnKind<'_>,
        span: rustc_span::Span,
        _: rustc_ast::NodeId,
    ) {
        // Ignore FnKind::Closure
        if let rustc_ast::visit::FnKind::Fn(_, ident, ..) = fn_kind {
            if ident.as_str() == "foo" {
                rustc_lint::LintContext::struct_span_lint(cx, &FN_NAME_IS_FOO_LINT, span, |diagnostic| {
                    let mut diagnostic = diagnostic.build("foo is a bad name for function");
                    // docs_link(&mut diagnostic, lint);
                    diagnostic.emit();
                });
            }
        }
    }
}

/**
```
[w@ww rustc_driver_trait]$ ldd ./target/debug/default_rustc
        linux-vdso.so.1 (0x00007fffa0ffc000)
        librustc_driver-d2cc96ed75437e33.so => not found
        libstd-d6566390077dd5f5.so => not found
        libgcc_s.so.1 => /usr/lib/libgcc_s.so.1 (0x00007f19822d5000)
        libc.so.6 => /usr/lib/libc.so.6 (0x00007f1982109000)
        /lib64/ld-linux-x86-64.so.2 => /usr/lib64/ld-linux-x86-64.so.2 (0x00007f198232d000)
```

LD_LIBRARY_PATH="/home/w/.rustup/toolchains/nightly-x86_64-unknown-linux-gnu/lib" RUSTC="./target/debug/default_rustc" cargo c

LD_LIBRARY_PATH="/home/w/.rustup/toolchains/nightly-x86_64-unknown-linux-gnu/lib/rustlib/x86_64-unknown-linux-gnu/lib" RUSTC="./target/debug/default_rustc" cargo c

*/
fn main() {
    let args = std::env::args().collect::<Vec<_>>(); 
    rustc_driver::init_rustc_env_logger();
    rustc_driver::RunCompiler::new(&args, &mut DefaultCallback)
        .run()
        .unwrap();
}
