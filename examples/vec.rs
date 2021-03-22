use fixed_queue::Vec;

fn main() {
    let mut vec: Vec<u8, 3> = Vec::new();
    assert_eq!(vec.capacity(), 3);
    assert!(vec.is_empty());
    vec.push(1);
    println!("{}", vec[0]);
    vec.push(2);
    println!("{}", vec[1]);
    vec.push(3);
    println!("{}", vec[2]);
    println!("{:?}", vec.pop());
    println!("{:?}", vec.pop());
    println!("{:?}", vec.pop());
    println!("{:?}", vec.pop());
}
