#![feature(maybe_uninit_uninit_array, maybe_uninit_array_assume_init, assert_matches)]

pub mod collections;
pub mod iter;
pub mod io;
pub mod ops;
pub mod math;

/// Wrappers for [`Rc<RefCell<T>>`] to enable finer tuning of mutability.
///
/// These exist because Rust allows internal mutability for all [`Rc`] holders, which is
/// annoying when I want to have an internally-immutable reference to a [`Rc`] that is
/// only holding a [`RefCell`] because *someone else* needs to be able to mutate it.
///
/// - [`Strong`]:    A [`Rc<RefCell<T>>`]   with [`Rc<T>`]            mutability.
/// - [`StrongMut`]: A [`Rc<RefCell<T>>`]   with [`Rc<RefCell<T>>`]   mutability.
/// - [`Weak`]:      A [`Weak<RefCell<T>>`] with [`Weak<T>`]          mutability.
/// - [`WeakMut`]:   A [`Weak<RefCell<T>>`] with [`Weak<RefCell<T>>`] mutability.
pub mod rc;

pub mod prelude {
    pub use crate::{
        iter::{
            directed::*,
            reversible::*,
        },
        io::*,
        rc::*,
        math::*,
    };
}
