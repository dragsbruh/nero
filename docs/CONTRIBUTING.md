# Nero Core Contribution Guidelines

Welcome to the Nero Core contribution guide! As a core contributor, you play a crucial role in enhancing and maintaining the foundational components of the Nero framework. This guide provides an overview of Nero's internal structure and outlines key points for contributing to the core functionality.

**This guide is for legends who want to contribute to core.rs.**

> - To create a client server architecture, follow [this guide](FLEX.md).
> - To add commands, follow [this guide](COMMANDS.md).

## Overview of Core.rs

The `core.rs` file defines essential enums, structs, and type aliases that form the backbone of Nero's internal communication system. Here's a brief overview:

- **Enums and Structs:**
  - `Output` enum contains `Text` and `Media` structs, forming the output system.
  - Three type aliases:
    - `Args`: A vector of strings representing command arguments.
    - `OutFun`: A function type for outgoing messages (takes `Output` enum).
    - `CmdFun`: A function type used by command developers to implement commands (takes `Args` and `OutFun`).

- **Command Input Parsing:**
  - Command inputs are generally received as strings and parsed by the `Command` struct.
  - The `Command` struct holds `name` and `args` fields.

- **Initialization Process:**
  - During initialization, `main.rs` calls the `init` function in `core.rs`.
  - The `init` function initializes the command registry and returns a `Registry` instance.

- **Registry Methods:**
  - `Registry` implements four methods: `new`, `get`, `enter`, and `exec`.
    - `new`: Creates a new `Registry` instance.
    - `get`: Retrieves a registry command as a `CmdFun` type.
    - `enter`: Sets a registry command (takes command name and command function).
    - `exec`: Executes a registry command (takes command name and args).

- **Tests:**
  - The file includes tests for the command parser.

## Contribution Guidelines

- **Priority:**
  - Address high-priority issues first. Lower-priority issues, such as those related to the command parser, can be tackled later.

- **Testing:**
  - Ensure that new contributions include appropriate tests.
  - Focus on improving the command parser tests as needed.

- **Documentation:**
  - Keep documentation up-to-date. If there are changes to the internal workings, update relevant sections in this guide.

- **Code Quality:**
  - Adhere to Rust's coding conventions and best practices.
  - Consider refactoring or optimizing existing code if opportunities arise.

## Interaction with Other Files

- `core.rs` is utilized by `nero.rs`, which simplifies creating client-server architecture in `main.rs`.
- `nero.rs` defines a `Nero` struct to streamline the usage of `core.rs`.

## Useful Structures

- `Nero::new`: Similar to `Vec::new`, spawns a new Nero instance.
- `Nero::exec`: Parses and executes a command (string).

## Pubbed Elements

- `core::Output`: Exposed for utility in `main.rs` development.
