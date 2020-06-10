# NA agnostic pairwise Hamming distance of binary data

Calculates the symmetric matrix of pairwise hamming distances between a list of samples with binary data.
The input file should look like
```
001N0
N1100
10N10
```
with any character out of [A-Za-z2-9] (`N` in this case) denoting missing values.
This would write
```
0,2,1
2,0,3
1,3,0
```
to the output file.
