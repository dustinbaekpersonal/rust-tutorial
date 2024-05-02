# Rust Tutorial

Reference: https://doc.rust-lang.org/book/ch01-00-getting-started.html

1. Rust source code file with __.rs__ should be compiled first using `$ rustc main.rs` and then to run the executable file `$ ./main`
2. Cargo is Rust's build system/package manager
    - Create new project using `$ cargo new project_name`
    - Build executable file from cargo project `$ cargo build`
    - Run executable file: `$ cargo run` (most developers use this because it compiles and run the file)
    - To check if it compiles but doesn't produce executable: `$ cargo check`
    - Instead of saving the result of the build in the same directory as our code, Cargo stores it in the target/debug directory.
    - When your project is finally ready for release, you can use cargo build --release to compile it with optimizations. This command will create an executable in target/release instead of target/debug. The optimizations make your Rust code run faster, but turning them on lengthens the time it takes for your program to compile. This is why there are two different profiles: one for development, when you want to rebuild quickly and often, and another for building the final program you’ll give to a user that won’t be rebuilt repeatedly and that will run as fast as possible. If you’re benchmarking your code’s running time, be sure to run cargo build --release and benchmark with the executable in target/release.
