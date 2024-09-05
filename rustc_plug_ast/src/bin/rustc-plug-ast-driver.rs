fn main() {
    env_logger::init();
    rustc_plugin::driver_main(rustc_plug_ast::PrintAst);
}
