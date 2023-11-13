# Creating Your Own Client-Server Architecture for Nero

If you want to customize Nero's client-server architecture for your specific needs, the process is straightforward. The provided `main.rs` file is where you'll make most of your modifications. This guide will walk you through the structure and provide examples for creating your client-server architecture.

## Understanding the Provided `main.rs`

The `main.rs` file provided in Nero's development environment is a simple command-line tool that interacts with the Nero core. Here's a quick breakdown:

- **Imports:**
  - `commands`: The module where you can define your custom commands.
  - `utils`: A module for utility functions.
  - `core`: The core functionality of Nero, including the `Output` enum.
  - `nero`: A module simplifying the usage of the Nero core.

- **Main Function:**
  - The `main` function:
    - Defines an `out` function that prints either media saving information or text output.
    - Creates an instance of `Nero` with the defined `out` function.
    - Enters a loop to continuously read user input and execute Nero commands.

## Modifying `main.rs`

### 1. Customize the `out` Function

Modify the `out` function to suit your preferred way of handling output. You can customize how media and text outputs are displayed or logged.

### 2. Handle User Input

Modify the loop to handle user input according to your desired input method. It could be through a graphical user interface (GUI), a web interface, or any other form that suits your application.

## Examples

### Customizing the `out` Function

```rust
fn out(output: Output) {
    match output {
        Output::Media(media) => {
            // Implement your own save_file function.
            save_file(media.name, media.data); // Media.data is a base64 encoded string
            println!("Media saved at: {}", media.name);
        }
        Output::Text(text) => {
            println!("Text: {}", text.data); // Text.data is a plain text string
        }
    }
}
```

More examples are given [here](../examples/README.md).

## Conclusion

The provided `main.rs` serves as a starting point, and customization is encouraged to meet your specific requirements. Feel free to experiment, add more features, and adapt the client-server architecture to suit your needs.

Happy customizing!
