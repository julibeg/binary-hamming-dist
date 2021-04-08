# NA agnostic pairwise Hamming distance of binary data

Calculates the symmetric matrix of pairwise hamming distances between a list of samples with binary data.
The input file should look like
```
001N0
N1100
10N10
```
with any character except for '0' and '1' denoting missing values ('N' in this case).
This would write
```
0,2,1
2,0,3
1,3,0
```
to the output. 


## Install
For some Linux systems the executable in `/bin` might work right out of the box. Otherwise, please install Rust, clone the repository and run `cargo build --release` in the root directory. The generated binary should be in `/target/release`.
