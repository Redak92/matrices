use std::ops::{Index, IndexMut};
type Element = f64;

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


#[cfg(test)]
mod tests{
    use super::*;
    #[test]
    fn test_new(){
        let m = Matrix::new(2, vec![1.0, 2.0, 3.0, 4.0]);
        assert_eq!(m, Matrix { n: 2, values: vec![1.0, 2.0, 3.0, 4.0] });
    }

}