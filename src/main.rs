use atoi::ascii_to_digit;
use bitvec::prelude::*;
use memmap::Mmap;
use std::collections::HashMap;
use std::fs::File;
use std::time::Instant;

fn main() {
    let digits = unsafe {
        let file = File::open("src/pi-billion-trunc.txt").unwrap();
        Mmap::map(&file).unwrap()
    };

    let now = Instant::now();

    let mut seen = bitbox![0; 9_999_999_999];
    let mut dupes: u32 = 0;

    for x in digits.chunks(10) {
        let y = x.iter().fold(0_u32, |a, e| {
            a * 10 + ascii_to_digit::<u8>(*e).unwrap() as u32
        });
        // If already seen
        if seen[y as usize] {
            dupes += 1;
        } else {
            *seen.get_mut(y as usize).unwrap() = true;
        }
    }

    let elapsed = now.elapsed();

    println!("Duplicates: {:?}", dupes);
    println!("Elapsed: {:?}", elapsed);
}
