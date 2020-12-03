# advent2020
My solutions for [Advent of Code 2020](https://adventofcode.com/2020/about). 
Doing it in Rust to learn the language, I expect much of this will be dirty af,
and not particularly idiomatic. I'm just trying to get a grasp on the language.

To keep things simple, I'm just setting up each day as it's own project, 
```
cargo new dayXX
```

Because this is just a learning project, I am not bothering to make the code
clean, and I do not to intend on going back and fixing previous days as I find
better ways of doing things (assuming I will.)
Although I will try to be consistent, there may be some changes as I figure out
how to do things. Most days will be set up to simply take the name of the input
as a commandline arguement. Below is a list of exceptions.
```
cd dayXX && cargo run input/input.txt
```
`day01` - `cargo run < input/input.txt`
