# TODO ! Remake the doc to be in the right order !

# Basic Types

This programming language supports several basic types, including signed and unsigned integer types with 8, 16, 32, and 64 bits, as well as floating point types with 32 and 64 bits. It also supports the char type for representing ASCII characters, the bool type for representing boolean values, and the String type for representing strings of text.

## Example

```rs
  // Define signed and unsigned integer variables
  let signedInt8: i8 = -128;
  let signedInt16: i16 = -32768;
  let signedInt32: i32 = -2147483648;
  let signedInt64: i64 = -9223372036854775808;
  let unsignedInt8: u8 = 255;
  let unsignedInt16: u16 = 65535;
  let unsignedInt32: u32 = 4294967295;
  let unsignedInt64: u64 = 18446744073709551615;

  // Define floating point variables
  let float32: f32 = 0.1;
  let float64: f64 = 0.1;

  // Define character, string and boolean variables
  let character: char = 'a';
  let boolean: bool = true;
  let string: String = "Hello, world!";
```
# Array Types

This programming language also supports several array types, including the DynamicArray type for representing arrays that can grow and shrink at runtime, the StaticArray type for representing arrays with a fixed size, and the EnsembleArray type for representing arrays that do not allow duplicate elements.

## Example

```rs
  // Define array variables
  let dynamicArray: DynamicArray(i32) = DynamicArray(i32)(1, 2, 3, 4, 5);
  let staticArray: StaticArray(i32, 5) = StaticArray(i32, 5)(1, 2, 3, 4, 5);
  let ensembleArray: EnsembleArray(i32) = EnsembleArray(i32)(1, 2, 3, 4, 5);
```

# ECS Types

In addition to the basic and array types, this programming language also supports specific types for implementing the entity-component-system (ECS) pattern. These types include the entity type for representing entities in the ECS system, the component type for representing components in the ECS system, and the system type for representing systems in the ECS system.

## Example 

```rs
// Define the Position component, which represents the position of an entity
component Position {
    key x: f32 = 0.0;
    key y: f32 = 0.0;
    key z: f32 = 0.0;
}

// Define the MoveSystem system, which updates the position of an entity
system MoveSystem {
    // The update() function is called automatically by the ECS system
    // It takes a list of entities as an argument, and it updates the entities' positions
    func update(entities: DynamicArray<entity>) {
        for entity in entities {
            // Get the Position component of the entity
            let position:Position = entity.getComponent(Position);

            // Update the position of the entity
            position.x += 1.0;
            position.y += 2.0;
            position.z += 3.0;
        }
    }
}
// Define the entities and include the Position component
entity EntityExample {
    include Position;
}


func main() {
    // Create a list of entities that have the Position component
    let entities:DynamicArray<entity> = DynamicArray(entity);
    entities.push(new EntityExemple);
    entities.push(new EntityExemple);

    // Update the entities using the MoveSystem system
    MoveSystem.update(entities);

    // Get the updated positions of the entities
    let position1:Position = entities[0].getComponent(Position);
    let position2:Position = entities[1].getComponent(Position);
    println "Entity 1's position:" + position1.x + position1.y + position1.z;
    println "Entity 2's position:" + position2.x + position2.y + position2.z;
}
```

# Variables

This programming language supports two keywords for declaring variables: var and let. The var keyword is used to declare a variable without specifying its type, while the let keyword is used to declare a variable with a specific type. When declaring a let variable, you must specify the type of the variable after the variable name and a colon (e.g. let varName: Type = varValue).

## Example

```rs
func main() {
    // Declare a statically typed variable (The type can't change at runtime)
    let hello:String = "Hello";
    // Declare a dynamically typed variable (The type can change at runtinme)
    var world = "world";
    // The type is the same, so I can make a concatenation
    println hello + world;
}
```

# Classes and Structs

This programming language supports the class and struct keywords for defining classes and structs, respectively. Classes and structs are user-defined types that can have attributes and behavior, and they can be used to organize and reuse code in your programs. The 2 main differences between classes and structs is that classes support inheritance, while structs do not and that class can have methods and functions, while structs can't.

## Example

```rs
// Basic struct
struct Position2D {
    // x is statically typed (recommended)
    key x: f32,
    // y is dynamically typed (not recommended)
    key y
}
// Basic class
class Human2D {
    // Statically typed (recommended)
    attr position: Position2D;
    // Dynamically typed (not recommended)
    att name;
    // Constructor of the class
    func init(initPosition: Position2D, initName) {
        self.position = initPosition;
        self.name = initName
    }
}
// A class can be extended
class Doctor2D extend Human2D {
    func init(initPosition:Position2D, initName) {
        // super is a keyword to call a function declared in the base class (here Human2D)
        super init(initPosition, initName);
    }
    func work() -> String {
        return "I'm working";
    }
}

func main() {
    // Create an instance of the Human2D and inside of it, create an instance of the struct Position2D
    let human:Human2D = new Human2D(new Position2D{x: 0.3, y: 0.4}, "David");
    // > David
    println human.name;
    // Create an instance of the Doctor2D
    let doctor:Doctor2D = new Doctor2D(new Position2D{x: 0.6, y: 0.23}, "John");
    // Call the work method from the Doctor2D classe
    // > I'm working
    println doctor.work();
}
```

# Functions

This programming language supports the func keyword for defining functions. Functions are named blocks of code that can be called from other parts of your program in order to perform a specific task. When defining a function, you can specify the types of the function's arguments and return value using the arrow syntax (e.g. func funcName(args) -> Type).

## Example 

```rs
// void function, so don't need to specify the return type
func printHelloWorld() {
    println "Hello World !";
}
// This function return something, so you need to specify it ("-> i32")
// x is statically typed, y dynamically typed, so if y is not a number and Error will be throw
func add(x: i32, y) -> i32 {
    return x + y
}
func main() {
    // > Hello World !
    printHelloWorld();
    // > 13
    println add(6, 7);
    // > Error because you can't add a String and a i32 like that.
    println add(6, "Yo!");
}
```

# Libraries

This programming language supports the use keyword for importing libraries. This allows you to access the data and behavior of the imported library in your code. To import a library, you use the use keyword followed by the name of the library (e.g. use libName).

In addition to the use keyword, this programming language also supports the unuse and clear keywords for disabling and clearing imported libraries, respectively. To disable an imported library, you use the unuse keyword followed by the name of the library (e.g. unuse libName). To clear an imported library, you use the clear keyword followed by the name of the library (e.g. clear libName).

## Example

```rs
// Import the Math lib
use Math;

func main() {
    // > 13
    println Math.add(6, 7);
    // You can use the unuse keyword :
        // Math can't be used again in this scope
        unuse Math;
        // Error, Math Object doesn't exist
        println Math.div(6, 2);
    // Or the clear keyword :
        // Math is free from the memory, the Object "disappear"
        clear Math;
        // Error, Math Object doesn't exist
        println Math.pow(6, 2);
}
```

# Importing Files

This programming language also supports the import keyword for importing files. This allows you to include the contents of another file in your code, which can be useful for organizing and modularizing your programs. To import a file, you use the import keyword followed by the path to the file you want to import (e.g. import filePath).

## Example

> example.rlg
```rs
func printHelloWorld() {
    println "Hello World !";
}
```
> main.rlg
```rs
import "example.rlg" as Example // You need to import the file as an object

func main() {
    // > Hello World !
    Example.printHelloWorld();
    // You can unused and clear an import too with the same keyword as the libs' ones :
        // Example can't be used again in this scope
        unuse Example;
    // Or :
        // Free Example from the memory
        clear Example;
}
```

# Printing and Output

This programming language supports the print and println keywords for printing values to the output without and with a newline at the end, respectively. To print a value without a newline, you use the print keyword followed by the value you want to print (e.g. print String). To print a value with a newline, you use the println keyword followed by the value you want to print (e.g. println String).

## Example

```rs
func main() {
    // > Hello 
    print "Hello ";
    // > Hello World !
    print "World !";
    // > Hello World with println !
    println "Hello World with println !";
}
```

# Entry Point

The entry point of a program in this programming language is the main function. The main function is a void function, meaning that it does not return a value. To define the main function, you use the func keyword followed by the main identifier (e.g. func main()). The main function is called automatically when your program starts, and it is where your program's execution begins.

## Example 

```rs
func main() {
    // All your program logic start here !
}
```

# Conditions

The condition system of this programming language works with 3 keywords: "if", "else", "elif". After an if or an elif statements, there need to be a condition who can return true or false, if the result is true, the language execute what's in the blocks. The else keyword is used when no condition can be fulfill.

## Example

```rs
func main() {
    var number = 35;
    if 32 > number {
        // This bloc is ignored because the result of the condition is false
        println "32 is higher than 35";
    } elif 32 == number {
        // Because the first condition is false, the language goes to check this condition (elif = else if)
        // This bloc is ignored too because the result is false too
        println "32 is equal to 35";
    } else {
        // Because all the conditions are false, this bloc is executed
        println "32 is less than 35";
    }
    // > 32 is less than 35
}
```

# Loops

There are 3 types of loop in this language: while, loop and for. The "loop" loop is an infinite loop like a `while(true)`, the while is an infinite loop whith a condition (while the condition is fulfill, the loop goes on) and the for loop who can iterate over object or be used when you already know how many loop you need to do.

## Example

```rs
func main() {
    var number = 0;
    // If number is higher than 25, the loop will break (so here that'll print 25 times a number between 1 and 26)
    while(number > 25) {
        number = number + 1;
        println number;
    }
    // Do the same as a while loop and if the number is higher than 50 we break the loop (so here that'll print 25 times a number between 27 and 51)
    // You need to manually break a "loop" loop if you want to exit it
    loop {
        if number > 50 {
            break;
        }
        number = number + 1;
        prinln number;
    }
    // Print all the number in this array
    var exampleArray = [0, 1, 2, 3, 4, 5];
    for i in exampleArray.len() {
        println exampleArray[i];
    }
}
```

> Need to rewrite some part of this doc to be up to date with some new features