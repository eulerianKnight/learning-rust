# Key Terms

- Enum: An enum, short for enumerator, is a data type in Rust that allows you to define a set of possible values. It's used to create custom types that represent distinct variants or cases. Enums are powerful because they enable you to encapsulate related values and provide exhaustive matching capabilities through the match keyword.

- Variant: A variant is a specific case within an enum. Each enum can have multiple variants, which may optionally contain associated data. For example, in the text provided, there are enums like Shape, with two variants: Circle and Square. The Circle variant has an associated radius value, while the Square variant does not carry any additional data.

- Match keyword: The match keyword is a powerful control flow construct that enables exhaustive pattern matching against enums or other values. It checks each case to find the first match, making it an excellent way to handle different enum variants in Rust. The text demonstrates using match with enums like Shape, WineRegion, and more.

- Option: The Option type is a built-in enum provided by Rust that represents either the presence or absence of a value, denoted as Some(value) or None. It's often used to handle nullable values in other languages. The text explores using Option, including its unwrapping methods like unwrap().

- Sum type: A sum type is an enumeration that can hold a variant with associated data of some type D, allowing you to create instances containing different types or combinations of data. In Rust, enums are examples of sum types because they let you define multiple variants, each potentially carrying distinct information. The text demonstrates this concept through the Shape enum and its associated data (radius for circles and side length for squares).

- Associated functions/methods: In Rust, it's possible to extend enums with implementation blocks that contain associated functions or methods, just like structs. This enables you to add functionality specific to the enum type directly within the enum definition. The text showcases this by implementing a format_size() method for the FileSize enum.

- Vectors: A vector is a growable array-like data structure in Rust that can hold multiple values of the same type. In the context of enums, vectors are useful when you want to store and manipulate collections of related enum instances or variants. The text demonstrates using vectors with enums by defining a Shapes vector containing different shapes (circles and squares).

- Exhaustive matches: Exhaustive matching is the practice of handling all possible cases when working with enums, ensuring that every variant has been accounted for in your code. Rust's type system helps enforce exhaustiveness by requiring you to handle each enum variant or use a catch-all pattern (_) if needed. The text highlights this concept through examples like the taste() function dealing with wine grape variants.

- Non-exhaustive patterns: When working with enums, non-exhaustive patterns refer to cases where not all possible enum variants are explicitly handled in a match statement or conditional expression. Rust's compiler will warn you about missing arms (unhandled variants) unless you use the catch-all pattern _ or explicitly mark your enum as non-exhaustive using the ... syntax. The text demonstrates this concept through examples like the wine grape enumerator and its associated functions.

- CLI tool: Command Line Interface tool is a type of software that can be used at the command line of an operating system.

- Makefile: A makefile contains targets to compile your code into executable binaries or libraries. It also includes rules for managing dependencies between files and creating distribution packages.

- debugger: Debugging tools help developers identify issues in their programs by stepping through the code, line by line, while monitoring variable values and other aspects of program execution.

- library: A collection of reusable functions, structures, macros, traits, and types. Libraries are distributed as binary crates or source code packages on crates.io.

- Cargo.toml: The configuration file for Cargo that contains metadata about your project such as its name, version, authors, dependencies, and links to documentation.

- buffered reader: A buffered reader is a type of input stream in Rust which reads data from a source (like stdin) into an internal buffer before providing it to the application for processing. This can improve performance by reducing the number of system calls required for reading small chunks of data.

- std::io::Stdin: The standard input handle, allowing you to read from user input or pipes in Rust programs.