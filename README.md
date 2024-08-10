# graveler-rs

See: https://github.com/arhourigan/graveler

A little bit faster version written in Rust.

uses rayon for parallel computing (https://github.com/rayon-rs/rayon).

and fastrand for generating random numbers (https://github.com/smol-rs/fastrand).

benchmarking on my windows laptop gives me a run time of about 15ms for 1,000,000 tries.

![image](https://github.com/user-attachments/assets/44bc6a14-53eb-49fd-833b-d3d5de826305)

To run; clone the repo, make sure rust and cargo is installed.

`cargo run --release` to run the executable
or `cargo bench` to run criterion benchmarks




