use crate::tensor::Tensor;

#[derive(Debug, Clone)]
pub struct Dnl {
    tensor: Tensor,
    bias: Tensor,
}

impl  Dnl {

    pub fn new(input_size: usize, output_size: usize) -> Self {
        let weights = Tensor::random(vec![input_size, output_size]);
        let biases = Tensor::zeros(vec![output_size]);
        DenseLayer { weights, biases }
    }

    pub fn feedForward(&self, input: Tensor)-> Tensor {
        // Perform matrix multiplication: output = input * weights + biases
        let output = input * self.weights;
        let matrixAdd = output + self.biases;
        matrixAdd
    }
}