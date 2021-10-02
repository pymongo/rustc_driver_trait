#![feature(rustc_private)]
extern crate rustc_ast;
extern crate rustc_driver;
extern crate rustc_interface;
extern crate rustc_lint;
extern crate rustc_span;

struct CompilerCallback;
impl rustc_driver::Callbacks for CompilerCallback {
    fn config(&mut self, config: &mut rustc_interface::interface::Config) {
        config.register_lints = Some(Box::new(move |_session, lint_store| {
            lint_store.register_early_pass(|| Box::new(FnNameIsFoo));
        }));
    }
}

struct FnNameIsFoo;
impl FnNameIsFoo {
    const LINT: rustc_lint::Lint = {
        let mut lint = rustc_lint::Lint::default_fields_for_macro();
        lint.name = "fn_name_is_foo";
        lint.default_level = rustc_lint::Level::Warn;
        lint
    };
}

impl rustc_lint::LintPass for FnNameIsFoo {
    fn name(&self) -> &'static str {
        "fn_name_is_foo"
    }
}

impl rustc_lint::EarlyLintPass for FnNameIsFoo {
    fn check_fn(
        &mut self,
        cx: &rustc_lint::EarlyContext<'_>,
        fn_kind: rustc_ast::visit::FnKind<'_>,
        span: rustc_span::Span,
        _: rustc_ast::NodeId,
    ) {
        if let rustc_ast::visit::FnKind::Fn(_, ident, ..) = fn_kind {
            if ident.as_str() == "foo" {
                rustc_lint::LintContext::struct_span_lint(cx, &Self::LINT, span, |diagnostic| {
                    let mut diagnostic = diagnostic.build("foo is a bad name for function");
                    diagnostic.emit();
                });
            }
        }
    }
}

fn main() {
    rustc_driver::RunCompiler::new(&std::env::args().collect::<Vec<_>>(), &mut CompilerCallback).run().unwrap();
}
