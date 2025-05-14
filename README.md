### Zellij GetMode

This plugin is a simple utility plugin that gets the current input mode of
Zellij.


#### Disclaimer

I'm new to rust, and this is plugin was really just an exercise. Feel free to
fork to fit your usecase.


#### How to build
Run the cargo build command and update your zellij config
```
cargo build --release --target wasm32-wasip1
# Add the built wasm binary to your zellij config.
#
# file: ~/.config/zellij/config.kdl (in mac)
# load_plugins {
#     "file:~/sources/github-cskth/zellij-getmode/target/wasm32-wasip1/release/zellij-getmode.wasm"
# }
#
```

#### How to use
```
zellij pipe getmode
```
