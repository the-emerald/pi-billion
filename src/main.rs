use atoi::ascii_to_digit;
use bitvec::macros::internal::core::sync::atomic::AtomicU32;
use bitvec::prelude::*;
use memmap::Mmap;
use once_cell::sync::Lazy;
use std::fs::File;
use std::sync::atomic::Ordering;
use std::sync::{Arc, Mutex};
use std::thread;
use std::time::Instant;

static PI_FILE: Lazy<Mmap> = Lazy::new(|| unsafe {
    let file = File::open("src/pi-billion-trunc.txt").unwrap();
    Mmap::map(&file).unwrap()
});

const NUMBER_SEGMENTS: usize = 50_000;
const TOTAL_BITBOX_SIZE: usize = 10_000_000_000;
const PER_SEGMENT_SIZE: usize = TOTAL_BITBOX_SIZE / NUMBER_SEGMENTS;
const ONE_BILLION: usize = 1_000_000_000;

struct Location(usize, u64);

impl Location {
    fn index(&self) -> usize {
        self.0
    }

    fn offset(&self) -> u64 {
        self.1
    }
}

struct Segment {
    bitbox: BitBox,
}

impl Segment {
    fn determine_loc(val: u64) -> Location {
        let index = val / PER_SEGMENT_SIZE as u64;
        let offset = val % PER_SEGMENT_SIZE as u64;
        Location {
            0: index as usize,
            1: offset,
        }
    }

    fn is_seen(&self, offset: u64) -> bool {
        self.bitbox[offset as usize]
    }

    fn mark_as_seen(&mut self, offset: u64) {
        *self
            .bitbox
            .get_mut(offset as usize)
            .expect("bitbox out of bounds") = true;
    }
}

impl Default for Segment {
    fn default() -> Self {
        Self {
            bitbox: bitbox![0; PER_SEGMENT_SIZE],
        }
    }
}

fn main() {
    Lazy::force(&PI_FILE);
    let bitboxes: Arc<Vec<Mutex<Segment>>> = {
        let mut bitboxes = Vec::with_capacity(NUMBER_SEGMENTS);
        for _ in 0..NUMBER_SEGMENTS {
            bitboxes.push(Mutex::new(Segment::default()));
        }
        Arc::new(bitboxes)
    };
    let duplicates: Arc<AtomicU32> = Arc::new(0_u32.into());
    let now = Instant::now();
    let mut threads = vec![];

    for thread_sized_chunk in PI_FILE.chunks(ONE_BILLION / 16) {
        let bitbox = bitboxes.clone();
        let duplicates = duplicates.clone();

        let handle = thread::spawn(move || {
            for ten in thread_sized_chunk.chunks(10) {
                let val = ten.iter().fold(0_u64, |a, e| {
                    a * 10 + ascii_to_digit::<u8>(*e).unwrap() as u64
                });

                let location = Segment::determine_loc(val);
                let segment = &mut bitbox[location.index()]
                    .lock()
                    .expect("could not acquire mutex");

                if segment.is_seen(location.offset()) {
                    duplicates.fetch_add(1, Ordering::SeqCst);
                } else {
                    segment.mark_as_seen(location.offset());
                }
            }
        });

        threads.push(handle);
    }

    for thread in threads {
        thread.join().unwrap();
    }

    let elapsed = now.elapsed();

    println!("Duplicates: {:?}", duplicates);
    println!("Elapsed: {:?}", elapsed);
}
