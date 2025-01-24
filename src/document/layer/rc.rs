use std::{cell::RefCell, rc::{self, Rc}};

pub trait StrongRef<T>: Sized {
    type Weak: WeakRef<T>;
    type Read<'a>: std::ops::Deref<Target = T> + 'a where Self: 'a;
    type Cloned: StrongRef<T>;
    fn new(value: T) -> Self;
    fn read(&self) -> Self::Read<'_>;
    fn downgrade(&self) -> Self::Weak;
    fn clone(&self) -> Self::Cloned;
}

pub trait WeakRef<T>: Sized {
    type Strong: StrongRef<T>;
    fn upgrade(&self) -> Option<Self::Strong>;
    type Cloned: WeakRef<T>;
    fn clone(&self) -> Self::Cloned;
}

/// An owned reference to an immutable `T`
#[derive(Clone)]
pub struct Strong<T>(Rc<RefCell<T>>);

impl<T> StrongRef<T> for Strong<T> {
    type Weak = Weak<T>;
    type Read<'a> = std::cell::Ref<'a, T> where Self: 'a;
    type Cloned = Self;

    fn new(value: T) -> Self {
        Self(Rc::new(RefCell::new(value)))
    }

    fn read(&self) -> Self::Read<'_> {
        self.0.borrow()
    }

    fn downgrade(&self) -> Weak<T> {
        Weak(Rc::downgrade(&self.0))
    }

    fn clone(&self) -> Self::Cloned {
        Self(self.0.clone())
    }
}

impl<T> From<StrongMut<T>> for Strong<T> {
    fn from(value: StrongMut<T>) -> Self {
        Self(value.0)
    }
}

/// An unowned reference to an immutable `T`
pub struct Weak<T>(rc::Weak<RefCell<T>>);

impl<T> WeakRef<T> for Weak<T> {
    type Strong = Strong<T>;
    type Cloned = Self;

    fn upgrade(&self) -> Option<Self::Strong> {
        self.0.upgrade().map(|rc| Strong(rc))
    }

    fn clone(&self) -> Self::Cloned {
        Self(self.0.clone())
    }
}

impl<T> From<WeakMut<T>> for Weak<T> {
    fn from(value: WeakMut<T>) -> Self {
        Self(value.0)
    }
}

/// An owned reference to a mutable `T`
pub struct StrongMut<T>(Rc<RefCell<T>>);

impl<T> StrongRef<T> for StrongMut<T> {
    type Weak = WeakMut<T>;
    type Read<'a> = std::cell::Ref<'a, T> where Self: 'a;
    type Cloned = Strong<T>;

    fn new(value: T) -> Self {
        Self(Rc::new(RefCell::new(value)))
    }

    fn read(&self) -> Self::Read<'_> {
        self.0.borrow()
    }

    fn downgrade(&self) -> WeakMut<T> {
        WeakMut(Rc::downgrade(&self.0))
    }

    fn clone(&self) -> Strong<T> {
        Strong(self.0.clone())
    }
}

impl<T> StrongMut<T> {
    pub fn write(&mut self) -> std::cell::RefMut<'_, T> {
        self.0.borrow_mut()
    }

    pub fn clone_mut(&self) -> Self {
        Self(self.0.clone())
    }
}

/// An unowned reference to a mutable `T`
pub struct WeakMut<T>(rc::Weak<RefCell<T>>);

impl<T> WeakRef<T> for WeakMut<T> {
    type Strong = StrongMut<T>;
    type Cloned = Weak<T>;

    fn upgrade(&self) -> Option<Self::Strong> {
        self.0.upgrade().map(|rc| StrongMut(rc))
    }

    fn clone(&self) -> Self::Cloned {
        Weak(self.0.clone())
    }
}

impl<T> WeakMut<T> {
    pub fn clone_mut(&self) -> Self {
        Self(self.0.clone())
    }
}
