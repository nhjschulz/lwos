# lwos
Light Weight OS Library for Rust

[![License](https://img.shields.io/badge/license-MIT-blue.svg)](http://choosealicense.com/licenses/mit/)
[![Repo Status](https://www.repostatus.org/badges/latest/wip.svg)](https://www.repostatus.org/#wip)

# Motivation
Not for external consumption, used to learn about Rust at this stage.

Goal is a simple framework for micro controller applications.
 -> no heap
 -> no std

# Installation of Rust toolchain
## Linux
Follow the installation instruction on [https://rustup.rs/](https://rustup.rs/) to install Rust.

## Windows
1. Download and start the ```rustup-ini.exe``` from [https://rustup.rs/](https://rustup.rs/).
2. By default, the compiler and tools of MSVC toolchain will be installed. For using the GNU toolchain, you have 2 options:
   1. Select target triple ```x86_64-pc-windows-gnu``` during installation for using GNU/MinGW-w64 toolchain. It will be downloaded and installed.
   2. For using a already installed MSYS2 toolchain, create a ```.cargo/config``` file in your user directory, e.g. ```C:\Users\<UserName>\.cargo\config ```.
        ```ini
        [target.x86_64-pc-windows-gnu]
        linker = "C:\\msys2\\mingw64\\bin\\gcc.exe"
        ar = "C:\\msys2\\mingw64\\bin\\ar.exe"
        ```
### See Also

* [rustup/installation/windows](https://rust-lang.github.io/rustup/installation/windows.html)
* [rustup/installation/other](https://rust-lang.github.io/rustup/installation/other.html)

## VSCode
1. Open the `lwos.code-workspace` file using VSCode. If prompted to open the associated workspace, accept.
2. Install the [rust-analyzer extension](https://marketplace.visualstudio.com/items?itemName=rust-lang.rust-analyzer) for Rust support. It may be suggested by VSCode on opening the workspace.
