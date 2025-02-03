use rand::Rng;


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