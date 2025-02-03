mod tensor;
use tensor::Tensor;

fn main() {
    let tensor_a = Tensor::new(vec![1.0, 2.0, 3.0,4.0], vec![4, 1]);
    let mut tenssor_r = Tensor::random(vec![3,3,3]);
    println!("{:?} \n", tensor_a);
    println!("{:?} \n", tenssor_r);
    println!("{:?} \n", tenssor_r.shape());
    println!("{:?} \n", tenssor_r.data());
    println!("{:?} \n", tenssor_r.data_mutable());
}