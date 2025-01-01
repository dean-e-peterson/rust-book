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

## Crates

Crates live at https://crates.io/

To use crates, add a crate reference with version and build.
```bash
$ vi Cargo.toml
$ cargo build
$ cat Cargo.lock
```

The first build resolves versions, obeying the first two version components
in Cargo.toml but grabbing the latest third version component.
However, those versions get written to Cargo.lock, and subsequent
builds won't even vary the third component unless you do...

```bash
$ cargo update
$ cat Cargo.lock
```

...which will act like the first build and only obey the first two version
but get the latest third one.

To update either of the first two version components you might have to
manually edit the version in your Cargo.toml file?  Is there another way?

```bash
$ vi Cargo.toml
$ cargo build
```

To build crate documentation and view in browser.

```bash
$ cargo doc --open
```

On `unit`:
> The tuple without any values has a special name, unit.
> This value and its corresponding type are both written ()
> and represent an empty value or an empty return type.
> Expressions implicitly return the unit value
> if they don’t return any other value.

