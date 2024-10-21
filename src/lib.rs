use std::ops::{Index, IndexMut};
type Element = f64;
use rand::Rng;
#[derive(Debug, PartialEq)]
pub struct Matrix {
    n: usize,
    values: Vec<Element>,
}
impl Matrix {
    pub fn new(n: usize, values: Vec<Element>) -> Matrix {
        assert_eq!(values.len(), n * n);
        Matrix { n, values }
    }
}

impl Index<(usize, usize)> for Matrix {
    type Output = Element;
    fn index(&self, index: (usize, usize)) -> &Element {
        let (i, j) = index;
        &self.values[i * self.n + j]
    }
}

impl IndexMut<(usize, usize)> for Matrix{
    fn index_mut(&mut self, index: (usize, usize)) -> &mut Self::Output {
        let (i, j) = index;
        &mut self.values[i * self.n + j]
    }
}

impl Matrix{
    pub fn id(n: usize) -> Matrix{
        let mut values = vec![0.0; n * n];
        for i in 0..n{
            values[i * n + i] = 1.0;
        }
        Matrix { n, values }
    }
    pub fn random(n: usize) -> Matrix{
        
        let mut values = vec![0.0; n^2];
        let mut rng = rand::thread_rng();
        values = values.iter_mut().map(|x| -> f64 {rng.gen_range(-1.0..1.0)}).collect();
        Matrix {n, values } 

    }
    pub fn multiply(m1:&Matrix, m2: &Matrix) -> Matrix{
        assert_eq!(m1.n, m2.n);
        let n: usize = m1.n;
        let mut values = vec![0.0; n^2];
        for i in 0..n{
            for j in 0..n{
                for k in 0..n{
                    values[i * m1.n + j] += m1[(i, k)] * m2[(k, j)];
                }
            }
        }
        Matrix { n: m1.n, values }
    }
}






#[cfg(test)]
mod tests{
    use super::*;
    #[test]
    fn test_new(){
        let m = Matrix::new(2, vec![1.0, 2.0, 3.0, 4.0]);
        assert_eq!(m, Matrix { n: 2, values: vec![1.0, 2.0, 3.0, 4.0] });
    }

    #[test]
fn indexes() {
    let mut m = Matrix::new(2, vec![1.0, 2.0, 3.0, 4.0]);
    assert_eq!(m[(0, 0)], 1.0);
    assert_eq!(m[(1, 0)], 3.0);

    m[(1,0)] = 5.0;
    assert_eq!(m[(1, 0)], 5.0);
}
#[test]
fn mult_matrix() {
    let m1 = Matrix::new(2, vec![1.0, 2.0, 3.0, 4.0]);
    let m2 = Matrix::new(2, vec![5.0, 6.0, 7.0, 8.0]);
    let m3 = Matrix::multply(&m1, &m2);
    assert_eq!(m3, Matrix::new(2, vec![19.0, 22.0, 43.0, 50.0]));

}

}