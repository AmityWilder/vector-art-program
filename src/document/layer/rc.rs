use std::{cell::RefCell, rc::{self, Rc}};

/// An owned reference to an immutable `T`
#[derive(Clone)]
pub struct Strong<T>(Rc<RefCell<T>>);

impl<T> Strong<T> {
    pub fn new(layer: T) -> Self {
        Self(Rc::new(RefCell::new(layer)))
    }

    pub fn read<'a>(&'a self) -> impl std::ops::Deref<Target = T> + 'a {
        self.0.borrow()
    }

    pub fn downgrade(&self) -> Weak<T> {
        Weak(Rc::downgrade(&self.0))
    }
}

impl<T> From<StrongMut<T>> for Strong<T> {
    fn from(value: StrongMut<T>) -> Self {
        Self(value.0)
    }
}

/// An unowned reference to an immutable `T`
#[derive(Clone)]
pub struct Weak<T>(rc::Weak<RefCell<T>>);

impl<T> Weak<T> {
    pub fn upgrade(&self) -> Option<Strong<T>> {
        self.0.upgrade().map(|rc| Strong(rc))
    }
}

impl<T> From<WeakMut<T>> for Weak<T> {
    fn from(value: WeakMut<T>) -> Self {
        Self(value.0)
    }
}

/// An owned reference to a mutable `T`
pub struct StrongMut<T>(Rc<RefCell<T>>);

impl<T> StrongMut<T> {
    pub fn new(layer: T) -> Self {
        Self(Rc::new(RefCell::new(layer)))
    }

    pub fn read<'a>(&'a self) -> impl std::ops::Deref<Target = T> + 'a {
        self.0.borrow()
    }

    pub fn write<'a>(&'a mut self) -> impl std::ops::DerefMut<Target = T> + 'a {
        self.0.borrow_mut()
    }

    pub fn downgrade(&self) -> WeakMut<T> {
        WeakMut(Rc::downgrade(&self.0))
    }

    pub fn clone(&self) -> Strong<T> {
        Strong(self.0.clone())
    }

    pub fn clone_mut(&self) -> Self {
        Self(self.0.clone())
    }
}

/// An unowned reference to a mutable `T`
pub struct WeakMut<T>(rc::Weak<RefCell<T>>);

impl<T> WeakMut<T> {
    pub fn upgrade(&self) -> Option<StrongMut<T>> {
        self.0.upgrade().map(|rc| StrongMut(rc))
    }

    pub fn clone(&self) -> Weak<T> {
        Weak(self.0.clone())
    }

    pub fn clone_mut(&self) -> Self {
        Self(self.0.clone())
    }
}
