# Rust Inverted Index

[**VIDEO PRESENTATION**](https://youtu.be/Hgs0JNseZt0)

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

## Design Decisions

For this project, I wanted to learn Rust, so I went through the [Rust documentation](https://doc.rust-lang.org/) to learn the language. However, because the Inverted Index in COMPSCI 377 was intended as a way to get familiar with C++ and not really relevant to Operating Systems, I added the multithreaded functionality to the program.

I kept the overall structure and main functionality of the program extremely similar to the original, so that comparisons can be made more directly. Under the hood, in the `main.rs` script, everything is handled by three main functions:

`build_inverted_index(input: &String) -> BTreeMap<String, BTreeSet<usize>>)`

Creates and returns an inverted index, which is a map of key value pairs. The key is the word, and the value is a set of the document indexes (0 for the first document, 1 for the second, 2 for the third, etc). The inverted_index variable starts out as an Arc Mutex, and the function spawns a thread for each document that needs to be processed, and joins them all together before returning the inner value of the mutex, which is the inverted_index itself.

`process_file(file: &String, file_index: usize, inverted_index: &Mutex<BTreeMap<String, BTreeSet<usize>>>)`

Loops through every word in a file, processes the word to remove non-alphabetic characters, and makes the whole word lowercase before adding it to the inverted_index. The inverted_index is an Arc Mutex, so it is locked. This makes sure that only one thread can directly add to the index at a time. This was crucial as it prevents race conditions and ensures thread safety. Since Arc Copies are being made of the index each time a thread is spawned, we do not need to unlock the mutex.

`print_inverted_index(inverted_index: &BTreeMap<String, BTreeSet<usize>>)`

Self explanatory, prints out the inverted_index the same way that the original C++ project does.

The other reason I added multithreaded functionality was when I took into consideration that in the real world, inverted indexes are created to store incredibely large amounts of data from _huge_ documents. This means it could take a while to process all of that data, so using multiple threads would help in creating that index in a more effecient manner. This of course meant avoiding race conditions, but thankfully, Rust includes Arc, which is Atomic Reference Counting. This creates a thread-safe pointer, allowing multiple entities to have ownership over the key variables in question. Combining this with mutex's means we can have safe, shared access to our inverted_index variable.

### Additions/differences from the original C++ version

- Rewritten in Rust, much simpler and easier to work with code overall
- Additional processing is done to each word. All non-alphabetical characters are removed, and every character is lowercased
- Multithreaded processing: a new thread is spawned for each file that is being opened and processed
- Overall runs more efficiently than the original implementation
