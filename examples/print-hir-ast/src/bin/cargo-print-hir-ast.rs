fn main() {
    env_logger::init();
    rustc_instrument::cli_main(print_hir_ast::PrintAst);
}
