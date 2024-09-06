# HACK AND HEAT - Let's get rusty!

Prerequisites:
1. Rust toolchain installed via [rustup.sh](https://rustup.sh) or by other means (Your distros package manager, nix, etc.)
2. Check this git repo out:
```sh
git clone git@github.com:Mastermindaxe/hack_and_heat_rust_workshop.git
```

## Part 1: Rustlings

[Rustlings](https://github.com/rust-lang/rustlings) is a tool that guides you through your first steps of Rust with the help of the official Rust compiler. Install it using [their guide on github](https://github.com/rust-lang/rustlings#installing-rustlings) which amounts to `cargo install rustlings` or by using a nix shell `nix-shell -p rustlings`.
After that just run:
```sh
rustlings init
cd rustlings
rustlings
```
and off you go. Open an editor of your choice (RustRover by Jetbrains, VSCode with rust-analyzer, Zed, Helix, etc.) and let's get started!

The rustlings tool will guide you through the exercises step-by-step. Just follow their guides and fix the buggy code! Keep in mind to carefully read the error messages and don't be afraid to press 'h' for hints.

Don't be afraid to call for help if you have questions regarding the assignment or if you want to ask follow-up questions.

### When to stop

__11:00__

## Part 2: Teaser - Rust magic?

There are multiple possible ways for you to explore the second part of this workshop:
- Take a look at built-in benchmarks in [benchmarks](part_2/benchmarks)
- Documentation that tests itself??? -> [doc_tests](part_2/doc_tests)
- Deploy a webservice without specifying any infrastructure code -> [shuttle/hack-and-heat-main](part_2/shuttle/hack-and-heat-main)
- Look at an actual backend service with all the goodies -> come over and take a look at my laptop
