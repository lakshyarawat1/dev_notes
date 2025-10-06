# Packages, Crates and Modules

## Crates

- Smallest unit of code that the Rust compiler considers at a time.
- Single source code file is considered as a crate by the compiler.
- Crates can contain multiple modules.
- Crates have two types:
  - **Binary Crate**: Contains a `main` function and can be executed.
  - **Library Crate**: Does not contain a `main` function and is used as a library for other crates and they don't compile to an executable.
- Crate root : Source file that the compiler starts with when compiling a crate.


## Packages

- Bundle of one or more crates.
- Package contains a `Cargo.toml` file that describes the package and its dependencies.

### Compiler with modules

- Compiler first looks for the crate root file (usually src/lib.rs for library crate or src/main.rs for the binary crate) for code to compile.
- In crate root file, you can declare new modules
- Compiler will look for the code in :  
  - Inline, with curly braces that replace the semicolor following `mod module_name;`
  - In the file `src/module_name.rs` 
  - In the file `src/module_name/mod.rs` (if the module is a directory)
- __Declaring submodules__ : 
  - Can be declared in any file other than the crate root file.
- `use` keyword is used to bring items into scope.