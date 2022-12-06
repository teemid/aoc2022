use std::env;
use std::fs;
use std::collections::HashSet;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        println!("Missing arguments. Usage [filename]");
        return;
    }

    let filename = &args[1];
    let content = fs::read_to_string(filename).unwrap();
    let start_of_packet_marker = find_marker(&content, 4);
    println!("The start of packet marker is at position {start_of_packet_marker}");

    let start_of_message_marker = find_marker(&content, 14);
    println!("The start of message marker is at position {start_of_message_marker}");
}

fn find_marker(content: &String, window_size: usize) -> i32 {
    for i in 0..content.len()-window_size {
        let slice = &content[i..i+window_size];
        let set: HashSet<char> = HashSet::from_iter(slice.chars());

        if set.len() == window_size {
            return (i + window_size).try_into().unwrap();
        }
    }

    return -1;
}
