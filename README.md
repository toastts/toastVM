# toastVM
_a painfully uninteresting virtual machine_

## Inspiration
Going through a computer architecture class I learned a bit more about assembly and CPU pipelines. This also led me down a bit of a rabbit hole of reading a ton about compilers, interpreters, and the instruction pipeline. Learning about how computers actually do stuff was pretty rad, and after writing a ton of MIPS assembly code for my class I thought to myself "you know, i could write at least an interpreter for this, the instruction set really isn't that complex" and so I did this. I decided on doing a register-based interpreter because 1) I was working with MIPS and understood CPU registers a lot better, and 2) I had already written a calculator with a tree-based syntax parser, and a stack-based interpreter seemed a bit weird to do because it doesn't have a machine-level analogue that I was learning in class. Also shoutout the Lua VM, bless up Lua.

## What is it
This is a toy VM so obviously don't use it for anything important, but it's otherwise pretty featureful. I started out with the assembler and defining opcodes because that's what I'm most familiar and comfortable with right now, but over time I'd like to extend the VM to have a proper higher level langauge and maybe some more cool features like clustering.

## Tests??
Yeah, I wanted to learn how to write software unit tests and I thought rust's method of testing was really intuitive. I wrote tests for the major modules like the VM functions and the REPL.

## TODO
- maybe use CLAP to make a CLI so I don't have to spin up a VM just to load an assembly file
- Support for concurrency and parallelism once I better understand that stuff
- an actual language that compiles down to the assembly, which would require a way better understanding of language design lol
- program an AI into the VM so it'll be my friend :^)
