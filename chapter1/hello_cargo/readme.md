Notes about cargo

- `cargo run` will run `./target/debug/{project_name}`, and if there are active code changes, will compile before doing so.
- `cargo build` will build an executable of the code
- `cargo check` will compiles but doesn't produce executable
- `cargo build --release` will optimize. takes a tad longer.
