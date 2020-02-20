use std::collections::{HashMap, HashSet};
use std::env;
use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader; // 1.2.7

#[derive(Debug, PartialEq, Eq, Copy, Clone)]
enum State {
    SignedUp,
    InProcess(usize),
    NeedsSignUp,
}

#[derive(Debug)]
struct Library {
    idx: usize,
    state: State,
    days_to_signup: usize,
    ship_rate: usize,
    books: Vec<usize>,

    sent: Vec<usize>,
}

impl Library {
    fn new(idx: usize, days_to_signup: usize, ship_rate: usize, books: Vec<usize>) -> Self {
        Self {
            idx,
            state: State::NeedsSignUp,
            days_to_signup,
            ship_rate,
            books,
            sent: Vec::new(),
        }
    }

    fn ship(&mut self, scanned: &mut HashSet<usize>) {
        let mut cnt = 0;

        while self.books.len() > 0 && cnt < self.ship_rate {
            let book = self.books.pop().unwrap();
            if scanned.contains(&book) {
                continue;
            }
            // Also add this book to the scanned set to prevent other libraries from sending the
            // same book.
            scanned.insert(book);
            self.sent.push(book);
            cnt += 1;
        }
    }

    fn process(&mut self, scanned: &mut HashSet<usize>) {
        match self.state {
            State::SignedUp => self.ship(scanned),
            State::InProcess(0) => {
                self.state = State::SignedUp;
                self.ship(scanned);
            }
            State::InProcess(days) => {
                self.state = State::InProcess(days - 1);
            }
            State::NeedsSignUp => {}
        }
    }

    fn state(&self) -> State {
        self.state
    }

    fn signup(&mut self) {
        self.state = State::InProcess(self.days_to_signup);
    }

    fn days_to_signup(&self) -> usize {
        self.days_to_signup
    }

    fn index(&self) -> usize {
        self.idx
    }

    fn sent(&self) -> &[usize] {
        &self.sent
    }

    fn books(&self) -> &[usize] {
        &self.books
    }

    fn signup_approx(
        &self,
        books_to_scores: &HashMap<usize, u32>,
        scanned: &HashSet<usize>,
        to_scan: &HashSet<usize>,
        days: usize,
    ) -> u32 {
        let mut cnt = 0;
        let mut books = self.books.clone();

        let mut score = 0;
        while books.len() > 0 && cnt < (days - self.days_to_signup) * self.ship_rate {
            let book = books.pop().unwrap();
            if scanned.contains(&book) || to_scan.contains(&book) {
                continue;
            }

            score += books_to_scores.get(&book).unwrap();
            cnt += 1;
        }

        score
    }
}

fn signup_one_lib(
    libraries: &mut Vec<Library>,
    books_to_scores: &HashMap<usize, u32>,
    scanned: &HashSet<usize>,
    to_scan: &mut HashSet<usize>,
    days: usize,
) -> Option<usize> {
    let mut to_signup = libraries
        .iter_mut()
        .filter(|lib| lib.state() == State::NeedsSignUp && lib.days_to_signup < days)
        .collect::<Vec<&mut Library>>();

    let lib_or_none = to_signup.iter_mut().max_by(|a, b| {
        a.signup_approx(&books_to_scores, &scanned, &to_scan, days)
            .cmp(&b.signup_approx(&books_to_scores, &scanned, &to_scan, days))
    });

    match lib_or_none {
        Some(library) => {
            library.signup();
            let mut cnt = 0;
            for book in library.books() {
                if cnt < (days - library.days_to_signup) * library.ship_rate {
                    if scanned.contains(book) || to_scan.contains(book) {
                        continue;
                    }
                    to_scan.insert(*book);
                    cnt += 1;
                } else {
                    break;
                }
            }

            Some(library.index())
        }
        None => None,
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() != 2 {
        println!("Please specify the data file to read.");
        std::process::exit(-1);
    }

    let filename = &args[1];
    let f = File::open(filename).expect("Failed to open file.");
    let mut reader = BufReader::new(f);

    let mut line = String::new();

    reader
        .read_line(&mut line)
        .expect("Failed to parse the first line.");

    let first_line = line
        .trim()
        .split(' ')
        .flat_map(str::parse::<usize>)
        .collect::<Vec<_>>();
    line.clear();

    let b = first_line[0];
    let l = first_line[1];
    let d = first_line[2];
    #[cfg(debug_assertions)]
    println!("{} {} {}", b, l, d);

    reader
        .read_line(&mut line)
        .expect("Failed to parse the second line.");
    let book_score_list = line
        .trim()
        .split(' ')
        .flat_map(str::parse::<u32>)
        .collect::<Vec<_>>();
    line.clear();

    #[cfg(debug_assertions)]
    println!("{:?}", book_score_list);

    assert_eq!(book_score_list.len(), b);

    let mut books_to_scores: HashMap<usize, u32> = HashMap::new();
    for (idx, score) in book_score_list.iter().enumerate() {
        books_to_scores.insert(idx, *score);
    }

    let mut libraries = Vec::with_capacity(l as usize);

    for lib_idx in 0..l as usize {
        // Parse the first line of library lib_idx
        reader
            .read_line(&mut line)
            .expect("failed to parse library first line.");

        let lib_header = line
            .trim()
            .split(' ')
            .flat_map(str::parse::<usize>)
            .collect::<Vec<_>>();
        let n = lib_header[0];
        let t = lib_header[1];
        let m = lib_header[2];

        #[cfg(debug_assertions)]
        println!("{} {} {}", n, t, m);

        line.clear();

        // Parse the second line of lbirary lib_idx
        reader
            .read_line(&mut line)
            .expect("failed to parse library second line.");

        let mut books = line
            .trim()
            .split(' ')
            .flat_map(str::parse::<usize>)
            .collect::<Vec<_>>();
        assert_eq!(books.len(), n);

        // Sort the books in decreasing order of the scores.
        books.sort_by(|a, b| {
            books_to_scores
                .get(b)
                .unwrap()
                .partial_cmp(books_to_scores.get(a).unwrap())
                .unwrap()
        });

        #[cfg(debug_assertions)]
        println!("{:?}", books);

        line.clear();

        libraries.push(Library::new(lib_idx, t, m, books));
    }

    // libraries.sort_by(|a, b| a.days_to_signup().partial_cmp(&b.days_to_signup()).unwrap());

    let mut scanned: HashSet<usize> = HashSet::new();
    let mut to_scan: HashSet<usize> = HashSet::new();

    let mut signed: Vec<usize> = Vec::new();

    let mut days_to_signup: i32 = 0;
    for day in 0..d {
        if days_to_signup <= 0 {
            if let Some(idx) = signup_one_lib(
                &mut libraries,
                &books_to_scores,
                &scanned,
                &mut to_scan,
                d - day,
            ) {
                signed.push(idx);
                days_to_signup = libraries[idx].days_to_signup() as i32;
            }
        }

        for library in &mut libraries {
            library.process(&mut scanned);
        }

        days_to_signup -= 1;
    }

    let mut signed_and_sent = 0;
    for idx in &signed {
        for library in &libraries {
            if library.index() == *idx && library.sent().len() > 0 {
                signed_and_sent += 1;
            }
        }
    }
    println!("{}", signed_and_sent);

    for idx in &signed {
        for library in &libraries {
            if library.index() == *idx {
                if library.sent().len() > 0 {
                    println!("{} {}", idx, library.sent().len());

                    println!(
                        "{}",
                        library
                            .sent()
                            .iter()
                            .map(|n| n.to_string())
                            .collect::<Vec<String>>()
                            .join(" ")
                    );
                }
            }
        }
    }
}
