# File organizer

This is my first rust project! This is a file organizer built in rust, the files needs to be in the root directory.
This project is for [Stardance](https://stardance.hackclub.com/@barseghian_daniel/projects).

## How to set up and run

### Install rust

Install the latest stable version of Rust using `rustup`.

## macOS

1. Open **Terminal**.
2. Run:

   ```sh
   curl --proto '=https' --tlsv1.2 https://sh.rustup.rs -sSf | sh
   ```

3. Follow the prompts, then open a new Terminal window.
4. If you encounter linker errors, install Apple’s command-line tools:

   ```sh
   xcode-select --install
   ```

## Windows

1. Download and run [`rustup-init.exe`](https://rustup.rs/).
2. Follow the prompts.
3. If prompted, install the Visual Studio C++ build tools. Rust needs them to link native programs.

## Linux

1. Open a terminal.
2. Run:

   ```sh
   curl --proto '=https' --tlsv1.2 https://sh.rustup.rs -sSf | sh
   ```

3. Follow the prompts.
4. If you encounter linker errors, install GCC or Clang using your distribution’s package manager. On Ubuntu or Debian, for example:

   ```sh
   sudo apt install build-essential
   ```

## Verify the installation

Open a new terminal or shell and run:

```sh
rustc --version
cargo --version
```

Both commands should print a version number.

### Install the project and run it!

    1. First clone the project
    ``` bash 
    git clone https://github.com/danielbarseghian/organize-file.git
    ``` 
    2. Then cd into it
    ``` bash
    cd organize-file
    ```
    3. Then run it!
    ``` bash
    cargo run -- filenames
    ```
    > You can organize mutliple files by giving the files name then a space then the second filename

## Descirption
This is my first project after completing the rustlings! it was really challenging to go from exercices to full on projects! But it was very fun!

## License

This project is licensed under the MIT License. See the [`LICENSE`](LICENSE) file for the full license text.

