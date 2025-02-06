
use crate::tensor::Tensor;
use std::f32::consts::E;

pub fn relu(mut input: Tensor)-> Tensor{
    let (row, col) = (input.shape()[0], input.shape()[1]);
    let data_points =  input.data_mutable();
    let mut data = vec![vec![0.0; row]; col];

    (0..row).for_each(|i| (0..col).for_each(|j| data[i][j] = f32::max(0.0, data_points[i][j]) ));
    Tensor::new(data, vec![row, col])
}

fn sigma(x: f32) -> f32 {
    1.0f32 / (1.0f32  + E.powf(-x)) // No need for f32 suffixes here
}

pub fn sigmoid(mut input: Tensor) -> Tensor{

    let (row, col) = (input.shape()[0], input.shape()[1]);
    let data_points =  input.data_mutable();
    let mut data = vec![vec![0.0; row]; col];


    (0..row).for_each(|i| (0..col).for_each(|j| data[i][j] = sigma(data_points[i][j]) ));
    Tensor::new(data, vec![row, col])
}