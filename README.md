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
Reg-Lang is a statically-typed programming language currently in development. 
> Currently a Tree Walker Interpreter.

The language is currently in its early development stage and supports basic programming constructs like variables, control flow statements (while/for loops, if/else), string manipulation, and print statements. The language supports three primitive data types: bool, int, and string.

We have two roadmaps for future releases: [`v0.0.2`](https://github.com/RedGear-Studio/Reg-Lang/milestone/2) and [`v0.0.3`](https://github.com/RedGear-Studio/Reg-Lang/milestone/3), which will include exciting features such as support for arrays, functions, and structures. We're excited to continue developing the language and look forward to your feedback and contributions

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
    x = x + 1;
end;
```

## Contributing
If you're interested in contributing to Reg-Lang, please consider contributing via [Github-Issues](https://github.com/RedGear-Studio/Reg-Lang/issues) or [Github-Pull Requests](https://github.com/RedGear-Studio/Reg-Lang/pulls).

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