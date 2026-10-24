use crate::my_vec::MyVec;

pub struct MyVecIter<'a, T> {
    pub(crate) vec: &'a MyVec<T>,
    pub(crate) pos: usize,
}

impl<'a, T> Iterator for MyVecIter<'a, T> {
    type Item = &'a T;

    fn next(&mut self) -> Option<Self::Item> {
        if self.pos >= self.vec.len() {
            return None;
        }

        let item = self.vec.get(self.pos);
        self.pos += 1;

        item
    }
}