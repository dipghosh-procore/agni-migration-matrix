mod tensor;
mod network;
use tensor::Tensor;

fn main() {
    let  tensor_a = Tensor::new(vec![vec![1.0, 2.0, 3.0,4.0]], vec![1, 4]);
    println!("{:?} \n", tensor_a);
    let tenssor_1 = Tensor::random(vec![3,3]);
    let tenssor_2 = Tensor::random(vec![3,3]);
    println!("Tensor {:?}", tenssor_1);
    println!("Tensor {:?}", tenssor_2);

    let mulp =  tenssor_1.clone() * tenssor_2.clone();

    println!("multiply {:?} \n", mulp.shape());
    println!("multiply {:?} \n", mulp.data());
    
    let addi = tenssor_1 + tenssor_2;
    println!("multiply {:?} \n", addi.data());
    println!("multiply {:?} \n", addi.data());

    let tensor_a_t = tensor_a.transpose();
    println!("{:?} \n", tensor_a_t);
}