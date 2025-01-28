use crate::rc::*;

pub mod bfs;
pub mod dfs;

pub use bfs::*;
pub use dfs::*;

pub trait Recursive: Sized {
    type Node;
    fn get_if_node(&self) -> Option<&Self::Node>;
    fn get_if_node_mut(&mut self) -> Option<&mut Self::Node>;
    fn children(node: &Self::Node) -> &Tree<Self>;
    fn children_mut(node: &mut Self::Node) -> &mut Tree<Self>;
}

pub struct Iter<'a, T> {
    iter: std::slice::Iter<'a, StrongMut<T>>,
}

impl<'a, T: Recursive> Iter<'a, T> {
    fn new(src: &'a Tree<T>) -> Self {
        Self { iter: src.0.iter() }
    }
}

impl<'a, T> Iterator for Iter<'a, T> {
    type Item = Strong<T>;
    fn next(&mut self) -> Option<Self::Item> {
        self.iter.next().map(|rc| rc.clone())
    }
}
impl<'a, T> DoubleEndedIterator for Iter<'a, T> {
    fn next_back(&mut self) -> Option<Self::Item> {
        self.iter.next_back().map(|rc| rc.clone())
    }
}
impl<'a, T> ExactSizeIterator for Iter<'a, T> {}

pub struct IterMut<'a, T> {
    iter: std::slice::IterMut<'a, StrongMut<T>>,
}

impl<'a, T: Recursive> IterMut<'a, T> {
    fn new(src: &'a mut Tree<T>) -> Self {
        Self { iter: src.0.iter_mut() }
    }
}

impl<'a, T> Iterator for IterMut<'a, T> {
    type Item = StrongMut<T>;
    fn next(&mut self) -> Option<Self::Item> {
        self.iter.next().map(|rc| rc.clone_mut())
    }
}
impl<'a, T> DoubleEndedIterator for IterMut<'a, T> {
    fn next_back(&mut self) -> Option<Self::Item> {
        self.iter.next_back().map(|rc| rc.clone_mut())
    }
}
impl<'a, T> ExactSizeIterator for IterMut<'a, T> {}

pub struct Tree<T: Recursive>(Vec<StrongMut<T>>);

impl<T: Recursive, V: Into<Vec<StrongMut<T>>>> From<V> for Tree<T> {
    fn from(value: V) -> Self {
        Self(value.into())
    }
}

impl<T: Recursive> FromIterator<StrongMut<T>> for Tree<T> {
    fn from_iter<I: IntoIterator<Item = StrongMut<T>>>(iter: I) -> Self {
        Self(iter.into_iter().collect())
    }
}

impl<T: Recursive> FromIterator<T> for Tree<T> {
    fn from_iter<I: IntoIterator<Item = T>>(iter: I) -> Self {
        Self(iter.into_iter().map(|x| StrongMut::new(x)).collect())
    }
}

impl<T: Recursive> std::ops::Index<usize> for Tree<T> {
    type Output = StrongMut<T>;
    fn index(&self, index: usize) -> &Self::Output {
        &self.0[index]
    }
}

impl<T: Recursive> std::ops::IndexMut<usize> for Tree<T> {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        &mut self.0[index]
    }
}

impl<T: Recursive> Tree<T> {
    pub const fn new() -> Self {
        Self(Vec::new())
    }

    pub fn with_capacity(capacity: usize) -> Self {
        Self(Vec::with_capacity(capacity))
    }

    #[inline]
    pub fn len(&self) -> usize {
        self.0.len()
    }

    #[inline]
    pub fn capacity(&self) -> usize {
        self.0.capacity()
    }

    #[inline]
    pub fn reserve(&mut self, additional: usize) {
        self.0.reserve(additional)
    }

    #[inline]
    pub fn is_empty(&self) -> bool {
        self.0.is_empty()
    }

    #[inline]
    pub fn is_full(&self) -> bool {
        self.0.len() == self.0.capacity()
    }

    #[inline]
    pub fn push(&mut self, value: StrongMut<T>) {
        self.0.push(value)
    }

    #[inline]
    pub fn pop(&mut self) -> Option<StrongMut<T>> {
        self.0.pop()
    }

    #[inline]
    pub fn insert(&mut self, index: usize, element: StrongMut<T>) {
        self.0.insert(index, element)
    }

    #[inline]
    pub fn remove(&mut self, index: usize) -> StrongMut<T> {
        self.0.remove(index)
    }

    /// Iterate over only the topmost layer
    #[inline]
    pub fn shallow_iter<'a>(&'a self) -> Iter<'a, T> {
        Iter::new(self)
    }

    /// Iterate over only the topmost layer
    #[inline]
    pub fn shallow_iter_mut<'a>(&'a mut self) -> IterMut<'a, T> {
        IterMut::new(self)
    }
}

pub trait RecursiveIterator: Iterator<Item: Owned<Inner = Self::Inner>> {
    type Inner: Recursive;

    /// Depth of the most recent item to have been returned by `next()`
    fn last_depth(&self) -> usize;
}

pub struct EnumerateDepth<I> {
    iter: I,
}

impl<I> EnumerateDepth<I> {
    fn new(iter: I) -> Self {
        Self { iter }
    }
}

impl<I: RecursiveIterator> Iterator for EnumerateDepth<I> {
    type Item = (usize, I::Item);
    fn next(&mut self) -> Option<Self::Item> {
        self.iter.next().map(|x| (self.iter.last_depth(), x))
    }
}

impl<I: RecursiveIterator + DoubleEndedIterator> DoubleEndedIterator for EnumerateDepth<I> {
    fn next_back(&mut self) -> Option<Self::Item> {
        self.iter.next_back().map(|x| (self.iter.last_depth(), x))
    }
}

#[cfg(test)]
mod tests {
    use crate::iter::directed::*;
    use super::*;

    enum Foo {
        Value(usize),
        Branch(char, Tree<Self>),
    }
    impl Recursive for Foo {
        type Node = Tree<Self>;
        fn get_if_node(&self) -> Option<&Self::Node> {
            match self {
                Foo::Value(_) => None,
                Foo::Branch(_, tree) => Some(tree),
            }
        }
        fn get_if_node_mut(&mut self) -> Option<&mut Self::Node> {
            match self {
                Foo::Value(_) => None,
                Foo::Branch(_, tree) => Some(tree),
            }
        }
        fn children(node: &Self::Node) -> &Tree<Self> { node }
        fn children_mut(node: &mut Self::Node) -> &mut Tree<Self> { node }
    }

    #[derive(Debug, Clone, Copy, PartialEq, Eq)]
    enum Bar {
        Value(usize),
        Branch(char),
    }
    impl From<Strong<Foo>> for Bar {
        fn from(value: Strong<Foo>) -> Self {
            match &*value.read() {
                Foo::Value(n) => Bar::Value(*n),
                Foo::Branch(id, _) => Bar::Branch(*id),
            }
        }
    }
    use Bar::*;

    fn generate_tree() -> Tree<Foo> {
        use Foo::*;
        Tree::from_iter([
            Value(0),
            Value(1),
            Branch('a', Tree::from_iter([
                Value(2),
                Value(3),
                Branch('b', Tree::from_iter([
                    Value(4),
                    Value(5),
                    Value(6),
                ])),
                Value(7),
            ])),
            Value(8),
            Branch('c', Tree::from_iter([
                Value(9),
            ])),
            Value(10),
        ])
    }

    #[cfg(test)]
    mod shallow {
        use super::*;

        #[test]
        fn test_shallow() {
            let flat: Vec<Bar> = generate_tree()
                .shallow_iter()
                .map(|x| x.into())
                .collect();

            assert_eq!(&flat[..], &[
                Value(0),
                Value(1),
                Branch('a'),
                Value(8),
                Branch('c'),
                Value(10),
            ]);
        }

        #[test]
        fn test_shallow_cforward() {
            let flat: Vec<Bar> = generate_tree()
                .shallow_iter()
                .cdir::<CForward>()
                .map(|x| x.into())
                .collect();

            assert_eq!(&flat[..], &[
                Value(0),
                Value(1),
                Branch('a'),
                Value(8),
                Branch('c'),
                Value(10),
            ]);
        }

        #[test]
        fn test_shallow_creverse() {
            let flat: Vec<Bar> = generate_tree()
                .shallow_iter()
                .cdir::<CReverse>()
                .map(|x| x.into())
                .collect();

            assert_eq!(&flat[..], &[
                Value(10),
                Branch('c'),
                Value(8),
                Branch('a'),
                Value(1),
                Value(0),
            ]);
        }
    }

    #[cfg(test)]
    mod dfs {
        use super::*;

        #[test]
        fn test_dfs() {
            let flat: Vec<Bar> = generate_tree()
                .dfs_iter(|_| true)
                .map(|x| x.into())
                .collect();

            assert_eq!(&flat[..], &[
                Value(0),
                Value(1),
                Branch('a'),
                    Value(2),
                    Value(3),
                    Branch('b'),
                        Value(4),
                        Value(5),
                        Value(6),
                    Value(7),
                Value(8),
                Branch('c'),
                    Value(9),
                Value(10),
            ]);
        }

        #[test]
        fn test_dfs_enumerate_depth() {
            let flat: Vec<(usize, Bar)> = generate_tree()
                .dfs_iter(|_| true)
                .enumerate_depth()
                .map(|(n, x)| (n, x.into()))
                .collect();

            assert_eq!(&flat[..], &[
                (0, Value(0)),
                (0, Value(1)),
                (0, Branch('a')),
                    (1, Value(2)),
                    (1, Value(3)),
                    (1, Branch('b')),
                        (2, Value(4)),
                        (2, Value(5)),
                        (2, Value(6)),
                    (1, Value(7)),
                (0, Value(8)),
                (0, Branch('c')),
                    (1, Value(9)),
                (0, Value(10)),
            ]);
        }

        #[test]
        fn test_dfs_cforward() {
            let flat: Vec<Bar> = generate_tree()
                .dfs_iter(|_| true)
                .cdir::<CForward>()
                .map(|x| x.into())
                .collect();

            assert_eq!(&flat[..], &[
                Value(0),
                Value(1),
                Branch('a'),
                    Value(2),
                    Value(3),
                    Branch('b'),
                        Value(4),
                        Value(5),
                        Value(6),
                    Value(7),
                Value(8),
                Branch('c'),
                    Value(9),
                Value(10),
            ]);
        }

        #[test]
        fn test_dfs_creverse() {
            let flat: Vec<Bar> = generate_tree()
                .dfs_iter(|_| true)
                .cdir::<CReverse>()
                .map(|x| x.into())
                .collect();

            assert_eq!(&flat[..], &[
                Value(10),
                Branch('c'),
                    Value(9),
                Value(8),
                Branch('a'),
                    Value(7),
                    Branch('b'),
                        Value(6),
                        Value(5),
                        Value(4),
                    Value(3),
                    Value(2),
                Value(1),
                Value(0),
            ]);
        }
    }

    #[cfg(test)]
    mod bfs {
        use super::*;

        #[test]
        fn test_bfs() {
            let flat: Vec<Bar> = generate_tree()
                .bfs_iter(|_| true)
                .map(|x| x.into())
                .collect();

            assert_eq!(&flat[..], &[
                Value(0),
                Value(1),
                Branch('a'),
                Value(8),
                Branch('c'),
                Value(10),
                    Value(2),
                    Value(3),
                    Branch('b'),
                    Value(7),
                    Value(9),
                        Value(4),
                        Value(5),
                        Value(6),
            ])
        }

        #[test]
        fn test_bfs_enumerate_depth() {
            let flat: Vec<(usize, Bar)> = generate_tree()
                .bfs_iter(|_| true)
                .enumerate_depth()
                .map(|(n, x)| (n, x.into()))
                .collect();

            assert_eq!(&flat[..], &[
                (0, Value(0)),
                (0, Value(1)),
                (0, Branch('a')),
                (0, Value(8)),
                (0, Branch('c')),
                (0, Value(10)),
                    (1, Value(2)),
                    (1, Value(3)),
                    (1, Branch('b')),
                    (1, Value(7)),
                    (1, Value(9)),
                        (2, Value(4)),
                        (2, Value(5)),
                        (2, Value(6)),
            ])
        }
    }
}
