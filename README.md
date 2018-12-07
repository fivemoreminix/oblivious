# Building The Game
Compiling the game is very easy. All you need is:
* A working internet connection
* The Rust compiler and Cargo package manager (https://rust-lang.org/)

Download the source code and put the folder somewhere on your computer. Open the folder in a terminal and within the oblivious folder, execute:
```sh
cargo run
```
The game will build in debug mode and begin running immediately. For a permanent build, it is preferred you do a release build using the following:
```sh
cargo build --release
```
Where you can then find the executable for the game in the `target/release` folder.
