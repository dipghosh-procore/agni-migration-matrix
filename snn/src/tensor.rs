use rand::Rng;


#[derive(Debug)]
pub struct Tensor {
    data: Vec<f32>,
    shape: Vec<usize>
}

impl Tensor {

    pub fn new(data: Vec<f32>, shape: Vec<usize>) -> Self{
        assert_eq!(data.len(), shape.iter().product(), "Data length must match shape");
        Tensor {data: data, shape: shape}
    }

    pub fn random(shape: Vec<usize>) -> Self {
        let shape_size =  shape.iter().product();
        let mut data = Vec::with_capacity(shape_size);

        for _ in 0..shape_size {
            data.push(rand::rng().random::<f32>());
        }
        Tensor {data: data, shape: shape}
    }

    pub fn shape(&self) -> &[usize] {
        &self.shape
    }

    pub fn data(&self) -> &[f32] {
        &self.data
    }

    pub fn data_mutable(&mut self) -> &mut [f32] {
        &mut self.data
    }
}   