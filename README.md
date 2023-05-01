<a name="readme-top"></a>


[![Contributors][contributors-shield]][contributors-url]
[![Forks][forks-shield]][forks-url]
[![Stargazers][stars-shield]][stars-url]
[![Issues][issues-shield]][issues-url]
[![MIT License][license-shield]][license-url]


<h3 align="center">Reg Lang</h3>

  <p align="center">
    A simple and in development programming language written in Rust.
    <br />
    <a href="https://github.com/RedGear-Studio/Reg-Lang/issues">Report Bug</a>
    ·
    <a href="https://github.com/RedGear-Studio/Reg-Lang/issues">Request Feature</a>
    ·
    <a href="https://github.com/RedGear-Studio/Reg-Lang/pulls">Add Features</a>
  </p>
</div>

# VM Architecture/Instruction Set Specification

The VM is a stack-based virtual machine that operates on a custom instruction set. It has a 32-bit address space, 16 registers, and a memory stack-based.

## Registers

The VM has 16 general-purpose registers, named `reg0` to `reg15` (`reg0` is the zero register, it's read-only and contains the value 0), each with a width of 32 bits. It also has a temporary register named `t_reg`, which is used by some instructions as a temporary storage location.

## Memory

The memory of the VM is a stack but with a size not defined (32 GB max). It behaves like a stack most of the time with the PUSH (psh) and POP (pop) instructions, but if needed, you can access to inner data using the LOAD (lod) and STORE (str) instructions, these instructions let you read and write wherever you want in the memory (when you use str, you overwrite what's at that address).

## Instruction Set

The following instructions are available in the VM:

- `nop`: (Nope) Do nothing.

- `add, t, reg1, reg2, reg3`: (Addition) Add the values in reg1 and reg2, and store the result in reg3. Interpret reg1 and reg2 as t.

- `sub, t, reg1, reg2, reg3`: (Subtraction) Subtract the value in reg1 from the value in reg2, and store the result in reg3. Interpret reg1 and reg2 as t.

- `mul, t, reg1, reg2, reg3`: (Multiplication) Multiply the values in reg1 and reg2, and store the result in reg3. Interpret reg1 and reg2 as t.

- `div, t, reg1, reg2, reg3`: (Division) Divide the value in reg2 by the value in reg1, and store the result in reg3 if reg2 value == 0, the program will return an error and stop. Interpret reg1 and reg2 as t.

- `inc, reg1`: (Increment) Increment the value in reg1 by 1.
> Can only be done on integers and unsigned integers

- `dec, reg1`: (Decrement) Decrement the value in reg1 by 1.
> Can only be done on integers and unsigned integers

- `swp, reg1, reg2`: (Swap) Swap the values of reg1 and reg2 by using the temporary register t_reg.

- `mov, reg1, im`: (Move) Move a 16-bits immediate value to reg1.

- `nxt, reg1, im`: (Next) Shift left reg1 by 16 bits and add a 16-bits immediate value to reg1.

- `lod, t, reg1, reg2`: (Load) Load the value at the memory address found in reg1 in reg2. Interpret the value as t.

- `str, t, reg1, reg2`: (Store) Store the value in reg2 at the memory address found in reg1 (if there's already something at that address, it's overwrited). Interpret the value as t.

- `and, reg1, reg2, reg3`: (And) Perform a bitwise AND operation on the values in reg1 and reg2, and store the result in reg3.

- `or, reg1, reg2, reg3`: (Or) Perform a bitwise OR operation on the values in reg1 and reg2, and store the result in reg3.

- `psh, t, reg1, reg2`: (Push) Push the value from reg1 to the top of the stack and get its address in reg2. Interpret the value as t.

- `pop, t, reg1`: (Pop) Pop the value from the top of the stack to reg1. Interpret the value as t.

- `cal, reg1`: (Call) Change the program counter to the value in reg1 and create a new stack frame.

- `ret`: (Return) Return from a function by using the bottom value of the stack frame as the address of where this function was called. 
> Need to rework this, because if something in the stack needs to be return, you can't remove it from the stack.

- `cmp, reg1, reg2`: (Compare) Compare the values in reg1 and reg2 and set the cmp_register based on the result.

- `cst, t1, t2, reg1`: (Cast) Cast the value in reg1 from the first type (t1) to the seconde one (t2).

- `jmp, reg1`: (Jump) Jump to the address specified in reg1. (Jump already moove the program counter by default to the new address)

- `jmc, cmp_flag, reg1`: (Jump Compare) Jump to the address contained in reg1 if a certain flag is set in the cmp_register. (Jump already moove the program counter by default to the new address)

- `ini, custom_instruction`: (Init) Initialize a custom instruction by pushing all the args registers used by that custom instruction to the stack to avoid losing data.
> Initialize automatically based on the instruction definition.
> NB: Custom instructions aren't implemented yet

- `cle, arg1, reg1`: (Clear) Move the value in arg1 to reg1 as the result value, then pop all the args registers from the stack to restore the previous state of registers.
> Clear automatically based on the instruction definition.
> NB: Custom instructions aren't implemented yet

- `custom_instruction, reg1, reg2, ...`: (Custom Instruction) Perform a custom instruction, similar to a function call, but with the possibility to easily add arguments and without creating a new stack frame.
> Nota Bene: A custom instruction call is coded on 64 bits, the first 32 bits are used to store the address of the custom instruction and the number of arguments, and the second 32 bits are used to store the used registers.
> NB: Custom instructions aren't implemented yet

- `sys, im`: (Syscall) Call the system function identified by the immediate value.

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

Distributed under the MIT and BEERWARE License. See [`MIT-LICENSE`](https://github.com/RedGear-Studio/Reg-Lang/blob/main/LICENSE-MIT.md) and [`BEERWARE-License`](https://github.com/RedGear-Studio/Reg-Lang/blob/main/LICENSE-BEERWARE.md)for more information.

<p align="right">(<a href="#readme-top">back to top</a>)</p>



## Contact

Twitter Name: [@RedGear Studio](https://twitter.com/RedGearS) 
Email: studio.redgear@gmail.com

Project Link: [https://github.com/RedGear-Studio/Reg-Lang](https://github.com/RedGear-Studio/Reg-Lang)

<p align="right">(<a href="#readme-top">back to top</a>)</p>

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