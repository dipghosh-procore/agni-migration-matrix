use rand::Rng;
use std::{ops::{Add, Mul}};


#[derive(Debug)]
pub struct Tensor {
    data: Vec<Vec<f32>>,
    shape: Vec<usize>
}

impl Tensor {

    pub fn new(data: Vec<Vec<f32>>, shape: Vec<usize>) -> Self{
        Tensor {data: data, shape: shape}
    }

    pub fn random(shape: Vec<usize>) -> Self {
        let column_size = shape[1];
        let row_size = shape[0];

        let mut data = Vec::with_capacity(column_size);
        for _ in 0..column_size {
            let mut row_data = Vec::with_capacity(row_size);
            for _ in 0..row_size {
                row_data.push(rand::rng().random::<f32>());
            }
            data.push(row_data);
        }
        Tensor {data: data, shape: shape}
    }

    pub fn shape(&self) -> &[usize] {
        &self.shape
    }

    pub fn data(&self) -> &Vec<Vec<f32>> {
        &self.data
    }

    pub fn data_mutable(&mut self) -> &mut Vec<Vec<f32>>{
        &mut self.data
    }

    pub fn transpose(&self) -> Self {
        let (rows, cols) = (self.shape[0], self.shape[1]);
        let mut transpose_data = vec![vec![self.data[0][0]; rows]; cols];
        for i in 0..rows {
            for j in 0..cols {
                transpose_data[j][i] = self.data[i][j];
            }
        }
        Tensor::new(transpose_data, vec![cols, rows])
    }
}


impl  Add for Tensor {

    type Output = Tensor;
    
    fn add(self, other: Self) -> Self::Output {

        assert_eq!(self.data.len(), other.data.len(), "Tensor must have the same length");
        let (rows, cols) = (self.shape[0], self.shape[1]);

        let mut sum = vec![vec![self.data[0][0]; cols]; rows]; 

        for i in 0..rows {
            for j in 0..cols {
                sum[i][j] =  self.data[i][j] + other.data[i][j];
            }
        }
        Tensor::new(sum, vec![rows, cols])
    }
}


impl Mul for Tensor {
    type Output = Tensor;

    fn mul(self, rhs: Self) -> Self::Output {
        let (lhs_row, rhs_cols) = (self.data.len(), rhs.data[0].len());
        assert_eq!(lhs_row, rhs_cols, "Matrix dimention misamtch");

        let (m, p) = (self.data.len(), rhs.data.len());

        let mut result_matrix = vec![vec![self.data[0][0];  m]; p];
        
        for i in 0..m {
            for j in 0..rhs_cols{
                let mut sum = 0.0;

                for k in 0..lhs_row {
                    sum =  sum + self.data[i][k] * rhs.data[k][j];
                }

                result_matrix[i][j] = sum;
            }
        }

        Tensor::new(result_matrix, vec![m, p])
    }
}