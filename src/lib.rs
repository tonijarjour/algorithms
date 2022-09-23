pub mod singly_linked;

#[cfg(test)]
mod data_structures {
    use super::singly_linked;

    #[test]
    fn singly_linked_list() {
        let mut list = singly_linked::List::new();
        list.push(3u8).unwrap();
        list.enq(1).unwrap();
        list.insert(1, 2).unwrap();

        let mut list_contents = String::new();
        let mut list_iter = list.iter();

        use std::fmt::Write;
        write!(&mut list_contents, "{}", list_iter.next().unwrap().borrow())
            .unwrap();

        for n in list_iter {
            write!(&mut list_contents, " {}", n.borrow()).unwrap();
        }

        assert_eq!(list_contents, "1 2 3");
        assert_eq!(list.position(2).unwrap(), 1);
        assert_eq!(list.get(1).unwrap().borrow().value, 2);
        assert_eq!(list.len(), 3);

        assert_eq!(list.remove(1).unwrap(), 2);
        assert_eq!(list.deq().unwrap(), 1);
        assert_eq!(list.pop().unwrap(), 3);
    }
}
