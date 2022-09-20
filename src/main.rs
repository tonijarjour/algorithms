mod singly_linked;

fn main() {
    let mut list = singly_linked::List::new();
    list.push(5u8);
    list.push(6);
    list.push(7);
    list.push(8);
    list.push(9);
    list.deq();
    list.pop();
    list.deq();
    list.pop();

    println!("{:#?}", list);
}
