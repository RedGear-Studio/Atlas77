# Raw Functions in Reg-Lang

The `raw` construct in Reg-Lang allows the programmer to write low-level code directly in the language, bypassing the high-level abstractions provided by the compiler. This is useful for when the programmer needs to perform an operation that cannot be expressed using the built-in language constructs, or when they want more control over the execution of the code.

A raw function in Reg-Lang is defined using the `raw` keyword, followed by the function name, arguments, return type, and the `start` keyword, like so:

```ruby
raw function_name(arg1: type1, arg2: type2):return_type start
    # raw code here
end;
```

The `arg1` and `arg2` parameters are input arguments to the function, with their types specified by `type1` and `type2`. The `return_type` specifies the type of the value returned by the function. Inside the `start` and `end` keywords, the programmer can write code directly in Reg-Lang assembly language, using the available instructions.

For example, the following raw function adds two integers together:

```ruby
raw add(a: int, b: int):int start
    ADD $0, $1, $2
    RET $2
end;
```

Here, `$0` and `$1` are the registers holding the input arguments a and b, respectively. The `ADD` instruction adds the values in `$0` and `$1`, and stores the result in` $2`. The `RET` instruction returns the value in `$2` as the result of the function.

It's important to note that raw functions are not subject to compiler optimizations, and that there is no type checking performed on the return value. It's up to the programmer to ensure that the code inside the `start` and `end` keywords is correct and efficient. Additionally, because the programmer has direct control over memory management, there is a risk of memory leaks or other memory-related issues if the code is not written carefully.

# For Loop in Reg-Lang
The `for` loop construct in Reg-Lang allows the programmer to write code repeatedly for a fixed number of times like in any other languages, but with a new feature. 

For example a normal loop like in all the language:
```ruby
let x: int = 0;
for x to 5 iterate
    print x; # 0, 1, 2, 3, 4
end;
```
But you can do a decreasing loop like this:
```ruby
let x: int = 10;
for x to 5 iterate decreasing
    print x; # 10, 9, 8, 7, 6
end;
```
You can also change the steps of the loop like this:
```ruby
let x: int = 0;
for x to 5 step 1
    print x; # 0, 1, 2, 3, 4
end;
```
Or you can also let the runtime choose wether it should decrease or increase the value of the loop:
```ruby
let x: int = 0;
for x to 5 both 1 step
    print x; # 0, 1, 2, 3, 4
end;
let y: int = 10
for x to 5 both 1 step
    print y; # 10, 9, 8, 7, 6
end;
```

# SVA Scalable Virtual Architecture

## Introduction:

SVA (Scalable Virtual Architecture) is a virtual architecture designed to provide an easy and scalable way to implement a virtual machine. It is inspired by the MIPS architecture but is not a complete implementation of it. Instead, its focus is on providing an easy way to add custom syscalls and coprocessors to the VM.

## Architecture:

SVA consists of a processor, memory, and a set of coprocessors. The processor implements the SVA instruction set, which is based on the MIPS instruction set but with additional instructions for interacting with coprocessors. The memory is divided into several regions, including the main memory and the memory of each coprocessor.

## Coprocessors:

Coprocessors are additional processors that can be added to the VM to perform specialized operations. Each coprocessor has its own instruction set and memory. The SVA instruction set includes instructions for interacting with coprocessors, allowing programs to communicate with them and take advantage of their specialized capabilities.

## Syscalls:

Syscalls are functions that allow programs running on the VM to interact with the host operating system. SVA includes a set of built-in syscalls for performing common operations such as file I/O and network communication. In addition, users can define their own syscalls to add custom functionality to the VM.

## Customization:

SVA is designed to be easily customizable. Users can define their own coprocessors and syscalls to add custom functionality to the VM. In addition, the SVA compiler can be extended to support custom syntax and code generation for new instructions and coprocessors.
Conclusion

SVA provides an easy and scalable way to implement a virtual machine with custom functionality. Its architecture allows for the addition of coprocessors and syscalls, making it ideal for a wide range of applications.