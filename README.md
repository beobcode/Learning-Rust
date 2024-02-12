# Learning Rust

This is my learning Rust repository.

Each project subdirectory will have some kind of goal to focus on and learn.

## Learning Project 1

- Learning goals
  - Learning the basics of setting up a Rust project
  - Learning to reduce the size of the final compiled file
  - Learn how to setup a simple program

- What I've learned
  - How to set up a simple Rust project using `cargo new [my_project_name]`, and
    using the `--vcs none` flag will not create a Git repo for the project
  - compiling the rust project using using cargo command
    - cargo build # used during development
    - cargo build -- release # used for release build
  - Rust seem to compile to large for a simple program
  - Learned to reduce the size of the final compiled file from 4.1MB to 114KB
    using the steps in
    [Minimizing Rust Binary Size](https://github.com/johnthagen/min-sized-rust)
    - To really shrink the binary size I used [upx](https://github.com/upx/upx)
  - Each program has to have a main function where program is run from

## Learning Project 2

- Learning goals
  - Learn how to assign variables
  - Learn types of variables I have access to
  - Learn to pass and return to and from a function

- What I've learned
  - Rust is statically typed language which means the complier needs to know the
    type on compile
  - You can have the complier infer the type without define the type
  - to define a variables use `let` or `const`
    - let defines a mutable variables
    - const defines an in mutable variable
  - The syntax is `let [variable name]: [variable type] = [value]`
  - You can use a pointer by putting `&`. This creates a immutable reference to
    the value
  - The `&` is also used in `&mut` aka "mutable borrower" this make a mutable
    reference so you can change the value
    - You can have only one mutable borrow per value at a time
  - Using comments '//' and multy line comments start '/_' and end with '_/'

<details>
<summary>
Excises
(Provided by googles Gemini)
</summary>
# Challenge: Define two functions:
    1. calculate_age_in_days(birth_year: u16): This function should take a birth year as input and return the user's age in days, assuming the current year is 2024. Use variables to store the birth year and current year. Remember to account for leap years!
    2. greet_by_name(name: &str, age: u32): This function should take a name and age as input and print a personalized greeting like "Hello, Alice! You are 30 years old."
# Bonus: Enhance the functions by:
    - Adding error handling for invalid inputs (e.g., incorrect birth year or negative age).
    - Allowing the calculate_age_in_days function to accept dates instead of just years.
    - Expanding the greet_by_name function to accept different greeting messages based on age ranges.
</details>

## Learning Project 3

- Learning goals
  - Learn about tuples
  - Learn about arrays
  - condition statements "if"
  - Learn loops
    - loop
    - while
    - for..in
