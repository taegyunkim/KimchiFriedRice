use std::collections::HashMap;
use std::env;
use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;

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
    let mut sizes_to_indices: HashMap<u32, Vec<usize>> = HashMap::new();
    let mut total_slices = 0;
    let mut largest = 0;

    let mut buf = vec![];
    for idx in 0..n as usize {
        let _num_bytes = f.read_until(b' ', &mut buf).unwrap();
        let slices = std::str::from_utf8(&buf)
            .unwrap()
            .trim()
            .parse::<u32>()
            .unwrap();
        #[cfg(debug_assertions)]
        println!("{}", slices);
        buf.clear();
        if total_slices + slices <= m {
            total_slices += slices;
            #[cfg(debug_assertions)]
            println!("adding {}", slices);

            if let Some(indices) = sizes_to_indices.get_mut(&slices) {
                indices.push(idx);
            } else {
                sizes_to_indices.insert(slices, vec![idx]);
            }

            if slices > largest {
                largest = slices;
            }
        } else {
            let excess = total_slices + slices - m;

            assert_ne!(largest, 0);
            for i in excess..=largest.min(slices) {
                if let Some(indices) = sizes_to_indices.get_mut(&i) {
                    if !indices.is_empty() {
                        indices.pop();

                        total_slices += slices - i;
                        #[cfg(debug_assertions)]
                        println!("adding {} subbing {}", slices, i);

                        if let Some(indices) = sizes_to_indices.get_mut(&slices) {
                            indices.push(idx);
                        } else {
                            sizes_to_indices.insert(slices, vec![idx]);
                        }

                        if slices > largest {
                            largest = slices;
                        }
                        break;
                    }
                }
            }
        }
    }

    let mut types = 0;
    let mut indices: Vec<usize> = Vec::new();
    for (_, v) in sizes_to_indices {
        types += v.len();
        indices.extend(v.iter());
    }
    println!("{}", types);
    println!(
        "{}",
        indices
            .iter()
            .map(|n| n.to_string())
            .collect::<Vec<String>>()
            .join(" ")
    );
}
