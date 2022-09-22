mod singly_linked;

fn main() {
    let mut list: singly_linked::List<u8> = singly_linked::List::new();
    list.enq(1).unwrap();
    list.enq(2).unwrap();
    list.enq(3).unwrap();
    list.enq(4).unwrap();
    list.enq(5).unwrap();
    list.enq(6).unwrap();

    list.insert(2, 1);
    list.insert(5, 1);

    list.remove(2);
    list.remove(4);

    for n in list {
        print!("{} ", n.borrow());
    }

    println!();
}
