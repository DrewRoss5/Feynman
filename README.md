# Feynman
## Summary 
Feynman is a CLI calculator/math interpreter built in Rust. 
### Current Features:
- Expression Evaluation
- Variables
## Roadmap:
- Add support for user-defined functions 
- Add floating point support
# Usage
#### To run Feynman: 
  - Ensure [cargo](https://doc.rust-lang.org/cargo/getting-started/installation.html) is installed
  - Clone this repo
  - Run `cargo run` in this repo's directory
## Command Line Interface:
When you run Feynman, you'll be greeted with a simple CLI prompt. To perform a calculation, simply enter your expression, and Feynman will show you the result.
#### Example:
```
Feyman> 1 + 1
= 2
```
### Brackets:
Feynman supports the usage of brackets to give precedence to specific operations.
#### Example:
```
Fenyman> 2 + 2 * 5
= 12
Feynman> [2 + 2] * 5
= 20
```
Nested brackets for complex expressions are also supported.
### Variables:
Feynman has support for command-line variables.
#### Example:
```
Feynman> let x = 9
= 9
Feynman> let y = 5 + 5
= 10
Feynman> x + y
= 19
Feynman> x * 2
= 18
```
The let command must be used to declare new variables.<br>
Additionally, Variables can be implicitly declared as part of a larger expression
#### Example:
```
Feynman> [let x = 5] + [let foobar = 2 * 2]
= 9
Feynman> x
= 5
Feynman> foobar
= 4
```
