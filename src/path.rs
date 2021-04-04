use std::ops::{Index, IndexMut};
use std::{mem, fmt};
use std::fmt::{Display, Formatter};

#[derive(Eq, PartialEq, Debug)]
pub struct Path(pub(crate) Vec<(usize, usize)>);

#[derive(Eq, PartialEq, Debug, Copy, Clone)]
pub enum HamiltonianResult {
    Ok,
    NoEdgeBack(usize, usize, (usize, usize)),
    NotVisited(usize),
}

impl Path
{
    pub fn uninitialized(size: usize) -> Self {
        assert!(size > 1);
        Self(vec![(0usize, 0usize); size])
    }

    pub fn new(data: Vec<(usize, usize)>) -> Self {
        assert!(data.len() > 1);
        Self(data)
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

    fn next(&self, coming_from: &mut usize, vertex: &mut usize) -> Option<usize> {
        if *coming_from <= self.0.len() && *vertex == 0 {
            return None;
        }

        let res = self.0[*vertex];
        let going_to = if res.0 != *coming_from { res.0 } else { res.1 };

        *coming_from = *vertex;
        *vertex = going_to;

        Some(going_to)
    }

    /// Vertices visited by the path ending in 0.
    pub fn vertices_visited(&self) -> VerticesVisited {
        VerticesVisited {
            path: &self,
            coming_from: self.0.len() + 1,
            vertex: 0,
        }
    }

    /// Edges visited by the path starting in (0, x) and ending in (y, 0).
    pub fn edges_visited(&self) -> EdgesVisited {
        EdgesVisited {
            path: &self,
            coming_from: self.0.len() + 1,
            vertex: 0,
        }
    }

    /// Check if the path is complete and Hamiltonian.
    /// This method is not optimized and is supposed to be used only for debug/sanity purposes.
    pub fn check_hamiltonian(&self) -> HamiltonianResult {
        // Check if all edges link back.
        for vertex in 0..self.0.len() {
            let adj = self[vertex];

            let adj0 = self[adj.0];
            if adj0.0 != vertex && adj0.1 != vertex {
                return dbg!(HamiltonianResult::NoEdgeBack(vertex, adj.0, adj0));
            };

            let adj1 = self[adj.1];
            if adj1.0 != vertex && adj1.1 != vertex {
                return dbg!(HamiltonianResult::NoEdgeBack(vertex, adj.1, adj1));
            };
        }

        // Check if all vertices are present
        let vertices = self.vertices_visited().collect::<Vec<_>>();
        for vertex in 0..self.0.len() {
            if !vertices.iter().any(|&v| v == vertex) {
                return dbg!(HamiltonianResult::NotVisited(vertex));
            }
        }

        HamiltonianResult::Ok
    }

    pub fn is_hamiltonian(&self) -> bool {
        self.check_hamiltonian() == HamiltonianResult::Ok
    }

    /// Twist two edges.
    /// Don't even try to understand this because I didn't.
    /// Let's just hope `is_hamiltonian` can find potential bugs.
    pub fn twist(&mut self, (i0, v0): (usize, usize), (i1, v1): (usize, usize)) {
        let a0 = &mut self[i0];
        let a0 = if a0.0 == v0 { &mut a0.0 } else { &mut a0.1 };
        debug_assert_eq!(*a0, v0);
        *a0 = i1;

        let a0_next = &mut self[v0];
        let a0_next = if a0_next.0 == i0 { &mut a0_next.0 } else { &mut a0_next.1 };
        debug_assert_eq!(*a0_next, i0);
        *a0_next = v1;

        let a1 = &mut self[i1];
        let a1 = if a1.0 == v1 { &mut a1.0 } else { &mut a1.1 };
        debug_assert_eq!(*a1, v1);
        *a1 = i0;

        let a1_next = &mut self[v1];
        let a1_next = if a1_next.0 == i1 { &mut a1_next.0 } else { &mut a1_next.1 };
        debug_assert_eq!(*a1_next, i1);
        *a1_next = v0;

        debug_assert!(self.is_hamiltonian());
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
        write!(f, "Path({:?})", self.vertices_visited().collect::<Vec<_>>())
    }
}

#[derive(Clone)]
pub struct VerticesVisited<'a> {
    path: &'a Path,
    coming_from: usize,
    vertex: usize,
}

impl<'a> Iterator for VerticesVisited<'a> {
    type Item = usize;

    fn next(&mut self) -> Option<Self::Item> {
        self.path.next(&mut self.coming_from, &mut self.vertex)
    }
}

#[derive(Clone)]
pub struct EdgesVisited<'a> {
    path: &'a Path,
    coming_from: usize,
    vertex: usize,
}

impl<'a> Iterator for EdgesVisited<'a> {
    type Item = (usize, usize);

    fn next(&mut self) -> Option<Self::Item> {
        let going_to = self.path.next(&mut self.coming_from, &mut self.vertex)?;
        Some((self.coming_from, going_to))
    }
}

#[cfg(test)]
mod tests {
    use crate::path::Path;

    fn get_path() -> Path {
        Path(vec![(1, 4), (0, 3), (3, 4), (1, 2), (0, 2)])
    }

    #[test]
    fn vertices() {
        let path = get_path();
        let actual = path.vertices_visited().collect::<Vec<_>>();
        let expected = vec![1usize, 3, 2, 4, 0];
        assert_eq!(actual, expected);
    }

    #[test]
    fn edges() {
        let path = get_path();
        let actual = path.edges_visited().collect::<Vec<_>>();
        let expected = vec![(0usize, 1usize), (1, 3), (3, 2), (2, 4), (4, 0)];
        assert_eq!(actual, expected);
    }


    #[cfg(test)]
    mod hamiltonian {
        use crate::path::{Path, HamiltonianResult};

        #[test]
        fn all() {
            let path = Path(vec![(1usize, 7usize), (0, 2), (1, 3), (2, 4), (3, 5), (4, 6), (5, 7), (6, 0)]);
            assert_eq!(path.check_hamiltonian(), HamiltonianResult::Ok);
            assert!(path.is_hamiltonian());
        }

        #[test]
        fn last_has_no_edge_back() {
            let path = Path(vec![(1usize, 1usize), (0, 2), (1, 3), (2, 4), (3, 5), (4, 6), (5, 7), (6, 0)]);
            assert_eq!(path.check_hamiltonian(), HamiltonianResult::NoEdgeBack(7, 0, (1, 1)));
        }

        #[test]
        fn disconnected_012_34567() {
            let path = Path(vec![(1usize, 2usize), (0, 2), (1, 0), (4, 7), (3, 5), (4, 6), (5, 7), (6, 3)]);
            assert_eq!(path.check_hamiltonian(), HamiltonianResult::NotVisited(3));
        }
    }
}
