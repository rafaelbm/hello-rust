# Hello-Rust

## Installation in WSL (Windows Subsystem for Linux)

Install pre-reqs:
> sudo apt install build-essential # Install pre-reqs

In wsl:
> curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh

[More info](https://www.rust-lang.org/tools/install)

## Why?

I decided to try Rust because i've heard alot about how hard it is to program in rust and also how annoying the compiler was.  

My first impressions are OK, i learn Rust by following a video tutorial so it was not a big challenge, but still something's feels a little bit odd, like the data type names but i believe that's because i lack practice with the language.  

Maybe in the future i could try to develop a side project using rust this way i could feel a real-world experience.

## Highlights

- Powerfull enumerations types as you can see [here](./src/enumerations.rs)
- Match statements (Pattern matching) as you can see [here](./src/match_statement.rs)
- Option type (Maybe) baked in the language as you can see [here](./src/options.rs)
- Collection of methods high order functions that resemble C# LINQ as you can see [here](./src/high_order_fun.rs)
- `println!` macro is very hand, mainly the `println!("{:?}", ...);` syntax that can output arrays/vectors/options nicely 