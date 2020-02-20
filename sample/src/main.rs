use std::collections::HashMap;
use std::env;
use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader; // 1.2.7

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() != 2 {
        println!("Please specify the data file to read.");
        std::process::exit(-1);
    }

    let filename = &args[1];
    let f = File::open(filename).expect("Failed to open file.");
    let mut f = BufReader::new(f);

    let mut line = String::new();

    f.read_line(&mut line)
        .expect("Failed to parse the first line.");

    let first_line = line
        .trim()
        .split(' ')
        .flat_map(str::parse::<u32>)
        .collect::<Vec<_>>();
    let m = first_line[0];
    let n = first_line[1];
    #[cfg(debug_assertions)]
    println!("{} {}", m, n);

    // A hash map of number of slices to the type indices
    let mut selected_types: HashMap<u32, Vec<usize>> = HashMap::new();
    let mut types_selected = 0;
    let mut total_slices = 0;

    let mut buf = vec![];
    for type_idx in 0..n {
        let _num_bytes = f.read_until(b' ', &mut buf).unwrap();
        let num_slices = std::str::from_utf8(&buf)
            .unwrap()
            .trim()
            .parse::<u32>()
            .unwrap();
        #[cfg(debug_assertions)]
        println!("{}", num_slices);
        buf.clear();
    }
}
