Collection of useful template & docs to quickly start developing with bevy.

## Preparation

- Install dependencies

    - Ubuntu:

      ```
      sudo apt update
      sudo apt install g++ pkg-config libx11-dev libasound2-dev libudev-dev libxkbcommon-x11-0
      ```

      > More on this: https://github.com/bevyengine/bevy/blob/main/docs/linux_dependencies.md

- Editor setup (RustRover)
    - External Linters
        - Use `Clippy`
        - Enable to run on the fly
    - Formatting
        - Use `Rustfmt` instead of the built-in formatter
        - Configure action on save
            - Check Reformat Code
            - Check Optimize Import

- Enable dynamic linking for faster compile in development
    - On Run configuration, append `--features bevy/dynamic_linking` inside the command
    - On Settings > Rust > External Linters, add `--features bevy/dynamic_linking` on Additional arguments

  > More on this: https://bevyengine.org/learn/quick-start/getting-started/setup/#dynamic-linking

## Building & Running

- Review dependency on [Cargo.toml](Cargo.toml)
- Run `cargo build` and check if compilation succeed

## Testing

See [test.rs](src/people/tests.rs).

## Docs

- [Tainted Coders](https://taintedcoders.com/)

## References

- [bevy-starter](https://github.com/nolantait/bevy-starter)
- [Bevy opinionated best practices](https://github.com/tbillington/bevy_best_practices)
