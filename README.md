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

# Reg-Lang
Reg-Lang is a statically-typed programming language currently in development. 
> Currently a Tree Walker Interpreter.

The language is currently in its early development stage and supports basic programming constructs like variables, control flow statements (while/for loops, if/else), string manipulation, and print statements. The language supports three primitive data types: bool, int, and string.

We have two roadmaps for future releases: [`v0.0.2`](https://github.com/RedGear-Studio/Reg-Lang/milestone/2) and [`v0.0.3`](https://github.com/RedGear-Studio/Reg-Lang/milestone/3), which will include exciting features such as support for arrays, functions, and structures, addition of an AST optimizer, REPL and basic stdlib. We're excited to continue developing the language and look forward to your feedback and contributions

> Note: Reg-Lang is still in development, and more features are planned for future releases. Stay tuned for updates!
> For now, only the Parser and Lexer are implemented.

# Installation/Usage
> For now there isn't any real way to use the language because there isn't any REPL or CLI. But it's planned for the `v0.0.2` release.

# Features
## Variables
Variables are declared using the `let` keyword. The type of the variable is declared after the variable name, and the value is assigned using the `=` operator. The following example declares a variable `x` of type `int` and assigns it the value `5`.
You can change its value later on by using the `=` operator again.
```rs
let x: int = 5;
x = 10;
```
## Control Flow
### If/Else
The `if` statement is used to execute a block of code if a condition is true. The `else` statement is used to execute a block of code if the condition is false. The following example prints `x is greater than 5` if the value of `x` is greater than 5, and `x is less than or equal to 5` otherwise.
```rs
if (x > 5) then
    print "x is greater than 5";
else
    print "x is less than or equal to 5";
end;
```
### While Loop
The `while` statement is used to execute a block of code repeatedly until a condition is false. The following example prints the numbers from 1 to 5.
```rs
let x: int = 1;
while (x <= 5) do
    print x;
    x = (x + 1);
end;
```
### For Loop
The `for` statement is used to execute a block of code repeatedly for a fixed number of times. The following example prints the numbers from 1 to 5.
```rs
for x in 1..5 iterate
    print x;
end;
```

## Types
### Integers
Integers are 64-bit signed integers. The following example declares an integer variable `x` and assigns it the value `5`.
```rs
let x: int = 5;
```
### Strings
Strings are sequences of characters. The following example declares a string variable `s` and assigns it the value `"Hello, World!"`.
```rs
let s: string = "Hello, World!";
```
### Booleans
Booleans are either `true` or `false`. The following example declares a boolean variable `b` and assigns it the value `true`.
```rs
let b: bool = true;
```
## Nota Bene
The binary operations need to be surrounded by parentheses. For example, the following code will not compile:
```rs
let x: int = 5;
let y: int = 10;
let z: int = x + y * 2;
```
The following code will compile:
```rs
let x: int = 5;
let y: int = 10;
let z: int = ((x + y) * 2);
```
> It's planned to remove this limitation in the future.

## Contributing
Thank you for your interest in contributing to our project! We welcome all contributions, whether they be bug fixes, new features, or improvements to the documentation.

To get started, please follow these steps:

- Fork the repository and clone it locally.
- Create a new branch for your changes: `git checkout -b my-new-feature`.
- Make your changes, and be sure to follow our coding conventions and style guide.
- Commit your changes using [conventional commit specifications](https://www.conventionalcommits.org/en/v1.0.0/): `git commit -m "feat(module): add new feature"`.
- Push your changes to your fork: `git push origin my-new-feature`.
- Open a pull request to our repository with a clear description of your changes and why they should be merged.

Once you have submitted your pull request, one of our maintainers will review it and provide feedback. Thank you for helping us make our project better!

## License

Distributed under the GPL-3.0 License. See [`LICENSE`](https://github.com/RedGear-Studio/Reg-Lang/blob/main/LICENSE) for more information.

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