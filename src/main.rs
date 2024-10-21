use td_mult_mat::Matrix;
use std::io;
fn main() {
    println!("Enter the size of the matrix");
    let mut n = String::new();
    let n: usize = match io::stdin().read_line(&mut n){
        Ok(n) => n as usize,
        Err(_) => panic!("Invalid input"),
    };

}
