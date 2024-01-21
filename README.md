<!-- markdownlint-disable MD033 MD041 -->

<img
src="https://kura.pro/wiserone/images/logos/wiserone.webp"
alt="the wiser one's logo"
height="199"
width="199"
align="right"
/>

<!-- markdownlint-enable MD033 MD041 -->

# The Wiser One

Daily nuggets of wisdom in a clean, minimalist design, inspiring deeper thought and personal growth with every visit.

<!-- markdownlint-disable MD033 MD041 -->
<center>
<!-- markdownlint-enable MD033 MD041 -->

[![Made With Rust][made-with-rust-badge]][5]
[![Crates.io][crates-badge]][7]
[![Lib.rs][libs-badge]][9]
[![Docs.rs][docs-badge]][8]
[![License][license-badge]][2]

• [Website][0]
• [Documentation][8]
• [Report Bug][3]
• [Request Feature][3]
• [Contributing Guidelines][4]

<!-- markdownlint-disable MD033 MD041 -->
</center>
<!-- markdownlint-enable MD033 MD041 -->

![divider][divider]

## Overview 📖

The Wiser One offers daily insights and wisdom in a sleek, minimalist interface, designed to inspire deeper thinking and foster personal growth. Each visit presents a unique opportunity to reflect and gain new perspectives.

## Features ✨

### Reading Quotes from a JSON File

- The Wiser One can seamlessly access and parse a wide range of quotes stored in a JSON format. This allows for a diverse and extensive collection of wisdom nuggets. The JSON structure is optimized for quick retrieval, ensuring a smooth user experience.

### Reading Quotes from a CSV File

- The Wiser One can also read quotes from a CSV file. This allows users to easily import their own collection of quotes, or to create a custom library of quotes. The CSV format is simple and intuitive, making it easy to add, edit, or delete quotes. This feature allows users to create a personalized collection of wisdom nuggets.

### Randomly Selecting a Quote

- With an innovative random selection algorithm, the Wiser One presents a different quote each time, making every interaction unique. This feature encourages varied learning and prevents the monotony of repetitive content. The randomness is designed to simulate the unpredictability and richness of gaining wisdom in real life.

### Creating an HTML File with a Random Quote

- The Wiser One can generate a beautifully formatted HTML file for each selected quote. This allows users to save their favourite quotes in a visually appealing format, which can be easily shared or printed. The HTML output includes customizable themes and layouts, giving a personalized touch to each piece of wisdom.

### Creating all the HTML Files with all the Quotes

- The Wiser One can also generate a complete set of HTML files for all the quotes in the library. This allows users to easily access their entire collection of wisdom nuggets in a visually appealing format. The HTML output includes customizable themes and layouts, giving a personalized touch to each piece of wisdom.

These features combine to make the Wiser One a powerful tool for those seeking daily inspiration and wisdom. The application's ease of use, coupled with its thoughtful design, makes it an ideal choice for users looking to enrich their daily routine with meaningful insights.

## Getting Started 🚀

It takes just a few minutes to get up and running with `wiserone`.

### Installation

To install `wiserone`, you need to have the Rust toolchain installed on
your machine. You can install the Rust toolchain by following the
instructions on the [Rust website][13].

Once you have the Rust toolchain installed, you can install `wiserone`
using the following command:

```shell
cargo install wiserone
```

You can then run the help command to see the available options:

```shell
wiserone --help
```

### Requirements

The minimum supported Rust toolchain version is currently Rust
**1.75.0** or later (stable).

### Platform support

`wiserone` is supported and tested on the following platforms:

### Tier 1 platforms 🏆

| | Operating System | Target | Description |
| --- | --- | --- | --- |
| ✅ | Linux   | aarch64-unknown-linux-gnu | 64-bit Linux systems on ARM architecture |
| ✅ | Linux   | i686-unknown-linux-gnu | 32-bit Linux (kernel 3.2+, glibc 2.17+) |
| ✅ | Linux   | x86_64-unknown-linux-gnu | 64-bit Linux (kernel 2.6.32+, glibc 2.11+) |
| ✅ | macOS   | x86_64-apple-darwin | 64-bit macOS (10.7 Lion or later) |
| ✅ | Windows | i686-pc-windows-gnu | 32-bit Windows (7 or later) |
| ✅ | Windows | i686-pc-windows-msvc | 32-bit Windows (7 or later) |
| ✅ | Windows | x86_64-pc-windows-gnu | 64-bit Windows (7 or later) |
| ✅ | Windows | x86_64-pc-windows-msvc | 64-bit Windows (7 or later) |

### Tier 2 platforms 🥈

| | Operating System | Target | Description |
| --- | --- | --- | --- |
| ✅ | Linux   | aarch64-unknown-linux-musl | 64-bit Linux systems on ARM architecture |
| ✅ | Linux   | arm-unknown-linux-gnueabi | ARMv6 Linux (kernel 3.2, glibc 2.17) |
| ✅ | Linux   | arm-unknown-linux-gnueabihf | ARMv7 Linux, hardfloat (kernel 3.2, glibc 2.17) |
| ✅ | Linux   | armv7-unknown-linux-gnueabihf | ARMv7 Linux, hardfloat (kernel 3.2, glibc 2.17) |
| ✅ | Linux   | mips-unknown-linux-gnu | MIPS Linux (kernel 2.6.32+, glibc 2.11+) |
| ✅ | Linux   | mips64-unknown-linux-gnuabi64 | MIPS64 Linux (kernel 2.6.32+, glibc 2.11+) |
| ✅ | Linux   | mips64el-unknown-linux-gnuabi64 | MIPS64 Linux (kernel 2.6.32+, glibc 2.11+) |
| ✅ | Linux   | mipsel-unknown-linux-gnu | MIPS Linux (kernel 2.6.32+, glibc 2.11+) |
| ✅ | macOS   | aarch64-apple-darwin | 64-bit macOS (10.7 Lion or later) |
| ✅ | Windows | aarch64-pc-windows-msvc | 64-bit Windows (7 or later) |

The [GitHub Actions][10] shows the platforms in which the `wiserone` library tests are run.

### Documentation

**Info:** Please check out our [website][0] for more information. You can find our documentation on [docs.rs][8], [lib.rs][9] and
[crates.io][7].

## Usage

### Command-line interface

`wiserone` provides a convenient way to generate daily quotes from a JSON file using the command line interface. There are a few options available to help you get started.

#### Generate a random quote

The following command generates a random quote from the `quotes.json` file.

```shell
wiserone --random ./quotes/01-quotes.json
```

or locally if you have cloned the repository:

```shell
cargo run random ./quotes/01-quotes.json
```


Have a look at the `tests/data/mylibrary.csv` file for an example and
feel free to use it for your own library as a template.

```shell
libmake --csv tests/data/mylibrary.csv
```

To use the `wiserone` library in your project, add the following to your
`Cargo.toml` file:

```toml
[dependencies]
wiserone = "0.0.1"
```

Add the following to your `main.rs` file:

```rust
extern crate wiserone;
use wiserone::*;
```

then you can use the functions in your application code.

### Examples

To get started with `wiserone`, you can use the examples provided in the
`examples` directory of the project.

To run the examples, clone the repository and run the following command
in your terminal from the project root directory.

```shell
cargo run --example example
```

## Semantic Versioning Policy 🚥

For transparency into our release cycle and in striving to maintain
backward compatibility, `wiserone` follows [semantic versioning][6].

## License 📝

The project is licensed under the terms of MIT OR Apache-2.0.

## Contribution 🤝

We welcome all people who want to contribute. Please see the
[contributing instructions][4] for more information.

Contributions in any form (issues, pull requests, etc.) to this project
must adhere to the [Rust's Code of Conduct][11].

Unless you explicitly state otherwise, any contribution intentionally
submitted for inclusion in the work by you, as defined in the
Apache-2.0 license, shall be dual licensed as above, without any
additional terms or conditions.

## Acknowledgements 💙

A big thank you to all the awesome contributors of [wiserone][5] for their
help and support.

A special thank you goes to the [Rust Reddit][12] community for
providing a lot of useful suggestions on how to improve this project.

[0]: https://wiserone.com
[2]: http://opensource.org/licenses/MIT
[3]: https://github.com/sebastienrousseau/wiserone/wiserone/issues
[4]: https://github.com/sebastienrousseau/wiserone/wiserone/blob/main/CONTRIBUTING.md
[5]: https://github.com/sebastienrousseau/wiserone/wiserone/graphs/contributors
[6]: http://semver.org/
[7]: https://crates.io/crates/wiserone
[8]: https://docs.rs/wiserone
[9]: https://lib.rs/crates/wiserone
[10]: https://github.com/sebastienrousseau/wiserone/wiserone/actions
[11]: https://www.rust-lang.org/policies/code-of-conduct
[12]: https://www.reddit.com/r/rust/
[13]: https://www.rust-lang.org/learn/get-started

[crates-badge]: https://img.shields.io/crates/v/wiserone.svg?style=for-the-badge 'Crates.io badge'
[divider]: https://kura.pro/common/images/elements/divider.svg "divider"
[docs-badge]: https://img.shields.io/docsrs/wiserone.svg?style=for-the-badge 'Docs.rs badge'
[libs-badge]: https://img.shields.io/badge/lib.rs-v0.0.1-orange.svg?style=for-the-badge 'Lib.rs badge'
[license-badge]: https://img.shields.io/crates/l/wiserone.svg?style=for-the-badge 'License badge'
[made-with-rust-badge]: https://img.shields.io/badge/rust-f04041?style=for-the-badge&labelColor=c0282d&logo=rust 'Made With Rust badge'
