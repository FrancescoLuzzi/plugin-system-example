# plugin-system-example
Example design of executable with `dylib`-based runtime plugin system in Rust, `libloading` used

## Usage

1. Compile plugin using `cargo build -p my-plugin`
2. Run master by `cargo run -p my-master`

## Example working and not are in the other branches of this repo

- [spawn_or_spawn_blocking](https://github.com/FrancescoLuzzi/plugin-system-example/tree/spawn_or_spawn_blocking)
- [handle_at_init](https://github.com/FrancescoLuzzi/plugin-system-example/tree/handle_at_init)
- [get_handle_fn](https://github.com/FrancescoLuzzi/plugin-system-example/tree/get_handle_fn)
- [pass_handler](https://github.com/FrancescoLuzzi/plugin-system-example/tree/pass_handler)

For more infos on what went on theese branches read [this](https://stackoverflow.com/questions/77294605/library-plugin-manager-in-rust-is-it-even-doable-right-now) StackOverflow question
