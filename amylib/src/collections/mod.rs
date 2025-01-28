pub mod stack;
pub mod tree;
/// non-public until stable
mod fixed_queue;

pub use {
    stack::{
        Stack,
        SizedStack,
        linked::LinkedStack,
        vec::VecStack,
        deque::VecDestack,
    },
    tree::{
        Recursive,
        RecursiveIterator,
        Tree,
    },
};
