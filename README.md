# Mines

![mines screenshot](res/screenshot.png "Mines screenshot")

Compatible with [Redox OS](https://github.com/redox-os/redox) and SDL2 platforms.

You can play the game in 9x9 field with 10 mines, however there are missing features. To be done:

- [ ] Right click to mark cell as mine (waiting for orbtk to support it)
- [ ] Create icon for a mine
- [ ] Mark all mines when you finish the game (you can only see the reset button change from `:)` to `:D`)
- [ ] Finish implementing mine countdown (required marking mines first)
- [ ] Implement time counter
- [ ] Implement larger fields
- [ ] Make prettier UI
- [ ] Add shared scores


## How to use it

### Get SDL2

Follow the [SDL2 instructions](https://github.com/Rust-SDL2/rust-sdl2).

In my case it didn't work on Windows that way, I had to copy files from `SDL2-devel-2.0.x-VC.zip` to
`C:\Users\<user>\.rustup\toolchains\nightly-x86_64-pc-windows-msvc\lib\rustlib\x86_64-pc-windows-msvc\lib`

### Run

```
cargo run
```
