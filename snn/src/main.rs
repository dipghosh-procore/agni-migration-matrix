mod tensor;
use tensor::Tensor;

fn main() {
    let mut tensor_a = Tensor::new(vec![vec![1.0, 2.0, 3.0,4.0]], vec![1, 4]);
    let tenssor_r = Tensor::random(vec![3,3]);
    println!("{:?} \n", tensor_a);
    println!("{:?} \n", tensor_a.shape());
    println!("{:?} \n", tensor_a.data());
    println!("{:?} \n", tensor_a.data_mutable());

    let tensor_a_t = tensor_a.transpose();
    println!("{:?} \n", tensor_a_t.shape());
    println!("{:?} \n", tensor_a_t.data());


    println!("{:?} \n", tenssor_r);
}