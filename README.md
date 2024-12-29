# rust-book
Repository for going through main Rust book.
The book can be found at https://doc.rust-lang.org/book/
An experimental version with quizzes can be found at https://rust-book.cs.brown.edu/

# Command notes

```bash
$ rustup show
$ rustup update
$ rustup self uninstall
$ rustc --version
$ rustc main.rs
$ cargo --version
$ cargo help new
$ cargo new project_name
$ cargo new --vcs project_name # Untested, avoid creating git repo, may need --vcs=none
$ cd project_name
$ cat Cargo.toml # Tom's Obvious, Minimal Language, similar to Windows .ini files.
$ vi src/main.rs
$ cargo build
$ ./target/debug/project_name
$ cargo run
$ cargo check # Check for errors without outputting binary.
$ cargo build --release
```
