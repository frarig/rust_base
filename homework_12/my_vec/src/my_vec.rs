use crate::my_vec_iter::MyVecIter;

#[derive(Debug)]
pub struct MyVec<T> {
    data: [Option<T>; 32],
    len: usize,
}

impl<T> MyVec<T> {
    pub fn new() -> Self {
        MyVec {
            data: [const { None }; 32],
            len: 0,
        }
    }

    pub fn push(&mut self, value: T) -> Result<(), T> {
        if self.len >= self.data.len() {
            Err(value)
        } else {
            self.data[self.len] = Some(value);
            self.len += 1;
            Ok(())
        }
    }

    pub fn pop(&mut self) -> Option<T> {
        if self.len == 0 {
            None

        } else {
            let item = self.data[self.len - 1].take();
            self.data[self.len-1] = None;
            self.len -= 1;

            item
        }
    }

    pub fn len(&self) -> usize {
        self.len
    }

    pub fn is_empty(&self) -> bool {
        self.len == 0
    }

    pub fn get(&self, index: usize) -> Option<&T> {
        self.data[index].as_ref()
    }

    pub fn get_mut(&mut self, index: usize) -> Option<&mut T> {
        self.data[index].as_mut()
    }

    pub fn iter(&self) -> MyVecIter<'_, T> {
        MyVecIter {
            vec: self,
            pos: 0
        }
    }

    pub fn clear(&mut self) {
        self.data = [const { None }; 32];
        self.len = 0;
    }
}

#[cfg(test)]
mod test {
    use std::cell::Cell;
    use std::rc::Rc;
    use crate::my_vec::MyVec;

    #[test]
    fn push_pop_basic() {
        let mut v: MyVec<i32> = MyVec::new();
        v.push(1).unwrap();
        v.push(2).unwrap();
        v.push(3).unwrap();
        assert_eq!(v.len(), 3);
        assert_eq!(v.pop(), Some(3));
        assert_eq!(v.pop(), Some(2));
        assert_eq!(v.len(), 1);
    }

    #[test]
    fn iter_borrows() {
        let mut v: MyVec<String> = MyVec::new();
        v.push("a".into()).unwrap();
        v.push("b".into()).unwrap();
        let collected: Vec<&String> = v.iter().collect();
        assert_eq!(collected.len(), 2);
    }

    #[test]
    fn capacity_overflow() {
        let mut v: MyVec<i32> = MyVec::new();
        for i in 0..32 { v.push(i).unwrap(); }
        assert!(v.push(99).is_err());
    }

    #[test]
    fn drop_called() {
        let value = Rc::new(Cell::new(10));
        let mut vec = MyVec::new();

        vec.push(Rc::clone(&value));

        assert_eq!(Rc::strong_count(&value), 2);

        vec.clear();

        assert_eq!(vec.len(), 0);
        assert_eq!(Rc::strong_count(&value), 1);
    }

    #[test]
    fn new_creates_empty_vec() {
        let vec = MyVec::<i32>::new();

        assert_eq!(vec.len, 0);
        assert_eq!(vec.data.len(), 32);
        assert!(vec.data.iter().all(Option::is_none));
    }

    #[test]
    fn push_to_empty_vec() {
        let mut vec = MyVec::new();

        let result = vec.push(10);

        assert_eq!(result, Ok(()));
        assert_eq!(vec.len, 1);
        assert_eq!(vec.data[0], Some(10));
        assert!(vec.data[1..].iter().all(Option::is_none));
    }

    #[test]
    fn push_multiple_elements() {
        let mut vec = MyVec::new();

        assert_eq!(vec.push(10), Ok(()));
        assert_eq!(vec.push(20), Ok(()));
        assert_eq!(vec.push(30), Ok(()));

        assert_eq!(vec.len, 3);
        assert_eq!(vec.data[0], Some(10));
        assert_eq!(vec.data[1], Some(20));
        assert_eq!(vec.data[2], Some(30));
    }

    #[test]
    fn push_to_full_vec() {
        let mut vec = MyVec::new();

        for i in 0..32 {
            assert_eq!(vec.push(i), Ok(()));
        }

        assert_eq!(vec.len, 32);

        let result = vec.push(100);

        assert_eq!(result, Err(100));
        assert_eq!(vec.len, 32);
        assert_eq!(vec.data[31], Some(31));
    }

    #[test]
    fn push_returns_ownership_on_error() {
        let mut vec = MyVec::new();

        for i in 0..32 {
            vec.push(format!("item_{i}")).unwrap();
        }

        let value = String::from("Rust");

        let result = vec.push(value);

        let returned_value = result.unwrap_err();

        assert_eq!(returned_value, "Rust");
        assert_eq!(vec.len, 32);
    }

    #[test]
    fn pop_from_empty_vec() {
        let mut vec = MyVec::<i32>::new();

        assert_eq!(vec.pop(), None);
        assert_eq!(vec.len, 0);
        assert!(vec.data.iter().all(Option::is_none));
    }

    #[test]
    fn pop_single_element() {
        let mut vec = MyVec::new();

        vec.push(10).unwrap();

        assert_eq!(vec.pop(), Some(10));
        assert_eq!(vec.len, 0);
        assert_eq!(vec.data[0], None);
    }

    #[test]
    fn pop_multiple_elements() {
        let mut vec = MyVec::new();

        vec.push(10).unwrap();
        vec.push(20).unwrap();
        vec.push(30).unwrap();

        assert_eq!(vec.pop(), Some(30));
        assert_eq!(vec.len, 2);

        assert_eq!(vec.pop(), Some(20));
        assert_eq!(vec.len, 1);

        assert_eq!(vec.pop(), Some(10));
        assert_eq!(vec.len, 0);

        assert_eq!(vec.pop(), None);
        assert!(vec.data.iter().all(Option::is_none));
    }

    #[test]
    fn push_after_pop() {
        let mut vec = MyVec::new();

        vec.push(10).unwrap();
        vec.push(20).unwrap();

        assert_eq!(vec.pop(), Some(20));

        vec.push(30).unwrap();

        assert_eq!(vec.len, 2);
        assert_eq!(vec.data[0], Some(10));
        assert_eq!(vec.data[1], Some(30));
    }

    #[test]
    fn pop_transfers_ownership() {
        let mut vec = MyVec::new();

        vec.push(String::from("Rust")).unwrap();

        let value = vec.pop().unwrap();

        assert_eq!(value, "Rust");
        assert_eq!(vec.len, 0);
        assert!(vec.data[0].is_none());
    }
}