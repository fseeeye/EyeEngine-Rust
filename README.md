# EyeEngine-Rust

[![Crates.io][shield1]][crate]
[![License][shield2]](./LICENSE)
![Lines][shield3]

*FOA, sry for my poor English.*

## What is Eye Engine?
Eye Engine is my first Real-Time 3D Game Engine learning project. 

There are two editions: 
* [Rust Edition](https://github.com/fseeeye/EyeEngine-Rust).
* [Cpp Edition](https://github.com/fseeeye/EyeEngine-Cpp). 

They are completely independent and have different architecture & development plan! 

As I say, **Building games above this engine is not the first level target**! The main purpose of those projects is helping me to learn modern 3D Game Engine, Graphic API & SL and practice Computer Grahpic theory. 

## Getting Started
1. Instal Rust lang and comfirm your Rust toolchain is **nightly**. I like to use latest features.
2. Run example. For now, there is only one example to test my project working well:
```sh
# Run the simple example to show a static triangle
cargo run --example simple
```

## Mainly Used Crates
* [winit](https://github.com/rust-windowing/winit): cross-platform window creator and manager. 
* [wgpu](https://wgpu.rs/): cross-platform graphics API wrapper. (support Vulkan, Metal, Dx12, Dx11, Gl, BrowserWebGpu)

## Reference
* [bevy](https://github.com/bevyengine/bevy) : A refreshingly simple data-driven game engine written in Rust. it's my favorite game engine. 
* [amethyst](https://github.com/amethyst/amethyst) : Data-oriented and data-driven game engine written in Rust.


[crate]: https://crates.io/crates/eyengine
[shield1]: https://img.shields.io/crates/v/eyengine
[shield2]: https://img.shields.io/crates/l/eyengine
[shield3]: https://tokei.rs/b1/github/fseeeye/EyeEngine-Rust?category=lines
