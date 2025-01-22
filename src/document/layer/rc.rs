use std::{cell::RefCell, rc::{Rc, Weak}};
use super::Layer;

/// An owned reference to an immutable [`Layer`]
#[derive(Clone)]
pub struct StrongLayer(Rc<RefCell<Layer>>);

impl StrongLayer {
    pub fn new(layer: Layer) -> Self {
        Self(Rc::new(RefCell::new(layer)))
    }

    pub fn read<'a>(&'a self) -> impl std::ops::Deref<Target = Layer> + 'a {
        self.0.borrow()
    }

    pub fn downgrade(&self) -> WeakLayer {
        WeakLayer(Rc::downgrade(&self.0))
    }
}

impl From<StrongLayerMut> for StrongLayer {
    fn from(value: StrongLayerMut) -> Self {
        Self(value.0)
    }
}

/// An unowned reference to an immutable [`Layer`]
#[derive(Clone)]
pub struct WeakLayer(Weak<RefCell<Layer>>);

impl WeakLayer {
    pub fn upgrade(&self) -> Option<StrongLayer> {
        self.0.upgrade().map(|rc| StrongLayer(rc))
    }
}

impl From<WeakLayerMut> for WeakLayer {
    fn from(value: WeakLayerMut) -> Self {
        Self(value.0)
    }
}

/// An owned reference to a mutable [`Layer`]
pub struct StrongLayerMut(Rc<RefCell<Layer>>);

impl StrongLayerMut {
    pub fn new(layer: Layer) -> Self {
        Self(Rc::new(RefCell::new(layer)))
    }

    pub fn read<'a>(&'a self) -> impl std::ops::Deref<Target = Layer> + 'a {
        self.0.borrow()
    }

    pub fn write<'a>(&'a mut self) -> impl std::ops::DerefMut<Target = Layer> + 'a {
        self.0.borrow_mut()
    }

    pub fn downgrade(&self) -> WeakLayerMut {
        WeakLayerMut(Rc::downgrade(&self.0))
    }

    pub fn clone(&self) -> StrongLayer {
        StrongLayer(self.0.clone())
    }

    pub fn clone_mut(&self) -> Self {
        Self(self.0.clone())
    }
}

/// An unowned reference to a mutable [`Layer`]
pub struct WeakLayerMut(Weak<RefCell<Layer>>);

impl WeakLayerMut {
    pub fn upgrade(&self) -> Option<StrongLayerMut> {
        self.0.upgrade().map(|rc| StrongLayerMut(rc))
    }

    pub fn clone(&self) -> WeakLayer {
        WeakLayer(self.0.clone())
    }

    pub fn clone_mut(&self) -> Self {
        Self(self.0.clone())
    }
}
