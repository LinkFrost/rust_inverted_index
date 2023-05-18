use std::env;
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;
use std::collections::{BTreeMap, BTreeSet};
use std::thread;
use std::sync::{Arc, Mutex};

// Returns a result of opening the lines of a file into a buffer 
// This comes from an example in the official Rust Language Documentation
// https://doc.rust-lang.org/rust-by-example/std_misc/file/read_lines.html
fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>

where P: AsRef<Path>, {
  let file = File::open(filename)?;
  Ok(io::BufReader::new(file).lines())
}

// Print's the inverted index to standard output
fn print_inverted_index(inverted_index: &BTreeMap<String, BTreeSet<usize>>) {
  for (word, file_indexes) in inverted_index {
    print!("{}: ", word);
    
    for file_index in file_indexes {
      print!("{} ", file_index);
    }

    print!("\n");
  }
}

// Opens the input file and loops over every line and word, performing some basic preprocessing before adding it to the inverted index
fn process_file(file: &String, file_index: usize, inverted_index: &Mutex<BTreeMap<String, BTreeSet<usize>>>) {
  if let Ok(lines) = read_lines(file) {
    // Lock the mutex so that only a single thread can operate on the inverted_index directly
    let mut locked_index = inverted_index.lock().unwrap();

    for l in lines {
      if let Ok(line) = l {
        let words = line.split_whitespace();

        for word in words {
          // Each word is stripped of any non-alphabetic characters and is made all lowercase
          let processed_word: String = word
            .chars()
            .filter(|ch| ch.is_alphabetic())
            .map(|ch| ch.to_lowercase().to_string())
            .collect();

          // Check if this word is already in the index. If it is, insert into it's set of file indexes. If not, create a new set with this file index
          if let Some(file_indexes) = locked_index.get_mut(&processed_word) {
            file_indexes.insert(file_index);
          } else {
            let mut file_indexes = BTreeSet::new();

            file_indexes.insert(file_index);
            locked_index.insert(processed_word, file_indexes);
          }
        }
      }
    }
  }
}

// Creates and returns an inverted index after looping through each word in every file provided in the input text file
fn build_inverted_index(input: &String) -> BTreeMap<String, BTreeSet<usize>> {
  // The inverted index is wrapped in an Arc<Mutex<>> to allow synchronization across threads, as well as maintaining thread safety when multiple threads are operating at the same time
  let inverted_index: Arc<Mutex<BTreeMap<String, BTreeSet<usize>>>> = Arc::new(Mutex::new(BTreeMap::new()));

  if let Ok(files) = read_lines(input) {
    let mut file_index = 0;
    let mut thread_handlers = vec![];

    // Loop through each file listed in the inputs 
    for f in files {
      if let Ok(file) = f {
        // Loop through each file specified in the input. Here, each file gets it's own thread to run process_file() in 
        let inverted_index_copy = Arc::clone(&inverted_index);

        let thread_handler = thread::spawn(move || {
          process_file(&file, file_index, &inverted_index_copy);
        });

        thread_handlers.push(thread_handler);
      }

      file_index += 1;
    }

    // Once this is done, join to close all the threads
    for thread_handler in thread_handlers {
      thread_handler.join().expect("There was an error joining thread {thread_handler}");
    }
  }

  // Now that all relevant processing and operations are done on the inverted index, it is unwrapped and the original value (the BTreeMap) is returned
  let inverted_index = Arc::try_unwrap(inverted_index).unwrap();
  let inverted_index = inverted_index.into_inner().unwrap();

  return inverted_index;
}

fn main() {
  let args: Vec<String> = env::args().collect();
  assert_eq!(args.len(), 2, "Usage: cargo run -- <input.txt>");

  let inputs_file = &args[1];
  let inverted_index: BTreeMap<String, BTreeSet<usize>> = build_inverted_index(inputs_file);

  print_inverted_index(&inverted_index);
}
