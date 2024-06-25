Collection of useful docs to quickly start developing with bevy using RustRover on Ubuntu.

# Setup

- Install dependencies

  For ubuntu:

    ```
    sudo apt update
    sudo apt install g++ pkg-config libx11-dev libasound2-dev libudev-dev libxkbcommon-x11-0
    ```
    
    > More on this: https://github.com/bevyengine/bevy/blob/main/docs/linux_dependencies.md

- Add bevy dependency to `Cargo.toml`

- Run `cargo build`

- Enable Query support for RustRover
  - `org.rust.cargo.evaluate.build.scripts`
  - `org.rust.macros.proc`

  > More on this: https://bevy-cheatbook.github.io/setup/editor/jetbrains.html

# Testing
> More on this: https://bevy-cheatbook.github.io/patterns/system-tests.html
