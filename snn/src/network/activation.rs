
use crate::tensor::Tensor;
use std::f64::consts::E;

fn relu(input: Tensor)-> Tensor{
    let (row, col) = input.shape();
    let mut data = vec![vec![0; row]; col];

    for i in 0..row{
        for j in 0..col{
            data[i][j] = max(0, input.data[i][j]);
        }
    }
    Tensor::new(data, [row, col])
}


fn sigmoid(x: f32) -> &f32{
    1.0/(1 + E.powf(-(x)))
}

fn sigmoidActivition(input: Tensor) -> Tensor{

    let (row, col) = input.shape();
    let mut data = vec![vec![0; row]; col];

    for i in 0..row{
        for j in 0..col{
            data[i][j] = sigmoid(input[i][j])
        }
    }
}