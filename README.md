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
     Experimental language build with Rust to make it fast and robust 
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



<!-- TABLE OF CONTENTS -->
<details>
  <summary>Table of Contents</summary>
  <ol>
    <li>
      <a href="#about-the-project">About The Project</a>
      <ul>
        <li><a href="#built-with">Built With</a></li>
      </ul>
    </li>
    <li>
      <a href="#getting-started">Getting Started</a>
      <ul>
        <li><a href="#prerequisites">Prerequisites</a></li>
        <li><a href="#installation">Installation</a></li>
      </ul>
    </li>
    <li><a href="#usage">Usage</a></li>
    <li><a href="#roadmap">Roadmap</a></li>
    <li><a href="#contributing">Contributing</a></li>
    <li><a href="#license">License</a></li>
    <li><a href="#contact">Contact</a></li>
    <li><a href="#acknowledgments">Acknowledgments</a></li>
  </ol>
</details>



<!-- ABOUT THE PROJECT -->
## About The Project

<!-- [![Product Name Screen Shot][product-screenshot]](https://example.com) -->



<p align="right">(<a href="#readme-top">back to top</a>)</p>



### Built With

- [Rustlang](https://www.rust-lang.org/)
- [Pest.rs](https://pest.rs) (The Elegant Parser)

<p align="right">(<a href="#readme-top">back to top</a>)</p>



<!-- GETTING STARTED -->
## Getting Started

This is an example of how you may give instructions on setting up your project locally.
To get a local copy up and running follow these simple example steps.

### Prerequisites

- The compiler and runtime of this language. For now there is no version available, but you can clone this repository and compile yourself !

### Installation

1. Clone the repo
   ```sh
   git clone https://github.com/RedGear-Studio/Reg-Lang.git
   ```
2. Later...

<p align="right">(<a href="#readme-top">back to top</a>)</p>



<!-- USAGE EXAMPLES -->
## Usage
> Reg-Byte usage not the upcoming Reg-Lang syntax

**If/Else:**
```ocaml
STORE $0 #5
STORE $1 #10
EQ $0 $1
STORE $2 #24
JMPE $2
STORE $4 #10
JMPF $4
STORE $3 #20
STORE $5 #4
JMPF $5
STORE $3 #10
HLT
```

**While loop:**
```ocaml
STORE $0 #5
STORE $1 #10
STORE $2 #1
STORE $3 #17
LT $0 $1
ADD $0 $2 $0
JMPE $3
HLT
```


<p align="right">(<a href="#readme-top">back to top</a>)</p>



<!-- ROADMAP -->
## Roadmap

### Upcoming features:

- Simple enough to make learning programming easy, yet robust enough to run complex software.
- Statically typed with type inference
- Live Programming (you can make changes while the program is running).
- A new, simpler type of Debugger for the programmer (based on the Lisp one).
- Add and remove Native/C/Rust libraries at runtime with the LCF pattern and FFI.
- Two types of compilation `executable` (.exe for Windows) and `.rbg` (all platform)
  - The first compile the Rust VM with the program already inside => Lot of optimisation (we can remove all the useless instruction from the VM loop)
  - The second compile to a binary file who contains all the instruction of the program. A .rbg can be read on any platform who got the standard VM.

### Current and in development features:

- Basic arithmetic with one register for the remainder of a division.
- Support for three numeric types : `Int`, `Float`, `UInt` (all in 64 bits)
- Comparison with one register to store the result
- Bytecode compiled
- Register-based Interpreter.

See the [open issues](https://github.com/RedGear-Studio/Reg-Lang/issues) for a full list of proposed features (and known issues).

<p align="right">(<a href="#readme-top">back to top</a>)</p>



<!-- CONTRIBUTING -->
## Contributing

You can help us improve this project by proposing changes through our [Discord server](https://discord.gg/zQfaTBAXg4) or the [Issues section](https://github.com/Gipson62/Reg-lang/issues) of our GitHub page. We welcome all suggestions and encourage you to submit [pull requests](https://github.com/Gipson62/Reg-lang/pulls) if you have developed content to contribute.

Our [Discord server](https://discord.gg/zQfaTBAXg4) is a place where contributors can discuss ideas and collaborate on the project. The [Issues section](https://github.com/Gipson62/Reg-lang/issues) on GitHub is where people can report bugs or propose new features for the project.

We appreciate any and all contributions, and we are excited to see what the community can come up with to help us improve our experimental language

<p align="right">(<a href="#readme-top">back to top</a>)</p>



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