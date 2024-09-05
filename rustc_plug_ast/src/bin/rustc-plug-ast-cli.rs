fn main() {
  env_logger::init();
  rustc_plugin::cli_main(rustc_plug_ast::PrintAst);
}
