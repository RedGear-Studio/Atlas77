# Atlas-core

``atlas-core`` is a user-friendly and expendable language creation tool that empowers developers to craft their programming languages with ease. Its modular architecture and rich feature set cater to a wide range of language design needs, making it an invaluable resource for both novices and seasoned developers.

At its core, Atlas-core offers a comprehensive suite of tools, including a built-in Abstract Syntax Tree (AST) and the flexibility to define custom grammars based on this foundation. Ensuring the semantic correctness of your programming language is effortless, thanks to the library's semantic checking capabilities.

Tokenization, a fundamental step in language design, is simplified with Atlas-core, streamlining the process of creating languages. For those who seek a higher degree of customization, the library provides the option to build custom ASTs, allowing for the implementation of specific language features and optimizations.

Atlas-core also introduces three distinct Intermediate Representations (IR) - High-Level (HLIR), Mid-Level (MLIR), and Low-Level (LLIR), enabling developers to optimize and compile their languages effectively. The library includes a set of tools for creating custom passes, with a collection of default passes for immediate use.

When it comes to execution, Atlas-core's cross-platform compatibility extends to a simple virtual machine (VM) and x86 architecture on both Linux and Windows. This ensures that your languages can reach a broad audience across different platforms.

NB: This is the goal of the lib, most of the features aren't implemented yet. The project is still in its early stages. Everything should be implemented during one of the 1.0 release.
> The x86 compiler will be implemented for the 2.0 release tho.