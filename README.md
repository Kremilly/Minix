# Minix

A straightforward minifier for JavaScript and CSS files, developed using Rust. This tool efficiently reduces the size of JS and CSS files by removing unnecessary characters, whitespace, and comments without affecting functionality. Utilizing Rust’s performance and safety features, it offers a fast and reliable solution for optimizing web assets, improving load times and overall website performance. Ideal for developers looking for a lightweight and effective way to minimize their codebase.

To Install using [crates.io](https://crates.io)

```shell
cargo install minix
```

Basic example usage:

```shell
minix -- -i file.js -o file.min.js
```
