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

# Heap for Reg-Lang VM

```rs
pub struct Heap {
    Vec<u8>
}
pub struct SymbolTable {
    HashMap<id, u64> //id(u64) represent a variable, u64 is the address of that variable in the heap
}
```
That's not fully functional, cuz for example, something like an Array is pretty hard to deal with
And I maybe need to add the end of the variable pointer
Or maybe is type too
And even is scope to drop value
And probably something that state which are the areas free in the Heap
Lot of stuff to implement for an heap

# VM Instructions
## .data Section
- `MOV` [destination: (register1 | stack)] [adress (from .data section)]: Load a value from the .data section to destination (register or the top of the stack)
## Mathematical Int Instructions
- `ADD` [register1] [register2] [register3]: Add the value in the register1 to the value in the register2 and store the result in register3
- `SUB` [register1] [register2] [register3]: Subtract the value in the register1 from the value in the register2 and store the result in register3
- `MUL` [register1] [register2] [register3]: Multiply the value in the register1 by the value in the register2 and store the result in register3
- `DIV` [register1] [register2] [register3]: Divide the value in the register1 by the value in the register2 and store the result in register3
- `MOD` [register1] [register2] [register3]: Modulo the value in the register1 by the value in the register2 and store the result in register3
## Mathematical Float Instructions
- `ADF` [register1] [register2] [register3]: Add the value in the register1 to the value in the register2 and store the result in register3 but treat each value as float
- `MLF` [register1] [register2] [register3]: Multiply the value in the register1 by the value in the register2 and store the result in register3 but treat each value as float
- `SBF` [register1] [register2] [register3]: Subtract the value in the register1 from the value in the register2 and store the result in register3 but treat each value as float
- `DVF` [register1] [register2] [register3]: Divide the value in the register1 by the value in the register2 and store the result in register3 but treat each value as float
- `MDF` [register1] [register2] [register3]: Modulo the value in the register1 by the value in the register2 and store the result in register3 but treat each value as float
## Comparison Instruction
- `CMP` [register1] [register2] [type]: Compare the value in register1 with the value in register2 (treat each value as type) and change the compare_flag based on the result (aka equal, less than, greater than)
## Jump Instructions
- `JMC` [register1] [compare_flag]: Jump to the address in register1 if the correct flag is set in the compare_flag (flag: "0": equal, "1": less than, "2": greater than, "3": not equal)
- `JMP` [register1]: Jump unconditionnaly to the address in register1
## Heap Instructions
- `ALC` [register1]: Allocate a block of memory of the size of the value stored in register1 in the heap and return the address of that block in register1
- `DLC` [register1]: Deallocate the block of memory at the address in register1 in the heap
- `SAV` [from: (register1 | pop_register)] [register2] [register3]: Store the value in either register1 or pop_register to the address in register2 (the value inside register3 is the size of the data that will be stored) in the heap
- `RLC` [register1] [register2]: Reallocate the block of memory at the address in register1 with the size of the value stored in register2 in the heap and store the new address in register1
- `GET` [register1]: Get the address of a block of memory based on the id (the id is generate during the compilation and often represent a variable, but instead of using the identifier, it's changed to a usize to be more efficient) in register1 and return the address in register1
- `COP` [register1] [destination: (register1 | stack)]: Transfer the value at the address stored in register1 to the destination (either register1 or top of the stack)
## Stack Instructions
- `PSH` [register1]: Push the value in register1 onto the stack
- `LOD` [register1]: Load a value from the pop_register to register1
- `STO` [register1]: Store the value in register1 onto the pop stack
- `POP` : Remove the value in the top of the stack and store it in the pop_register (special register with "infinite" sized)
- `CMS` : Compare the value in the pop_register and the value in the top of the stack and change the compare_flag based on the result (aka equal, less than, greater than)
- `RET` [bool (if needed to return a value)]: Close a function (leave a scope, so can be used to launch the garbage collector and/or drop the current stack of this scope) and POP the value in top of the stack to be used as the return value if bool is true
## Miscellaneous Instructions
- `CST` [register1] [type1] [type2]: Take the value in register1, treat it as type1, and store it as type2 in the specified register
- `PRT` [from: (register | pop_register)] [type]: Print the value in the register or pop_register by interpreting it based on its type (string, int, float, bool, char, array(only with basic types in it))


- .data section: This section typically contains pre-defined constants and data that are used by the program, such as strings, arrays, and other data structures. Since this memory is read-only, it cannot be modified by the program during execution.

- Register: Registers are a fast, low-level type of memory that can be used to store values for mathematical and logical operations. Registers are typically used for temporary storage of values during computation, and are often limited in size and number.

- Heap: The heap is a more complex type of memory that can be used for dynamic allocation of memory during program execution. Unlike the stack, which typically has a fixed size and is used for temporary storage of values, the heap can grow or shrink in size as needed to accommodate new data. This makes it useful for storing more complex data structures such as arrays, linked lists, and trees.

- Stack: The stack is a LIFO (last-in, first-out) type of memory that is used for temporary storage of values during program execution. The stack typically has a fixed size and is used to store function call frames, local variables, and other data that is only needed for a short period of time. The stack is often used as a symbol table for the current scope, since each function call creates a new frame on the stack with its own set of local variables and parameters.