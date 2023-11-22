# Nero

> v0.10.1
> We're gonna hit 1.0.0 soon:)

**NOTICE:** Since Nero is going to hit 1.0.0 soon, I am working on a different project that demonstrates the use of Nero in a "real world scenario". This will help catch bugs and stuff and also make sure Nero is actually working fine. The project will be [here](https://github.com/dragsbruh/neroo) soon and will be called Neroo (for secret reasons).

**NOTICE:** Nero is temporarily discontinued :\(

## Overview

Nero is a post-exploitation-like framework developed in Rust. It is designed to be lightweight with low system overhead, making it suitable for backup access on servers. However, it's crucial to emphasize that the post-exploitation features are intended strictly for educational purposes and should not be used for unethical activities.

The framework's flexibility lies in its ability to support various client-server architectures. Nero's extensibility allows users to easily add commands to tailor the tool to their specific needs.

## Features

- **Lightweight and Low Overhead:** Nero is optimized for minimal impact on system resources, making it an efficient choice for backup access.

- **Flexibility:** Nero supports any client-server architecture, providing users the freedom to choose the setup that best fits their requirements. Initially built as a command-line tool for development purposes, it can easily adapt to other architectures, such as a Discord bot.

- **Extensibility:** Users can effortlessly add commands to the framework, enhancing its functionality according to specific use cases.

- **Synchronous Design:** Nero adopts a purely synchronous approach for ease of maintenance. This design choice is driven by preferences, simplicity, and the absence of a need for asynchronous properties in the project.

## Disadvantages

- **Windows only:** Nero's development now focuses on Windows targets, so Linux targets might not work at all. Heck, they might not even compile, meaning that this tool currently is heavily unsuitable for servers, since most servers run on Linux. But hang tight, we will do it :\)
- **Heavy development:** Nero is under some serious development. It is highly unstable and will most likely be useless. But hang tight for this one too, we will do it :\)

## Contribution Guidelines

Nero is currently in heavy development, and the codebase is subject to frequent changes. Contributors are advised to exercise caution when making significant contributions, as the architecture, including the out architecture for handling outgoing messages, may undergo modifications.

**Important Note:** Major contributions are discouraged until Nero reaches the beta stage. This ensures stability and reduces the risk of conflicts with ongoing development.

## Project Structure

- **`src/`:** Contains the source code of the Nero framework.

- **`docs/`:** Documentation for using and contributing to Nero.

- **`examples/`:** Examples demonstrating the implementation of Nero commands.

## Getting Started

To get started with Nero, refer to the documentation in the `docs/` directory. It provides comprehensive information on installation, usage, and contribution guidelines.

## Modifying Nero

The flexibility of Nero extends to its main.rs file, allowing users to tailor it to their specific requirements. Below is a basic template illustrating how you might structure your main.rs. Keep in mind that this is just a starting point, and you can modify it extensively based on your use case, and also the fact that extensibility means ease of migrating to different client-server architectures.

```rust
// TODO: this
```

## Disclaimer

Nero is developed for educational purposes and ethical use only. Any misuse or unethical activities using the post-exploitation features are strongly discouraged.

## License

Nero is open-source software released under the [Apache License 2.0](LICENSE). Feel free to contribute, share, and modify the code in compliance with the license terms.
