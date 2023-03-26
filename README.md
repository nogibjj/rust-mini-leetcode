# LeetCode CLI

A simple Rust-based command-line interface (CLI) tool to fetch LeetCode problem and solution URLs by question number or title.

## Installation

Before you begin, make sure you have Rust installed. If you don't have Rust installed, follow the instructions on the [official Rust website](https://www.rust-lang.org/tools/install).

To install the LeetCode CLI tool, follow these steps:

1. Clone this repository:

   ```sh
   git clone https://github.com/nogibjj/rust-mini-leetcode.git
   ```
2. Build the binary:
    ```
    cargo build --release
    ```
3. (Optional) Add the binary to your PATH:
    ```
    cp target/release/leetcode /usr/local/bin/
    ```
    If you prefer not to add the binary to your PATH, you can run the tool using ./target/release/leetcode.

## Usage
To use the LeetCode CLI tool, run the following command:


```
leetcode -q <question_number_or_title>
```
Or:
```
leetcode -a <question_number_or_title>
```
Replace <question_number_or_title> with the appropriate question number or title (e.g., 1 or two-sum). Use -q to get the question URL, and -a to get the answer (solution) URL.

For example:

```
leetcode -q 1
```
This command will output the question URL for the "Two Sum" problem on LeetCode.

```
leetcode -a two-sum
```
This command will output the answer (solution) URL for the "Two Sum" problem on LeetCode.

## Contributing
Pull requests are welcome. For major changes, please open an issue first to discuss what you would like to change.

Please make sure to update tests as appropriate.

## References

* [rust-cli-template](https://github.com/kbknapp/rust-cli-template)
