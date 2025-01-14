# Rust Actix

## Introduction

- Actix-web is a powerful, pragmatic, and extremely fast web framework for Rust. It is built on the Actix actor framework and supports asynchronous programming with async/await.

## Features

- Asynchronous 
- Event Driven
- Supports middleware, routes, and RESTful APIs
- WebSockets support
- High performance and scalability

## Setting Up Actix

- Create a new Rust project using `cargo new actix_project --bin`
- Add the following dependencies to your `Cargo.toml` file:

```toml
[dependencies]
actix-web = "4.0.0-beta.8"
```

- Run `cargo build` to install the dependencies