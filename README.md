# NES emulator in Rust

Project made follwing the [NES Ebook](https://bugzmanov.github.io/nes_ebook/chapter_1.html)

This is a project that I planned to make on a weekend but I overly underestimated it

## How to use

To run the program you need to have rustc and Cargo installed.  
You can run the program without compiling using Cargo
```
# use the --release flag for better performance
cargo run --release -- <PATH-TO-FILE>
```

or compile it
```
cargo build --release
# the executable is generated located at "target/release/nes_emulator_in_rust"
# it can be copied and placed wherever you desire
./target/release/nes_emulator_in_rust <PATH-TO-FILE>
```

## Progress

- [x] CPU
- [x] PPU
- [x] JOYPADS (need to create configuration for two players)
- [ ] APU
- [x] Implement CLI arguments

The book isn't complete yet, so the APU is not implemented, and I don't think I have necessary knowledge to do it by my self yet, perhaps after reading the recommended material on the book I come back to this project.
