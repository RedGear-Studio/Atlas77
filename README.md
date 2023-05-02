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

The VM is a stack-based virtual machine that operates on a custom instruction set. It has a 32-bit address space, 16 registers, and a memory stack-based.

## Registers

The VM has 16 general-purpose registers, named `reg0` to `reg15` (`reg0` is the zero register, it's read-only and contains the value 0), each with a width of 32 bits. It also has a temporary register named `t_reg`, which is used by some instructions as a temporary storage location.

## Memory

The memory of the VM is a stack but with a size not defined (32 GB max). It behaves like a stack most of the time with the PUSH (psh) and POP (pop) instructions, but if needed, you can access to inner data using the LOAD (lod) and STORE (str) instructions, these instructions let you read and write wherever you want in the memory (when you use str, you overwrite what's at that address).

## Instruction Set

> No documentation available yet.

### Existing Syscalls :

- 0: Print integer, the integer is found in reg1.
- 1: Print float, the float is found in reg1.
- 2: Print string, the string address is found in reg1. (The string needs to be null terminated)
- 3: Read integer, store the integer in reg1.
- 4: Read float, store the float in reg1.
- 5: Read string, store the string on top of the stack and store its address in reg1. (The string needs to be null terminated)
- 6: Exit the program, found the exit code in reg1 (0 for success, 1 for failure).

### Types for the instructions:
> This is the value of a type in the VM. The type is used to know how to interpret the value in a register.
- 0: `u8`
- 1: `u16`
- 2: `u32`
- 3: `i8`
- 4: `i16`
- 5: `i32`
- 6: `f32`
- 7: `char`

### cmp_flag:
The cmp_flag is a 4-bit register used for comparison operations. It contains four individual flags, each represented by a single bit:

- The Neq flag, representing "Not Equal", is set to 1 if the two compared values are not equal.
- The Gt flag, representing "Greater Than", is set to 1 if the first compared value is greater than the second.
- The Lt flag, representing "Less Than", is set to 1 if the first compared value is less than the second.
- The Eq flag, representing "Equal", is set to 1 if the two compared values are equal.

During a comparison operation, one or more of these flags will be set based on the result of the comparison. For example, if the first value is greater than the second, the Gt flag will be set to 1 and the Neq flag will also be set. These flags can then be used in conjunction with the jmc instruction to control program flow based on the outcome of the comparison.

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