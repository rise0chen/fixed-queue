use fixed_queue::VecDeque;

fn main() {
    let mut vec: VecDeque<u8, 4> = VecDeque::new();
    assert_eq!(vec.capacity(), 3);
    assert!(vec.is_empty());
    vec.push_back(1);
    println!("{}", vec[0]);
    vec.push_front(2);
    println!("{}", vec[1]);
    vec.push_back(3);
    println!("{}", vec[2]);
    println!("{:?}", vec.pop_back());
    println!("{:?}", vec.pop_back());
    println!("{:?}", vec.pop_back());
    println!("{:?}", vec.pop_front());
    println!("{:?}", vec.pop_back());
}
