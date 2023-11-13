# Command Developer's Guide

Welcome to the command development guide for the Nero. This guide provides a step-by-step walkthrough for developers looking to contribute new commands to the Nero project.

## Prerequisites

Before you begin, ensure you have the following:

- Rust programming environment set up.
- Familiarity with Nero's existing codebase.
- Understanding of synchronous programming, as Nero aims to be purely synchronous.

## Command Structure

Commands in Nero are structured using the following template:

```rust
pub fn your_command(args: Args, out: OutFun) {
    // Your command logic here
    text_output!(out, "Your output text");
}
```

- `args`: Represents the arguments supplied by the user (a `Vec<String>`).
- `out`: An output function that accepts the `Output` enum type (Text or Media).

## Output Macros

Nero simplifies output generation with macros:

- `text_output!(out, "Your text data")`: Sends text output to the specified `out` function.
- `media_output!(out, "filename", "base64_encoded_data")`: Sends media output to the specified `out` function.

## Registering Your Command

To make your command accessible, register it in `core.rs`:

```rust
// Registry initializer
pub fn init(out: OutFun) -> Registry {
    use crate::commands::*;

    let mut reg = Registry::new(out);

    reg.enter("your_command", your_command);

    // Additional command registrations go here

    return reg;
}
```

If your command very niche? Will it have a very specific use case? Set a feature flag to your command and the registry line.
You may look at `rickroll` command for example.

```rust
// commands.rs
#[cfg(feature = "niche_feature")]
pub fn your_command() {
    // Something niche
}

// core.rs]
pub fn init(out: OutFun) -> Registry {
    //... existing code

    #[cfg(feature = "niche_feature")]
    reg.enter("your_command", your_command);

    //... existing code
}
```

## Additional Considerations

- The project is designed to be purely synchronous; avoid using async properties.
- Explore the existing commands, like "ping" and "quit," as examples.
- Feature flags can conditionally include or exclude commands based on project configurations.

## Contributing Your Command

1. Develop your command logic in `commands.rs`.
2. Utilize the provided macros for streamlined output generation.
3. Register your command in the `init` function in `core.rs`.
4. Test your command within Nero.
5. Submit a pull request to contribute your command to the project.
