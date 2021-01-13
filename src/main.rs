use std::collections::HashMap;
// use indicatif::{ProgressBar, ProgressStyle};
use rayon::prelude::*;
use std::time::Instant;
use atoi::{ascii_to_digit};
use std::fs::File;
use memmap::Mmap;

fn main() {
    let digits = unsafe {
        let file = File::open("src/pi-billion-trunc.txt").unwrap();
        Mmap::map(&file).unwrap()
    };

    // let mut combinations = HashMap::new();
    //
    // for chunk in digits.chunks(10) {
    //     let n = chunk.iter().fold(0_u32, |acc, e| acc * 10 + *e as u32);
    //     let seen = combinations.entry(n).or_insert(0);
    //     *seen += 1;
    //
    //     bar.inc(1);
    // }

    let now = Instant::now();

    let combinations = digits.par_chunks(10)
        .map(|n| {
            n.iter().fold(0_u32, |acc, e| acc * 10 + ascii_to_digit::<u8>(*e).unwrap() as u32)
        })
        .fold(|| HashMap::<u32, u32>::new(),
              |mut a, v| {
                  let seen = a.entry(v).or_insert(0);
                  *seen += 1;
                  a
              })
        .reduce(|| HashMap::new(),
        |one, two| {
            two.iter().fold(one, |mut a, (k, v2)| {
                let v1 = a.entry(*k).or_insert(0);
                *v1 += v2;
                a
            })
        });

    let dupes = combinations.into_par_iter()
        .map(|(_, vs)| vs - 1)
        .reduce(|| 0_u32, |a, b| a + b);

    let elapsed = now.elapsed();

    println!("Duplicates: {:?}", dupes);
    println!("Elapsed: {:?}", elapsed);
}
