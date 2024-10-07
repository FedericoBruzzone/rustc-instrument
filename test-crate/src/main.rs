#[cfg(feature = "test")]
fn ciao() {}

fn main() {
    ciao()
}

// mod test_file;
// mod test_file1;
//
// #[cfg(feature = "xxx")]
// fn xxx() {}
//
// #[cfg(feature = "yyy")]
// fn xxx() {}
//
// fn yyy() {
//     if cfg!(feature = "yyy") {}
// }
//
// fn main() {
//     // env_logger::init();
//     test_file::test_func();
//     xxx();
//     yyy();
// }

// #[cfg(feature = "p1")]
// fn pippo() -> i32 {
//     3
// }
//
// #[cfg(feature = "p2")]
// fn pippo() -> String {
//     "ciao".to_string()
// }
//
// struct C;
//
// fn test_c1(c: &mut C) {}
// fn test_c2(c: &C) {}
// fn test_c3(c: C) {}
//
// fn pippo_call(i: i32) {}
//
// fn main() {
//
//     let mut x = pippo();
//     println!("{}", x);
//
//     pippo_call(x);
//
//     // x = 10;
//     // println!("{}", x + 1)
//
//     let mut x = 1;
//     if cfg!(feature = "f1") {
//         println!("F1 is active");
//     }
//     if cfg!(feature = "f2") {
//         println!("F2 is active");
//         if cfg!(feature = "f3") {
//             println!("F3 is active");
//             x = true_main2();
//         }
//         if cfg!(feature = "f5") {
//             x = true_main();
//         }
//
//         if cfg!(feature = "f4") {
//             println!("F3 is active");
//             println!("{}", x)
//         }
//     }
// }
//
// fn true_main() -> i32 {
//     1
// }
//
// fn true_main2() -> i32 {
//     1
// }
