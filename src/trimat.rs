use std::cmp;
use std::io;
use std::ops;

#[derive(Debug)]
pub struct TriMat<T> {
    pub mat: Vec<Vec<T>>,
}

impl<T> TriMat<T> {
    pub fn new(n: usize) -> TriMat<T> {
        let mut mat: Vec<Vec<T>> = Vec::with_capacity(n);
        for i in 0..n {
            mat.push(Vec::with_capacity(n - i));
        }
        TriMat { mat }
    }
}

impl<T> TriMat<T>
where
    T: num::Zero + std::string::ToString + Copy,
{
    pub fn write_symmetric<Buffer: io::Write>(&self, buffer: &mut Buffer) {
        let n = self.mat.len();
        for i in 0..n + 1 {
            let mut line: Vec<T> = Vec::with_capacity(n);
            for j in 0..n + 1 {
                let dist: T;
                if i == j {
                    dist = T::zero();
                } else {
                    let smaller = cmp::min(i, j);
                    let larger = cmp::max(i, j);
                    dist = self[smaller][larger - smaller - 1];
                }
                line.push(dist);
            }
            let line: Vec<String> = line.into_iter().map(|i| i.to_string()).collect();
            writeln!(buffer, "{}", &line.join(","))
                .unwrap_or_else(|_| panic!("Error writing result at i: {}", i));
        }
    }
}

impl<T> ops::Index<usize> for TriMat<T> {
    type Output = Vec<T>;

    fn index(&self, i: usize) -> &Self::Output {
        &self.mat[i]
    }
}

impl<T> ops::IndexMut<usize> for TriMat<T> {
    fn index_mut(&mut self, i: usize) -> &mut Self::Output {
        &mut self.mat[i]
    }
}
