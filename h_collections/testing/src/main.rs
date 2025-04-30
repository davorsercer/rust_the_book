use std::collections::VecDeque;
fn main() {
    let vec = [1, 2, 3, 4];
    let buf: VecDeque<_> = vec.into_iter().collect();
    println!("{:?}", buf);
}
