<a name="readme-top"></a>


[![Contributors][contributors-shield]][contributors-url]
[![Forks][forks-shield]][forks-url]
[![Stargazers][stars-shield]][stars-url]
[![Issues][issues-shield]][issues-url]
[![MIT License][license-shield]][license-url]


<h3 align="center">ASL</h3>

  <p align="center">
     [A Simple Language] is a programming language in development written in Rust 
    <br />
    <a href="https://github.com/RedGear-Studio/ASL/issues">Report Bug</a>
    ·
    <a href="https://github.com/RedGear-Studio/ASL/issues">Request Feature</a>
    ·
    <a href="https://github.com/RedGear-Studio/ASL/pulls">Add Features</a>
  </p>
</div>

# VM Architecture/Instruction Set Specification

The VM is a stack-based virtual machine that operates on a custom instruction set. It has a 32-bit address space, 32 registers, and a memory stack-based.

## Registers

### Integers

The VM has 16 registers dedicated to Integers:

- `$zero`: holds the value zero as a constant and cannot be changed (`$0`)
- `$a0` to `$a3`: registers that store integer arguments for function calls (`$1` to `$4`)
- `$t0` to `$t3`: temporary registers that allow modification of their stored value (`$5` to `$8`).
> The value of these registers can change quickly depending on the program needs.
- `$s0` to `$s6`: registers that hold values across function calls, and their value remains unchanged until no longer needed (`$10` to `$16`).
> By this, we mean that if you don't change the value explicitely, the program won't change it.

### Floats

The VM has 16 registers dedicated to Floats:

`$fa0` to `$fa3`: registers that store float arguments for function calls (`$16` to `$19`)
`$ft0` to `$ft3`: temporary registers that allow modification of their stored value (`$20` to `$23`).
> The value of these registers can change quickly depending on the program needs.
`$f0` to `$f7`: registers that hold values across function calls, and their value remains unchanged until no longer needed (`$24` to `$31`).
> By this, we mean that if you don't change the value explicitely, the program won't change it.


## Memory

The memory of the VM is a stack that has no predefined size (maximum 32 GB). It behaves like a stack most of the time with the PUSH (`psh`) and POP (`pop`) instructions. However, you can access data within the memory using the LOAD (`lod`) and STORE (`str`) instructions, which allow you to read and write data from anywhere in the memory. If you use the STORE instruction, it overwrites any data already stored at that address.

## Directives

### .include

This directive let you include/import external file to allows you a multi-files and library system.
Here's an example of how to use the `.include` directive:
> NB: Currently not implemented yet
`.include <filename.asr>`
> When this directive will be implemented, you'll be able to natively import some built-in library such as `stdlib.asr` that hold several useful datas and functions

### .data

This directive defines the data section of your program, which is where you can define variables and allocate memory for them. The data section is typically located at the beginning of the program.

Here's an example of how to use the .data directive:
```
.data
  msg: .string "Hello World"
  int: .i32 -8
  uint: .u32 8
  float: .f32 8.0
```
In this example, we define four variables: `msg` which is a string, `int` which is a signed 32-bit integer, `uint` which is an unsigned 32-bit integer, and `float` which is a 32-bit floating-point number.

### .text

This directive defines the code section of your program, which is where you can write your program's instructions. The code section typically comes after the data section.
To use it, you just have to write `.text` at the beginning of your program
> This directive needs to be after the data section.

### .global or .library

These directives either define the entry-point of your program (`.global`) or define a library without entry-point (`.library`), to use it you need to do it like this:
Here's an example with the global one:
```
.text         ; The directive have to be right after the `.text` one
.global _main ; The function name can differ with your needs
```
Here's an example with the library one:
```
.text    ; The directive have to be right after the `.text` one
.library ; There's no need to add additional data.
```

### .end

This directive is used to end a program. 

## Instruction Set

- `add <register>, <register>, <register>`: Performs addition on the first two registers and stores the result in the third register.
- `sub <register>, <register>, <register>`: Performs subtraction on the first two registers and stores the result in the third register.
- `div <register>, <register>, <register>`: Performs division on the first two registers and stores the result in the third register.
- `mul <register>, <register>, <register>`: Performs multiplication on the first two registers and stores the result in the third register.
- `lor  <register>, <register>, <register>`: Performs the (Logical) OR operation on the first two registers and stores the result in the third register.
- `and <register>, <register>, <register>`: Performs the AND operation on the first two registers and stores the result in the third register.
- `mov <register>, [<data_label> | <int> | <float> | <register>]`: Moves a value to a register, either from the data segment (data_label), an immediate value (int or float), or another register.
- `inc <register>`: Increments the value stored in a register by one.
- `dec <register>`: Decrement the value in a register by one.
- `swp <register>, <register>`: Swaps the values stored in two registers, using `$t0` as a temporary register.
> If there was already a value in `$t0`, it gets overwritten.
- `cmp <register>, <register>`: Compares the value of 2 registers and set a flag in the VM (Eq, Lt, Gt, Ne)
- `jmc <flag>, [<label> | <function_label>]`: Jumps to a function or an inner label if the latest comparison has the same flag as the one in the instruction
- `jmp [<label> | <function_label>]`: Jumps unconditionnaly to a function or an inner label
- `cal <function_label>`: Calls a function by creating a new stack frame and storing the next instruction of the current function as the first element in the stack
- `ret`: Return from a function by jumping to the first element in the stack previously stored by the `cal` instruction
- `sys <immediate>`: Performs a Syscall based on the immediate value, the 
- `lod <register>, <register>`: Loads from memory at the address in the first register to the second register
- `str <register>, <register>`: Stores to memory at the address in the first register the value in the second one
> Overwrite what's already there
- `psh <register>, <register>`: Pushes to the stack the value of the first register and store the address in the second register
- `pop <register>`: Pops from the stack the top value to the register
- `shr <register>, <int>`: Shifts right the value of the register by the given immediate value (int).
- `shl <register>, <int>`: Shifts left the value of the register by the given immediate value (int).

### Existing Syscalls :

- 0: Print integer, the integer is found in `$a0`.
- 1: Print float, the float is found in `$a0`.
- 2: Print string, the string address is found in `$a0`. (The string needs to be null terminated)
- 3: Read integer, store the integer in `$a0`.
- 4: Read float, store the float in `$a0`.
- 5: Read string, store the string on top of the stack and store its address in `$a0`. (The string needs to be null terminated)
- 6: Exit the program, found the exit code in `$a0` (0 for success, 1 for failure).

### Supported types:
- u32: Unsigned int, 32-bit long
- i32: Signed int, 32-bit long
- f32: Floating points number, 32-bit long

### Compare flag:
The Compare flag is a 3-bit register used for comparison operations. It contains three individual flags (Greater Than, Less Than, Equal), each represented by a single bit:

- The Gt flag, representing "Greater Than", is set to 1 if the first compared value is greater than the second. (001)
- The Lt flag, representing "Less Than", is set to 1 if the first compared value is less than the second. (010)
- The Eq flag, representing "Equal", is set to 1 if the two compared values are equal. (100)

> When you use the `jmc` instruction you can use more than these 3 flags. Gte, Lte and Ne can be used, they are just combinations of existing flags.

# Contributing
Thank you for your interest in contributing to our project! We welcome all contributions, whether they be bug fixes, new features, or improvements to the documentation.

To get started, please follow these steps:

- Fork the repository and clone it locally.
- Create a new branch for your changes: `git checkout -b my-new-feature`.
- Make your changes, and be sure to follow our coding conventions and style guide.
- Commit your changes using [conventional commit specifications](https://www.conventionalcommits.org/en/v1.0.0/): `git commit -m "feat(module): add new feature"`.
- Push your changes to your fork: `git push origin my-new-feature`.
- Open a pull request to our repository with a clear description of your changes and why they should be merged.

Once you have submitted your pull request, one of our maintainers will review it and provide feedback. Thank you for helping us make our project better!

# License

Distributed under the MIT and BEERWARE License. See [`MIT-LICENSE`](https://github.com/RedGear-Studio/ASL/blob/main/LICENSE-MIT.md) and [`BEERWARE-License`](https://github.com/RedGear-Studio/ASL/blob/main/LICENSE-BEERWARE.md)for more information.

<p align="right">(<a href="#readme-top">back to top</a>)</p>



## Contact

Twitter Name: [@RedGear Studio](https://twitter.com/RedGearS) 
Email: studio.redgear@gmail.com

Project Link: [https://github.com/RedGear-Studio/ASL](https://github.com/RedGear-Studio/ASL)

<p align="right">(<a href="#readme-top">back to top</a>)</p>

[contributors-shield]: https://img.shields.io/github/contributors/RedGear-Studio/ASL.svg?style=for-the-badge
[contributors-url]: https://github.com/RedGear-Studio/ASL/graphs/contributors
[forks-shield]: https://img.shields.io/github/forks/RedGear-Studio/ASL.svg?style=for-the-badge
[forks-url]: https://github.com/RedGear-Studio/ASL/network/members
[stars-shield]: https://img.shields.io/github/stars/RedGear-Studio/ASL.svg?style=for-the-badge
[stars-url]: https://github.com/RedGear-Studio/ASL/stargazers
[issues-shield]: https://img.shields.io/github/issues/RedGear-Studio/ASL.svg?style=for-the-badge
[issues-url]: https://github.com/RedGear-Studio/ASL/issues
[license-shield]: https://img.shields.io/github/license/RedGear-Studio/ASL.svg?style=for-the-badge
[license-url]: https://github.com/RedGear-Studio/ASL/blob/master/LICENSE.txt