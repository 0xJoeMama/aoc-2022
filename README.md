# Advent of Code 2022
So, since I really like Rust as a programming language, I decided to do this year's Advent of Code using vanilla Rust.

When I say vanilla I mean without any external dependencies, unless I really need them(generally that is the way I like to program in).
Thus, I spent the days of December solving Advent of Code. To be honest, I believe AoC is one of the best ways to learn a new programming language.

I not only wrote my solutions, but a full fledged library as well as some codegen(using macros) that I use in all my solutions.
This was a fun project and I loved most of it(except when I was stuck debugging for 2 hours, until I finally realised I just had a sign flipped ;)).

Enjoy!

## Running
You need to have 'cargo' and 'rustc' installed. Other than that, I don't think you need anything else.

Make sure your input is located in the root of the workspace and that it's name is 'input-day-xx.txt' where 'xx' is your target day.
Now, just run 'cargo run --bin=day-xx'.
