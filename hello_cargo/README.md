Following https://doc.rust-lang.org/book/getting-started.html#converting-to-cargo

Build with 
```bash
$ cargo build
   Compiling hello_cargo v0.0.1 (file:///C:/Users/juanrh/git/rust-experiments/hello_cargo)
    Finished debug [unoptimized + debuginfo] target(s) in 0.26 secs

```
and run with
```bash
$ target/debug/hello_cargo.exe
Hello Rust with Cargo!
```
or compile and run with 
```bash
$ cargo run
    Finished debug [unoptimized + debuginfo] target(s) in 0.0 secs
     Running `target\debug\hello_cargo.exe`
Hello Rust with Cargo!
```

Compile and run optimized code
```bash
$ cargo build --release
   Compiling hello_cargo v0.0.1 (file:///C:/Users/juanrh/git/rust-experiments/hello_cargo)
    Finished release [optimized] target(s) in 0.24 secs
$ target/release/hello_cargo.exe
Hello Rust with Cargo!
```