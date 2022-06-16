// Packages: A Cargo feature that lets you build, test, and share crates
// Crates: A tree of modules that produces a library or executable
// Modules and use: Let you control the organization, scope, and privacy of paths
// Paths: A way of naming an item, such as a struct, function, or module

// THE MODULE SYSTEM:

// Part 1: Packages and Crates:

// Package: One or more crates. A package (&self) contains a Cargo.toml file that describes how to build those crates
// -------

// Crate: Is either a Binary or Library crate
// -----

// Binary crate: a program you can compile to an executable (ex: Command line programs or servers). They must have a `main` function
// ------------

// Library crate: Doesn't have a `main` function and doesn't compile to exe. To be used with Binary crates (ex: rand)
// -------------

// Package rules:
// 1. A package can have at most 1 library crate.
// 2. It can contain as many binary crates as you want.
// 3. It must contain at least one create (either library or binary)

// Part 2: Defining Modules to Control Scope and Privacy

// Rules for easy reference

// 1. Start from the crate root file
// Either src/main.rs or src/lin.rs

// 2. Declaring modules
// in the root file, declare a new module with the `mod` keyword. Ex: `mod garden`.
// The compiler will automatically look into 3 places:
// - directly following `mod garden` within the curly brackets
// - in the file src/garden.rs
// - in the file src/garden/mod.rs

// 3. Declaring submodules
// in any file except the crate root file, you can declare submodules.
// Ex: in `src/garden.rs`, declare submodule: `mod vegetables`
// The complier will check the same 3 relative places. Ex: (inline, `src/garden/vegetables.rs`, or `src/garden/vegetanles/mod.rs` )

// 4. Path to code in modules
// Once a module is compiled, you can refer to code in that module by using the path
// ex: `crate::garden::vegetables::Asparagus`

// 5. Private vs public (private by default)
// to make a module public: `pub mod`
// to make items public within a public module: `pub` before its declaration

// 6. The `use` keyword. Creates shortcuts to ittem within a scope
// Example,
// 1. create a shortcut: `use crate::garden::vegetables::Asparagus`
// 2. Only need to write `Asparagus` to make use of the type in the scope

use crate::garden::vegetables::Asparagus;
// use crate::garden::vegetables;
// use vegetables::Asparagus; // also works

pub mod garden;

fn main() {
    let plant = Asparagus {};
    println!("I'm growing {:?}!", plant);
}
