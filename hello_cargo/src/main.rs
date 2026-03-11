// use std::env;
// use std::str::FromStr;

// fn main() {
//     let mut numbers = Vec::new();
//     for arg in env::args().skip(1) {
//         println!("arg: {}", arg);
//         println!("&arg: {}", &arg);
//         numbers.push(u64::from_str(&arg).expect("error parsing argument"));
//     }

//     if numbers.len() == 0 {
//         eprintln!("Usage: gcd NUMBER ...");
//         std::process::exit(1);
//     }
//     let mut d = numbers[0];
//     for m in &numbers[1..] {
//         d = gcd(d, *m);
//     }
//     println!("The greatest common divisor of {:?} is {}", numbers, d);
// }

// fn gcd(mut n: u64, mut m: u64) -> u64 {
//     assert!(n != 0 && m != 0);
//     while m != 0 {
//         if m < n {
//             let t = m;
//             m = n;
//             n = t;
//         }
//         m = m % n;
//     }
//     n
// }

// #[test]
// fn test_gcd() {
//     assert_eq!(gcd(14, 15), 1);

//     assert_eq!(gcd(2 * 3 * 5 * 7 * 11, 3 * 7 * 11 * 13 * 19), 3 * 7 * 11);
// }

// fn reverse(input: &str) -> String {
//     let input_len: usize = input.len();
//     println!("reverse string! input: {}, len: {}", input, input_len);
//     let mut reversed: String = String::new();
//     for (i, c) in input.chars().rev().enumerate() {
//         // println!("Pushing {}th element {}", i, c);
//         reversed.push(c)
//     }
//     reversed
// }

// #[test]
// fn test_reverse() {
//     let input: &str = "!hello ";
//     let output: String = reverse(input);
//     let expected: &str = " olleh!";
//     assert_eq!(expected, output)
// }

use time::macros::datetime;
use time::Duration;
use time::PrimitiveDateTime as DateTime;

pub fn find_date_time_one_gigasecond_after(start: DateTime) -> DateTime {
    println!("start: {}", start);
    let lag = 1e9 as i64;
    let lag_seconds = Duration::seconds(lag);
    println!("lag: {}, lag_seconds: {}", lag, lag_seconds);
    // let end: DateTime = start
    //     .checked_add(lag_seconds)
    //     .expect("Should be 1 gigaseconds later.");
    // println!("end: {:?}", end);
    // end
    let end: Option<DateTime> = start.checked_add(Duration::seconds(lag));
    // end.unwrap()
    match end {
        Some(v) => v,
        None => panic!("What's happening!"),
    }
}

#[test]
fn date_only_specification_of_time() {
    let start = datetime!(2011-4-25 0:00:00);
    let actual = find_date_time_one_gigasecond_after(start);
    let expected = datetime!(2043-1-1 1:46:40);
    assert_eq!(actual, expected);
}

fn main() {
    let mut start: DateTime = datetime!(2015-12-31 11:35:23);
    let output = find_date_time_one_gigasecond_after(start);
    println!("output: {}", output)
}
