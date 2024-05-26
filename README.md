# g203-rs

![](https://i.imgur.com/s5IKtVe.jpeg)
[Watch it in action.](https://streamable.com/e/jd1erd)

<br>

This is a command-line interface and library for controlling the Logitech G203 Lightsync mouse. It is built in Rust and uses the `g203_lib` library for device control and the `clap` crate for command-line argument parsing.

Only tested in MacBook Pro M1 2020 (Sonoma 14).

## Installation

You need `libusb` installed.

```sh
git clone https://github.com/carlos-menezes/g203-rs.git
cd g203ctl
cargo build --release
```

The executable will be located in the `target/release` directory.

## Usage

### Library

You can see an example on how to use the library in `main.rs`.

### CLI

```sh
Usage: g203ctl <COMMAND>

Commands:
  solid
  breathe
  cycle
  triple
  wave
  blend
  help     Print this message or the help of the given subcommand(s)
```

### Examples

```sh
./g203ctl solid ff0000
./g203ctl blend 10 100
./g203ctl cycle
./g203ctl breathe 00ff00 100 50
```

## License

MIT License

Copyright (c) 2024 Carlos Menezes

Permission is hereby granted, free of charge, to any person obtaining a copy of this software and associated documentation files (the "Software"), to deal in the Software without restriction, including without limitation the rights to use, copy, modify, merge, publish, distribute, sublicense, and/or sell copies of the Software, and to permit persons to whom the Software is furnished to do so, subject to the following conditions:

The above copyright notice and this permission notice shall be included in all copies or substantial portions of the Software.

THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY, FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM, OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE SOFTWARE.
