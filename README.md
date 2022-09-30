# Overview

Because I'm always trying to improve my software skills,
 I decided to pick up Rust and write a quick program to get familiar with the language.

**Corrosion Dice** is a Rust implementation of a virtual dice-rolling program I wrote in Python a few months ago. 
Writing the software in python first was definitely a good idea, since it allowed me to focus on the syntax and Rust-only concepts rather than the structure of the code. The structure was not one-to-one, since Rust does not directly support polymorphism, but I managed to get it working in the end.
The code showcases variables, expressions, conditionals, loops, and functions, as well as some of the techniques that you can use to emulate an object-oriented design structure.

When run, **Corrosion Dice** supplies the user with a minimal command-line-type interface that they can use to roll dice for the role-playing game of their choice, using commands like "roll 20" or "roll 3". Although minimal, this is only the beginning, and I will definitely be coming back to add more functionality.

Watch me demo the software:

[Software Demo Video](http://youtube.link.goes.here)

# Development Environment

For this project, I used VS code with the *Rust Syntax* and *rust-analyzer* extensions. These two extensions working together provided some really helpful debugging and error information, as well as automatic type detection and formatting.

From [the official Rust book](https://doc.rust-lang.org/book/ch00-00-introduction.html): "Rust is for people who crave speed and stability in a language. By speed, we mean the speed of the programs that you can create with Rust and the speed at which Rust lets you write them. The Rust compiler’s checks ensure stability through feature additions and refactoring."
Essentially, Rust is a brutally fast language that removes the risk of segmentation faults and other tough memory problems offered by C++ through a complicated system of ownership and borrowing of variables, arrays, and functions.

For this project, I used the `rand` crate to generate random numbers and the `colored` crate to make the output look nice.
# Useful Websites

{Make a list of websites that you found helpful in this project}
* ["The book" (official rust guide)](https://doc.rust-lang.org/book)
* [Rust By Example](https://doc.rust-lang.org/stable/rust-by-example)
* [Programming-Idioms (specifically Idiom #45)](https://programming-idioms.org/)
* [*Lets Get Rusty*, a Rust tutorial YouTube channel](https://www.youtube.com/c/LetsGetRusty)
# Future Work

Sometime in the future, I would like to further my Rust knowledge by implementing the following features:
* Syntax highlighting for commands
* Multiple color themes ( I think a black/red/white would look really cool, as well as fit with the theme of Rust)
* The option to roll multiple dice at once (I implemented this in the Python version, but didn't have time to add it in this initial release)
* The option to roll any dice with advantage or disadvantage (see [dndbeyond.com](https://www.dndbeyond.com/sources/basic-rules/using-ability-scores#AdvantageandDisadvantage))