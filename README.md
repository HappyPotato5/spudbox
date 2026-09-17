[![Crates.io](https://img.shields.io/badge/Crates.io-latest-blue?style=for-the-badge&logo=rust&logoColor=black&labelColor=BBBBBB)](https://crates.io/crates/spudbox)
[![Docs.rs](https://img.shields.io/badge/Documentation-SpudBox-blue?style=for-the-badge&logo=Docs.rs&logoColor=black&labelColor=BBBBBB)](https://docs.rs/serax/latest/spudbox/)
[![Github](https://img.shields.io/badge/Github-SpudBox-blue?style=for-the-badge&logo=github&logoColor=black&labelColor=BBBBBB)](https://github.com/HappyPotato5/spudbox)
[![License](https://img.shields.io/badge/License-GPL--v3-blue?style=for-the-badge&logo=readdotcv&logoColor=black&labelColor=BBBBBB)](https://github.com/HappyPotato5/spudbox/blob/master/LICENSE)

# Spudbox
**Spudbox** is a library that implements some **data structures** used in my **Rust** projects.

## Types
+ [`VecSet<T>`](https://docs.rs/spudbox/0.1.0/spudbox/vecset/struct.VecSet.html) Is an **ordered** `HashSet<T>`.
+ [`Arena<T>`](https://docs.rs/spudbox/0.1.0/spudbox/arena/struct.Arena.html) Is an **Arena allocator** for any **T** type.

## Features
This crate is **subdivided** with **features** to allow you to only import the submodules that **you use**.

### Default:
+ **`vecset`**: Enables [`VecSet<T>`](https://docs.rs/spudbox/0.1.0/spudbox/vecset/struct.VecSet.html).
+ **`arena`**: Enables [`Arena<T>`](https://docs.rs/spudbox/0.1.0/spudbox/arena/struct.Arena.html).


All the contents from **submodules** are automatically **re-exported** through the **spudbox** crate, to avoid this you can use the **`no-prelude`** **feature**.