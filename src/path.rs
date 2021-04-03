use std::ops::{Index, IndexMut};
use std::{mem, fmt};
use std::fmt::{Display, Formatter};

#[derive(Eq, PartialEq, Debug)]
pub struct Path(Vec<(usize, usize)>);

impl Path
{
    pub fn new(path: Vec<(usize, usize)>) -> Self {
        debug_assert!(path.len() > 1);
        Self(path)
    }

    pub fn from_size(size: usize) -> Self {
        Self::new(vec![(0usize, 0usize); size])
    }

    fn internal_init_edge(&mut self, v0: usize, v1: usize) {
        let edge = &mut self[v0];

        let vertex = if edge.0 == 0 { &mut edge.0 } else { &mut edge.1 };
        *vertex = v1;

        if edge.0 > edge.1 {
            mem::swap(&mut edge.0, &mut edge.1);
        }
    }

    pub fn init_edge(&mut self, v0: usize, v1: usize) {
        self.internal_init_edge(v0, v1);
        self.internal_init_edge(v1, v0);
    }

    /// Vertices visited by the path finishing in 0.
    pub fn vertices_visited(&self) -> Vec<usize> {
        let mut res = vec![0usize; self.0.len()];

        let prev_vertex = self.0[self.0.len() - 1];
        let mut prev_vertex = if prev_vertex.0 == 0 { prev_vertex.1 } else { prev_vertex.1 };

        let mut vertex = 0;
        let mut i = 0;

        loop {
            let edge = self.0[vertex];
            let edge = if edge.0 == prev_vertex { edge.1 } else { edge.0 };

            res[i] = edge;
            i += 1;

            prev_vertex = vertex;
            vertex = edge;

            if vertex == 0 { break; }
        }

        res
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

impl Display for Path {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "Path({:?})", self.vertices_visited())
    }
}

#[cfg(test)]
mod tests {
    #[cfg(test)]
    mod vertices {
        use crate::path::Path;

        #[test]
        fn test() {
            let path = Path::new(vec![(1, 4), (0, 3), (3, 4), (1, 2), (0, 2)]);
            let actual = path.vertices_visited();
            let expected = vec![1usize, 3, 2, 4, 0];
            assert_eq!(actual, expected);
        }
    }
}
