use rand::Rng;
use rayon::prelude::*;
use std::str::Chars;
// use diam::prelude::*; // NEW

fn detect_capital_use(word: &str) -> bool {
    // Constraints: 1 <= word.length <= 100
    let mut it = word.chars();

    // // If else solution
    // let first_letter = it.next().unwrap();

    // if first_letter.is_lowercase() {
    //     it.all(|c: char| c.is_lowercase())
    // } else {
    //     if let Some(c) = it.next() {
    //         if c.is_lowercase() {
    //             it.all(|c: char| c.is_lowercase())
    //         } else {
    //             it.all(|c: char| c.is_uppercase())
    //         }
    //     } else {
    //         true
    //     }
    // }

    // Match solution
    match (it.next(), it.next()) {
        (_, Some('a'..='z')) => it.all(|c: char| c.is_lowercase()),
        (Some('A'..='Z'), Some('A'..='Z')) => it.all(|c: char| c.is_uppercase()),
        (_, None) => true,
        (_, _) => false, // Should never happen
    }
}

fn parallel_all_detect_capital_use(word: &str) -> bool {
    // If the words is small, there is no need to do it in parallel
    if word.len() < 1000 {
        return detect_capital_use(word);
    }

    let mut it: Chars<'_> = word.chars();

    let cl: fn(char) -> bool = match (it.next(), it.next()) {
        (_, Some('a'..='z')) => |c: char| c.is_lowercase(),
        (Some('A'..='Z'), Some('A'..='Z')) => |c: char| c.is_uppercase(),
        (_, _) => |_c: char| false,
    };

    it.as_str()
        .par_chars()
        //.log("all")
        .all(cl)
}

fn parallel_rayon_all_detect_capital_use(word: &str) -> bool {
    // If the words is small, there is no need to do it in parallel
    if word.len() < 1000 {
        return detect_capital_use(word);
    }

    let mut it: Chars<'_> = word.chars();

    let cl: fn(char) -> bool = match (it.next(), it.next()) {
        (_, Some('a'..='z')) => |c: char| c.is_lowercase(),
        (Some('A'..='Z'), Some('A'..='Z')) => |c: char| c.is_uppercase(),
        (_, _) => |_c: char| false,
    };

    #[inline]
    fn is_false(x: &bool) -> bool {
        !x
    }

    it.as_str()
        .par_chars()
        //.log("map")
        .map(cl)
        .find_any(is_false)
        .is_none()
}

fn parallel_map_detect_capital_use(word: &str) -> bool {
    // If the words is small, there is no need to do it in parallel
    if word.len() < 1000 {
        return detect_capital_use(word);
    }

    let mut it: Chars<'_> = word.chars();

    let cl: fn(char) -> bool = match (it.next(), it.next()) {
        (_, Some('a'..='z')) => |c: char| c.is_lowercase(),
        (Some('A'..='Z'), Some('A'..='Z')) => |c: char| c.is_uppercase(),
        (_, _) => |_c: char| false,
    };

    it.as_str()
        .par_chars()
        //.log("map")
        .map(|c: char| cl(c))
        .reduce(|| true, |l, r| l && r)
}

fn parallel_try_fold_detect_capital_use(word: &str) -> bool {
    // If the words is small, there is no need to do it in parallel
    if word.len() < 1000 {
        return detect_capital_use(word);
    }

    let mut it: Chars<'_> = word.chars();

    let cl: fn(char) -> bool = match (it.next(), it.next()) {
        (_, Some('a'..='z')) => |c: char| c.is_lowercase(),
        (Some('A'..='Z'), Some('A'..='Z')) => |c: char| c.is_uppercase(),
        (_, _) => |_c: char| false,
    };

    it.as_str()
        .par_chars()
        //.log("fold")
        .try_fold(
            || true,
            |b, e| {
                let n = b && cl(e);

                if n {
                    Some(n)
                } else {
                    None
                }
            },
        )
        .try_reduce(|| true, |l, r| Some(l && r))
        .unwrap_or(false)
}

fn main() {
    // // Used only for better diagrams and analysis
    // rayon::ThreadPoolBuilder::new().num_threads(4).build_global().unwrap();

    let test_sizes = [
        1_000,
        10_000,
        100_000,
        1_000_000,
        10_000_000,
        100_000_000,
        1_000_000_000,
    ];

    for size in test_sizes {
        println!("Input size: {:?}", size);

        let test_strings = &create_set(size);

        let mut start = std::time::Instant::now();
        for test in test_strings {
            test.assert(detect_capital_use);
        }
        println!("basic took {:?}", start.elapsed());

        start = std::time::Instant::now();
        for test in test_strings {
            test.assert(parallel_all_detect_capital_use);
        }
        println!("parallel all took {:?}", start.elapsed());
        // _create_diagrams(parallel_all_detect_capital_use, "all", &test_strings[11].string, &test_strings[12].string, &test_strings[14].string);

        start = std::time::Instant::now();
        for test in test_strings {
            test.assert(parallel_rayon_all_detect_capital_use);
        }
        println!("parallel rayon_all took {:?}", start.elapsed());
        // _create_diagrams(parallel_rayon_all_detect_capital_use, "rayon_all", &test_strings[11].string, &test_strings[12].string, &test_strings[14].string);

        start = std::time::Instant::now();
        for test in test_strings {
            test.assert(parallel_map_detect_capital_use);
        }
        println!("parallel map took {:?}", start.elapsed());
        // _create_diagrams(parallel_map_detect_capital_use, "map", &test_strings[11].string, &test_strings[12].string, &test_strings[14].string);

        start = std::time::Instant::now();
        for test in test_strings {
            test.assert(parallel_try_fold_detect_capital_use);
        }
        println!("parallel try fold took {:?}", start.elapsed());
        // _create_diagrams(parallel_try_fold_detect_capital_use, "try", &test_strings[11].string, &test_strings[12].string, &test_strings[14].string);

        println!("\n");
    }
}

struct Test {
    string: String,
    expected: bool,
}

impl Test {
    pub fn assert(&self, function: fn(word: &str) -> bool) {
        match self.expected {
            true => assert!(function(&self.string)),
            false => assert!(!function(&self.string)),
        }
    }
}

fn create_set(size: usize) -> Vec<Test> {
    let first_example: String = String::from("USA");
    let second_example: String = String::from("FlaG");
    let test1 = String::from("Leetcode"); // First letter capitalized, the rest lowercase
    let test2 = String::from("leetcode"); // All lowercase
    let test3 = String::from("LEETCODE"); // All uppercase
    let test4 = String::from("LeetCode"); // Mixed case, not valid
    let test5 = String::from("uSA"); // Only first letter lowercase, the rest uppercase
    let test6 = String::from("A"); // Single uppercase letter
    let test7 = String::from("a"); // Single lowercase letter

    // Test with a very large word, all uppercase
    let big_uppercase: String = "A".repeat(size);

    // Test with a very large word, all lowercase
    let big_lowercase: String = "a".repeat(size);

    // Test with a very large word, only the first letter capitalized
    let mut big_first_cap: String = "A".to_string();
    big_first_cap.push_str(&"a".repeat(size - 1));

    // Test with a very large word with incorrect capitalization pattern
    let mut big_incorrect: String = "A".repeat(size / 2);
    big_incorrect.push_str(&"a".repeat(size / 2));

    let mut almost_all_lowercase: String = "a".repeat(size - 1);
    almost_all_lowercase.push('Z'); // Last letter uppercase

    let mut rng = rand::thread_rng();
    let random_index = rng.gen_range(0..size);
    let mut random_incorrect: String = "o".repeat(random_index);
    random_incorrect.push('O');
    random_incorrect.push_str(&"o".repeat(size - random_index - 1));

    vec![
        Test {
            string: first_example,
            expected: true,
        },
        Test {
            string: second_example,
            expected: false,
        },
        Test {
            string: test1,
            expected: true,
        },
        Test {
            string: test2,
            expected: true,
        },
        Test {
            string: test3,
            expected: true,
        },
        Test {
            string: test4,
            expected: false,
        },
        Test {
            string: test5,
            expected: false,
        },
        Test {
            string: test6,
            expected: true,
        },
        Test {
            string: test7,
            expected: true,
        },
        Test {
            string: big_uppercase,
            expected: true,
        },
        Test {
            string: big_lowercase,
            expected: true,
        },
        Test { // index 11
            string: big_first_cap,
            expected: true,
        },
        Test { // index 12
            string: big_incorrect,
            expected: false,
        },
        Test {
            string: almost_all_lowercase,
            expected: false,
        },
        Test { // index 14
            string: random_incorrect,
            expected: false,
        },
    ]
}

fn _create_diagrams(function: fn(word: &str) -> bool, name: &str, big_first_cap: &str, big_incorrect: &str , random_incorrect: &str) {
    // Create visualisation of operations
    let mut path = name.to_owned();
    path.push_str("_full.svg");
    diam::svg(path, || function(&big_first_cap)).unwrap();

    // Create visualisation of operations
    path = name.to_owned();
    path.push_str("_half.svg");
    diam::svg(path, || function(&big_incorrect)).unwrap();

    // Create visualisation of operations
    path = name.to_owned();
    path.push_str("_random.svg");
    diam::svg(path, || function(&random_incorrect)).unwrap();
}
