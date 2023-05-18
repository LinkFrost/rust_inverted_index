# Rust Inverted Index

## Overview

This project started out as recreating the Inverted Index project from COMPSCI 377 at UMass Amherst. The original project was done in C++, this implementation is done in Rust. On top of replicating the original functionality of the project, this was also turned into a multithreaded program to allow for faster index creation when dealing with large files.

## How to run it

If you do not have Cargo installed, you can run

`target/debug/rust_inverted_index <input.txt>`

If you have Cargo installed, simply run

`cargo run -- <input.txt>`

The input txt file is of the following format:

```
<file_1>.txt
<file_2>.txt
.
.
.
<file_n>.txt
```

These file names should be the path to the various text files _from the root folder_.

### Example

`cargo run -- test_inputs/pr1_original.txt`

pr1_original.txt:

```
test_inputs/pr1_original/foo1.txt
test_inputs/pr1_original/foo2.txt
```

test_inputs/pr1_original/foo1.txt:

```
this is a test. cool.
```

test_inputs/pr1_original/foo2.txt:

```
this is also a test.
boring.
```

**Output:**

```
$ cargo run -- test_inputs/pr1_original.txt

Finished dev [unoptimized + debuginfo] target(s) in 0.00s
Running `target/debug/rust_inverted_index test_inputs/pr1_original.txt`

a: 0 1
also: 1
boring: 1
cool: 0
is: 0 1
test: 0 1
this: 0 1
```

## How it works

In `main.rs`, the program will open up the input text file and loop over each line. Each line in this input text file represents a document of interest that will be used to create the inverted index. It loops every single word throughout all the files, and after doing some processing to the word, adds it to the inverted index. The index is then printed out to standard output.

This program uses multithreading, so that each document file is handled by a new thread. Locks and mutex's are used to ensure thread synchronization and safety.

### Additions/differences from the original C++ version

- Rewritten in Rust, much simpler and easier to work with code overall
- Additional processing is done to each word. All non-alphabetical characters are removed, and every character is lowercased
- Multithreaded processing: a new thread is spawned for each file that is being opened and processed
- Overall runs more efficiently than the original implementation
