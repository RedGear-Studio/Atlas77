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
    add reg1, reg2
    ret
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

# VM Architecture/Instruction Set Specification

The VM is a stack-based virtual machine that operates on a custom instruction set. It has a 32-bit address space, 16 registers, and a memory consisting of both heap and stack (both in one).

## Registers

The VM has 16 general-purpose registers, named reg0 to reg15 (reg0 is the zero register, it's read-only and contains the value 0), each with a width of 32 bits. It also has a temporary register named t_reg, which is used by some instructions as a temporary storage location.

## Memory

The memory of the VM is a mix of stack and heap. The size of the memory is currently undefined. It means that you can push and pop values on top of it and access inner memory by address to get their value or to modify it. When you modify a value, you overwrite it, so you need to be careful when using this instruction. Later on, there will be a separate heap and a stack with a defined size of 512KB.

## Instruction Set

The following instructions are available in the VM:

- `[nop]`: Do nothing.

- `[add ~ reg1 ~ reg2]`: Add the values in reg1 and reg2, and store the result in reg2.

- `[sub ~ reg1 ~ reg2]`: Subtract the value in reg1 from the value in reg2, and store the result in reg2.

- `[mul ~ reg1 ~ reg2]`: Multiply the values in reg1 and reg2, and store the result in reg2.

- `[div ~ reg1 ~ reg2]`: Divide the value in reg2 by the value in reg1, and store the result in reg2 if reg2 value == 0, the program will return an error and stop.

- `[swp ~ reg1 ~ reg2]`: Swap the values of reg1 and reg2 by using the temporary register t_reg.

- `[mov ~ reg1 ~ im]`: Move a 7-bit immediate value to reg1.

- `[nxt ~ reg1 ~ im]`: Used to "complete" the value moved in a register, you can only move 7 bits in a register as an immediate value, and that's maybe not enough, so you use this instruction to add the next 7 bits after the ones already in the 32-bit registers.

- `[lod ~ reg1 ~ reg2]`: Load the value at the memory address found in reg1 in reg2.

- `[str ~ reg1 ~ reg2]`: Store the value in reg2 at the memory address found in reg1 (if there's already something at that address, it's overwrited).

- `[and ~ reg1 ~ reg2]`: Perform a bitwise AND operation on the values in reg1 and reg2, and store the result in reg2.

- `[psh ~ reg1 ~ reg2]`: Push the value from reg1 to the top of the stack and get its address in reg2.

- `[pop ~ reg1]`: Pop the value from the top of the stack to reg1.

- `[cal ~ reg1]`: Change the program counter to the value in reg1 and create a new stack frame.

- `[ret]`: Return from a function by using the bottom value of the stack frame as the address of where this function was called.

- `[cmp ~ reg1 ~ reg2]`: Compare the values in reg1 and reg2 and set the cmp_register based on the result.

- `[jmp ~ reg1]`: Jump to the address specified in reg1. (Jump already moove the program counter by default to the new address)

- `[jmc ~ cmp_flag]`: Jump to the address contained in reg1 if a certain flag is set in the cmp_register. (Jump already moove the program counter by default to the new address)

- `[sys ~ im]`: Call the system function identified by the immediate value.

### Existing Syscalls :

- 0: Print integer, the integer is found in reg1.
- 1: Print float, the float is found in reg1.
- 2: Print string, the string address is found in reg1. (The string needs to be null terminated)
- 3: Read integer, store the integer in reg1.
- 4: Read float, store the float in reg1.
- 5: Read string, store the string on top of the stack and store its address in reg1. (The string needs to be null terminated)
- 6: Exit the program, found the exit code in reg1 (0 for success, 1 for failure).

## cmp_flag:
The cmp_flag is a 4-bit register used for comparison operations. It contains four individual flags, each represented by a single bit:

- The Neq flag, representing "Not Equal", is set to 1 if the two compared values are not equal.
- The Gt flag, representing "Greater Than", is set to 1 if the first compared value is greater than the second.
- The Lt flag, representing "Less Than", is set to 1 if the first compared value is less than the second.
- The Eq flag, representing "Equal", is set to 1 if the two compared values are equal.

During a comparison operation, one or more of these flags will be set based on the result of the comparison. For example, if the first value is greater than the second, the Gt flag will be set to 1 and the Neq flag will also be set. These flags can then be used in conjunction with the jmc instruction to control program flow based on the outcome of the comparison.