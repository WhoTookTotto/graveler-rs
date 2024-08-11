const TRIES: u32 = 1_000_000_000;


pub fn run_tries() -> u8 {
    let mut rng = fastrand::Rng::new();

    let mut max_ones = 0;

    for _ in 0..TRIES {

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
