# PA8

This assignment explores the gray zone between compilers and interpreters.
Interpreters generally execute code as it is read. Compilers on the other hand
first process the code to find optimizations, then output a new representation
that can be executed at a later date.

To start this assignment, finish the skeleton code provided in `interp1/src/main.rs`.
You should not need more than 15 lines of code to complete this interpreter.

After testing your interpreter (see `interp1/README.md`), continue on to `interp2`.
As a first step on `interp2`, you will copy your `interp1/src/main.rs` implementation
into the `interp2/src/` folder.

Continue this process through all the `interp` puzzles. None of the implementations
should require more than 10 or so lines changed from the previous interp.

## Grading

Each interp is weighted equally and must pass all the provided `.bf` files.

## Submission

Submit only the interp1.rs, interp2.rs, etc. files.

## Description of each BF implementation

  * `interp1`: the basic BF interpreter.
  * `interp2`: the basic BF interpreter with bracket caching.
  * `interp3`: a BF interpreter with opcodes and bracket caching.
  * `interp4`: a BF interpreter with opcodes and inline bracket caching.
  * `interp5`: a BF interpreter with opcodes, inline bracket caching, and Add
    optimisation.
  * `interp6`: a BF interpreter with opcodes, inline bracket caching, and
    Add/Sub/Left/Right optimisations.
  * `interp7`: a BF interpreter with opcodes, inline bracket caching,
    Add/Sub/Left/Right optimisations, and Zero optimisation.
