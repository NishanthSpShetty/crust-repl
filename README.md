## crust-repl
### C/C++ to Rust interactive transpiler powered by crust

C++ is a system programming language widely used in development of operating system,  firmwares, device drivers and in some application development. Major drawback with C++ or any other system programming language is memory safety, null pointers and dangling pointers, which are very dangerous if not handled properly by the programmer. The new programming language *Rust* is system programming language provides the safe and secure programming with highly enforced compiler restrictions with zero cost abstraction.

------------------------------------------------------------------------

This tool is intended to translate existing C++ code base into Rust with less effort.
May require manual lookup or minute edit to the translated code.

------------------------------------------------------------------------

## Usage

First, make sure you are setup for Rust development. Check out [http://www.rust-lang.org](http://www.rust-lang.org) for more information. The installation sets up the Rust compiler and Cargo package management system. Also, it adds `rustc` and `cargo` commands to your PATH variable.

Now that you're setup for rust. Open any suitable terminal and `cd` into the crust directory

Run `cargo build` to compile the entire project and download some dependencies.

Now, you many test out CRUST using some of the examples in the example folder as follows:
`cargo run`
This will launch a repl where you can input C/C++ construct. It will return equivalent rust code.
Command supported
```
help | :h to display help
exit | :e to exit the repl
clear| :c to clear the console 
```
