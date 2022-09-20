mod singly_linked;

fn main() {
    let mut list: singly_linked::List<u8> = singly_linked::List::new();
    list.push(5).unwrap();
    list.push(6).unwrap();
    list.push(7).unwrap();
    list.push(8).unwrap();
    list.push(9).unwrap();
    list.enq(1).unwrap();
    list.enq(0).unwrap();

    println!("{:#?}", list);

    list.pop().unwrap();
    list.pop().unwrap();
    list.pop().unwrap();
    list.deq().unwrap();
    list.deq().unwrap();
    list.deq().unwrap();
    list.pop().unwrap();

    println!("{:#?}", list);
}
