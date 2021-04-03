use std::ops::{Index, IndexMut};

#[derive(Eq, PartialEq, Debug)]
pub struct Path(pub(crate) Vec<(usize, usize)>);


impl Path
{
    pub fn new(path: Vec<(usize, usize)>) -> Self {
        debug_assert!(path.len() > 1);
        Self(path)
    }

    pub fn from_size(size: usize) -> Self {
        Self::new(vec![(0usize, 0usize); size])
    }
}

impl Index<usize> for Path {
    type Output = (usize, usize);

    fn index(&self, index: usize) -> &Self::Output {
        debug_assert!(index < self.0.len());
        unsafe { self.0.get_unchecked(index) }
    }
}

impl IndexMut<usize> for Path {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        debug_assert!(index < self.0.len());
        unsafe { self.0.get_unchecked_mut(index) }
    }
}
