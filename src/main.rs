fn main() {

    let mut curr = 0;
    let mut willb = 1;

    for i in 0..5 {
        let mut temp = willb;
        willb = willb + curr;
        curr = temp;
        println!("{}",willb)
    }
}
