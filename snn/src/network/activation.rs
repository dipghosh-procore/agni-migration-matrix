
use crate::tensor::Tensor;
use std::f64::consts::E;

pub struct Activition {
    input: Tensor
}

impl Activation {

    fn relu(input: Tensor)-> Tensor{
        let (row, col) = (input.shape()[0], input.shape()[1]);
        let mut data = vec![vec![0; row]; col];

        (0..row).for_each(|i| (0..col).for_each(|j| data[i][j] = max(0, input.data[i][j]) ));
        Tensor::new(data, [row, col])
    }

    fn sigma(x: f32) -> &f32{
        1.0/(1 + E.powf(-(x)))
    }
    
    fn sigmoid(input: Tensor) -> Tensor{
    
        let (row, col) = (input.shape()[0], input.shape()[1]);
        let mut data = vec![vec![0; row]; col];
        (0..row).for_each(|i| (0..col).for_each(|j| data[i][j] = sigma(input[i][j]) ));
        Tensor::new(data, [row, col])
    }
}