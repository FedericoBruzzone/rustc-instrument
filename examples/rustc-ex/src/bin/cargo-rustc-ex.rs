fn main() {
    env_logger::init();
    rustc_instrument::cli_main(rustc_ex::RustcEx);
}

