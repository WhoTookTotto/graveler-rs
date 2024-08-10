# graveler-rs

See: https://github.com/arhourigan/graveler

A little bit faster version written in Rust.

uses rayon for parallel computing (https://github.com/rayon-rs/rayon).

and fastrand for generating random numbers (https://github.com/smol-rs/fastrand).

benchmarking on my windows laptop gives me a run time of about 15ms for 1,000,000 tries (100_000 TRIES, 10 RUNS).

![image](https://github.com/user-attachments/assets/44bc6a14-53eb-49fd-833b-d3d5de826305)

1,000,000,000 tries took about 14-15 seconds using 10_000_000 TRIES over 100 RUNS

![image](https://github.com/user-attachments/assets/663aa909-9cd9-4400-a4a2-6fd5a881c31b)

To run; clone the repo, make sure rust and cargo is installed.

`cargo run --release` to run the executable
or `cargo bench` to run criterion benchmarks




