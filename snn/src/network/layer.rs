use crate::tensor::Tensor;

#[derive(Debug, Clone)]
pub struct Dnl {
    weight: Tensor,
    bias: Tensor,
}

impl  Dnl {

    pub fn new(input_size: usize, output_size: usize) -> Self {
        let weights = Tensor::random(vec![input_size, output_size]);
        let biases = Tensor::zeros(vec![output_size]);
        Dnl {weight: weights, bias: biases }
    }

    pub fn feedForward(&self, input: Tensor)-> Tensor {
        // Perform matrix multiplication: output = input * weights + biases
        let output = input * self.weight.clone();
        let matrix_add = output + self.bias.clone();
        matrix_add
    }
}