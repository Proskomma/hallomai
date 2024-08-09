# Hallomai

Hallomai is a Rust-based library for transforming files between different formats: USFM, USX, and JSON. The library is designed to be compiled to WebAssembly (Wasm) for use in web applications, providing fast and reliable format conversions directly in the browser.

## Table of Contents

- [Overview](#overview)
- [Supported Formats](#supported-formats)
- [Installation](#installation)
- [Usage](#usage)
    - [Rust](#rust)
    - [Wasm](#wasm)
- [Building for WebAssembly](#building-for-webassembly)
- [Testing](#testing)
- [Contributing](#contributing)
- [License](#license)

## Overview

Hallomai provides functions to transform content from one format to another. The library currently supports three file formats:

- **USFM** (Unified Standard Format Marker)
- **USX** (Unified XML Standard)
- **JSON** (JavaScript Object Notation)

The main functionality is exposed via the `transform` function, which can be compiled to WebAssembly for integration into web applications. This allows for seamless file format transformations directly in the browser.

## Supported Formats

The following input and output file formats are supported:

- **Input Formats**: "usfm", "usx", "json" (or "usj")
- **Output Formats**: "usfm", "usx", "json" (or "usj")

## Installation

### Rust

Add the following to your `Cargo.toml`:

```toml
[dependencies]
hallomai = "0.1.0"
```

### Wasm
To use Hallomai in a web application, you need to compile it to WebAssembly. Follow the instructions in the Building for WebAssembly section.

## Usage
### Rust

```rust
use hallomai::transform;

let input_content = String::from("your input content here");
let input_format = String::from("usfm");
let output_format = String::from("json");

let result = transform(input_content, input_format, output_format);

println!("Transformed content: {}", result);
```

### Wasm
First, compile the library to WebAssembly:
```sh
wasm-pack build --target web
```

Then, use it in your JavaScript application:
```javascript
import init, { transform } from './path_to_generated_wasm_file';

async function run() {
    await init();
    const inputContent = `your input content here`;
    const result = transform(inputContent, "usfm", "json");
    console.log("Transformed content: ", result);
}

run();
```

### Benchmarks in Wasm (in a js code)

#### From the USFM of the book of "Song of Songs"

![song_of_songs_benchmark](./assets/images/song_of_songs_benchmark.png)

#### From the USFM of the book of "Psalms"

![song_of_songs_benchmark](./assets/images/psalms_benchmark.png)

## Building for WebAssembly

To build Hallomai for WebAssembly:
1. Install the necessary tools:
```sh
cargo install wasm-pack
```
2. Build the project:
```sh
wasm-pack build --target web
```
3. The generated WebAssembly files will be in the pkg directory. You can include these files in your web application.

## Testing
TODO

## Contributing

Contributions are welcome! Please open an issue or submit a pull request on GitHub.
1. Fork the repository.
2. Create your feature branch (git checkout -b feature/YourFeature).
3. Commit your changes (git commit -m 'Add YourFeature').
4. Push to the branch (git push origin feature/YourFeature).
5. Open a pull request.

## License

Hallomai is licensed under the MIT License. See the [LICENSE](LICENSE) file for more details.