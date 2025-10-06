# Cargo

## Introduction 

- Cargo is the Rust package manager and build system. It simplifies the process of managing Rust projects, including dependencies, compilation, and distribution.

### Binary Project

- A binary project is a Rust project that produces an executable. It typically contains a `src/main.rs` file, which is the entry point of the application.
- To create a new binary project, use the command:
  
```bash
  cargo new my_project --bin
```
or  
```bash
  cargo init
```

### Library Project

- A library project is a Rust project that produces a library, which can be used by other Rust projects. It typically contains a `src/lib.rs` file.
- To create a new library project, use the command:

```bash
  cargo new my_library --lib
```
or  
```bash
  cargo init --lib
```

