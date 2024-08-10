use rayon::iter::{IntoParallelIterator, ParallelIterator};

const TRIES_PER_THREAD: usize = 100_000;

const RUNS : usize = 10;

fn main() {
    run_threads();
}

fn run_threads() -> u8 {
    let r = (0..RUNS).into_par_iter().map(move |_| run_tries()).collect::<Vec<u8>>();

    let max = r.iter().max().unwrap();

    println!("Max: {}", max);
    println!("Number of rolls: {}", RUNS*TRIES_PER_THREAD);

    *max
}

fn run_tries() -> u8 {
    let mut rng = fastrand::Rng::new();

    let mut max_ones = 0;

    for _ in 0..TRIES_PER_THREAD {

        let mut ones = 0;

        let mut rolls = [0u8; 231];

        rng.fill(&mut rolls);

        for roll in rolls.iter() {
            ones += (*roll&0b11 == 0b01) as u8;
        }

        if ones>max_ones {
            max_ones = ones;

            if max_ones == 177 {
                break;
            }
        }
    }


    max_ones
}
