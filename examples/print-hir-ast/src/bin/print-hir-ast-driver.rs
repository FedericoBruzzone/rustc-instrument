fn main() {
    env_logger::init();
    rustc_instrument::driver_main(print_hir_ast::PrintAst);
}