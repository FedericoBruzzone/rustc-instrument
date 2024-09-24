fn main() {
    env_logger::init();
    rustc_instrument::driver_main(rustc_ex::RustcEx);
}
