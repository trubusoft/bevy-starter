Template to quickly start developing with bevy.
Highly inspired from [bevy_new_2d](https://github.com/TheBevyFlock/bevy_new_2d)
and [nolantait/bevy-starter](https://github.com/nolantait/bevy-starter).

## Preparation

### Install dependencies

#### Ubuntu

```
sudo apt update
sudo apt install g++ pkg-config libx11-dev libasound2-dev libudev-dev libxkbcommon-x11-0
```

> More on
>
this: [bevyengine/linux_dependencies](https://github.com/bevyengine/bevy/blob/latest/docs/linux_dependencies.md#ubuntu)

#### Windows

Follow the [windows setup](https://bevyengine.org/learn/quick-start/getting-started/setup/#windows)

### Editor setup

We assume that RustRover is being used.

#### External Linter

Use 'Clippy' and optionally enable to run on the fly.

#### Formatting

Use `Rustfmt` instead of the built-in formatter.

Configure action on save:

- Check Reformat Code
- Do **not** check the Optimize Import, as sometiems it conflicts with `Rustfmt` formatting. Instead, run
  the Optimize Import on case-by-case basis when import optimization is needed (e.g. plenty of unused import
  after refactoring)

#### Run Configurations

Create several Shell configurations, like:

- cargo run

    ```
    cargo run --features bevy/dynamic_linking --color=always
    ```

- cargo test

    ```
    cargo test --features bevy/dynamic_linking --color=always
    ```

- cargo build for release

    ```
    cargo build --release --no-default-features --color=always
    ```

## Docs and References

- [Tainted Coders](https://taintedcoders.com/)
- [Bevy opinionated best practices](https://github.com/tbillington/bevy_best_practices)
