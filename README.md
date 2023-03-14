<!-- Improved compatibility of back to top link: See: https://github.com/othneildrew/Best-README-Template/pull/73 -->
<a name="readme-top"></a>
<!--
*** Thanks for checking out the Best-README-Template. If you have a suggestion
*** that would make this better, please fork the repo and create a pull request
*** or simply open an issue with the tag "enhancement".
*** Don't forget to give the project a star!
*** Thanks again! Now go create something AMAZING! :D
-->



<!-- PROJECT SHIELDS -->
<!--
*** I'm using markdown "reference style" links for readability.
*** Reference links are enclosed in brackets [ ] instead of parentheses ( ).
*** See the bottom of this document for the declaration of the reference variables
*** for contributors-url, forks-url, etc. This is an optional, concise syntax you may use.
*** https://www.markdownguide.org/basic-syntax/#reference-style-links
-->
[![Contributors][contributors-shield]][contributors-url]
[![Forks][forks-shield]][forks-url]
[![Stargazers][stars-shield]][stars-url]
[![Issues][issues-shield]][issues-url]
[![MIT License][license-shield]][license-url]



<!-- PROJECT LOGO -->
<br />
<div align="center">
  <a href="https://github.com/RedGear-Studio/Reg-Lang">
    <img src="images/logo.png" alt="Logo" width="80" height="80">
  </a>

<h3 align="center">Reg Lang</h3>

  <p align="center">
    A simple, fast and experimental VM for Reg-Lang.
    <br />
    <!-- <a href="https://github.com/RedGear-Studio/Reg-Lang"><strong>Explore the docs »</strong></a>
    <br />
    <br />
    <a href="https://github.com/RedGear-Studio/Reg-Lang">View Demo</a>
    · -->
    <a href="https://github.com/RedGear-Studio/Reg-Lang/issues">Report Bug</a>
    ·
    <a href="https://github.com/RedGear-Studio/Reg-Lang/issues">Request Feature</a>
    ·
    <a href="https://github.com/RedGear-Studio/Reg-Lang/pulls">Add Features</a>
  </p>
</div>

# Reg-Lang

This is a simple virtual machine (VM), for the Reg-Lang language, that can execute bytecode instructions. It has 64 registers, each capable of storing 64 bits of binary data, and can execute eight different opcodes: MOV, ADD, JMP, CMP, JMC, PRT, UWU, and HLT.

# How to Use

To use the VM, you will need to write a program in bytecode format. The bytecode is a sequence of instructions encoded as bytes, where each byte corresponds to an opcode or a value to be loaded into a register.

Here is an example program that loads the value 42 into register 0, adds the value 23 to it, and prints the result:

```rs
// Load 42 into register 0
MOV $0 42 //42 will be store in the .data section, so it's gonna be replaced by a pointer to where it's stored

// Load 23 into register 1
MOV $1 23

// Add register 0 and register 1, and store the result in register 2
ADD $0 $1 $2

// Print the value in register 2
PRT $2
/// Note: all the instructions are in 32 bits, so PRT $2 is in reality : PRT $2 0 0
```
To run this program, you can create a new instance of the VM, set its program property to the bytecode sequence, and call the run method:

```rust
use reg_lang_vm::VM;

fn main() {
    let mut vm = VM::new();
    vm.program = vec![
        0x00, 0x00, 0x2a, 0x00,
        0x00, 0x01, 0x17, 0x00,
        0x01, 0x00, 0x01, 0x02,
        0x05, 0x02, 0x00, 0x00,
    ];
    vm.run();
}
```
This will output:

``> 65``

# OpCodes

    `MOV`: Loads a value from the .data section into a register.
    `ADD`: Adds the value in one register to the value in another register and stores the result in a third register.
    `JMP`: Jumps to the address stored in a register.
    `CMP`: Compares the values in two registers and sets a flag based on the result (greater-than, less-than, or equal).
    `JMC`: Jumps to an address if a specified flag is set (e.g. jumps if the previous comparison result was greater-than).
    `PRT`: Prints the value in a register to the console or output stream.
    `UWU`: Change the uwu_flag to print the value in a register in a more uwu way.
    `HLT`: Halts the program.

  ## Incoming OpCodes
    
    `ADF`: Adds the value in one register to the value in another register and stores the result in a third register but both of the register will be treat as Float
    `CST`: Cast a register to one number type to another (like Float -> Int or Int -> UInt) (Because a Register just store bits and nothing else)


<!-- LICENSE -->
## License

Distributed under the GPL-3.0 License. See [`LICENSE`](https://github.com/RedGear-Studio/Reg-Lang/blob/main/LICENSE) for more information.

<p align="right">(<a href="#readme-top">back to top</a>)</p>



<!-- CONTACT -->
## Contact

Twitter Name - [@RedGear Studio](https://twitter.com/RedGearS) - studio.redgear@gmail.com

Project Link: [https://github.com/RedGear-Studio/Reg-Lang](https://github.com/RedGear-Studio/Reg-Lang)

<p align="right">(<a href="#readme-top">back to top</a>)</p>


<!-- MARKDOWN LINKS & IMAGES -->
<!-- https://www.markdownguide.org/basic-syntax/#reference-style-links -->
[contributors-shield]: https://img.shields.io/github/contributors/RedGear-Studio/Reg-Lang.svg?style=for-the-badge
[contributors-url]: https://github.com/RedGear-Studio/Reg-Lang/graphs/contributors
[forks-shield]: https://img.shields.io/github/forks/RedGear-Studio/Reg-Lang.svg?style=for-the-badge
[forks-url]: https://github.com/RedGear-Studio/Reg-Lang/network/members
[stars-shield]: https://img.shields.io/github/stars/RedGear-Studio/Reg-Lang.svg?style=for-the-badge
[stars-url]: https://github.com/RedGear-Studio/Reg-Lang/stargazers
[issues-shield]: https://img.shields.io/github/issues/RedGear-Studio/Reg-Lang.svg?style=for-the-badge
[issues-url]: https://github.com/RedGear-Studio/Reg-Lang/issues
[license-shield]: https://img.shields.io/github/license/RedGear-Studio/Reg-Lang.svg?style=for-the-badge
[license-url]: https://github.com/RedGear-Studio/Reg-Lang/blob/master/LICENSE.txt