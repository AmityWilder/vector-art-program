//! Wrappers for [`Rc<RefCell<T>>`] to enable finer tuning of mutability.
//! These exist because Rust doesn't appear to have a good means of enabling
//! multiple mutable references in a guaranteed single-threaded environment.

use std::{cell::{Ref, RefCell, RefMut}, fmt, rc::{self, Rc}};

// --- Owned ---

/// A strong, owning reference
pub trait Owned<T: ?Sized> {
    type Downgraded: Unowned<T>;
    type Read<'a>: std::ops::Deref<Target = T> + 'a where Self: 'a;
    type Cloned: Owned<T>;
    fn read(&self) -> Self::Read<'_>;
    fn downgrade(&self) -> Self::Downgraded;
    fn clone(&self) -> Self::Cloned;
}

/// An owned reference to an immutable `T`
#[derive(Clone)]
pub struct Strong<T: ?Sized>(Rc<RefCell<T>>);

impl<T: ?Sized + fmt::Debug> fmt::Debug for Strong<T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        fmt::Debug::fmt(&*self.read(), f)
    }
}

impl<T: ?Sized + fmt::Display> fmt::Display for Strong<T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        fmt::Display::fmt(&*self.read(), f)
    }
}

impl<T> Strong<T> {
    pub fn new(value: T) -> Self {
        Self(Rc::new(RefCell::new(value)))
    }

    pub fn new_cyclic<F: FnOnce(&Weak<T>) -> T>(data_fn: F) -> Self {
        Self(Rc::new_cyclic(|me| RefCell::new(data_fn(&Weak(me.clone())))))
    }
}

impl<T: ?Sized> Strong<T> {
    pub fn read(&self) -> Ref<'_, T> {
        self.0.borrow()
    }

    pub fn downgrade(&self) -> Weak<T> {
        Weak(Rc::downgrade(&self.0))
    }

    pub fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}

impl<T: ?Sized> Owned<T> for Strong<T> {
    type Downgraded = Weak<T>;
    type Read<'a> = Ref<'a, T> where Self: 'a;
    type Cloned = Self;

    #[inline]
    fn read(&self) -> Self::Read<'_> {
        self.read()
    }

    #[inline]
    fn downgrade(&self) -> Self::Downgraded {
        self.downgrade()
    }

    #[inline]
    fn clone(&self) -> Self::Cloned {
        self.clone()
    }
}

impl<T: ?Sized> From<StrongMut<T>> for Strong<T> {
    fn from(value: StrongMut<T>) -> Self {
        Self(value.0)
    }
}

impl<T: ?Sized> PartialEq for Strong<T> {
    fn eq(&self, other: &Self) -> bool {
        Rc::ptr_eq(&self.0, &other.0)
    }
}
impl<T: ?Sized> Eq for Strong<T> {}

/// An owned reference to a mutable `T`
pub struct StrongMut<T: ?Sized>(Rc<RefCell<T>>);

impl<T: ?Sized + fmt::Debug> fmt::Debug for StrongMut<T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        fmt::Debug::fmt(&*self.read(), f)
    }
}

impl<T: ?Sized + fmt::Display> fmt::Display for StrongMut<T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        fmt::Display::fmt(&*self.read(), f)
    }
}

impl<T> StrongMut<T> {
    pub fn new(value: T) -> Self {
        Self(Rc::new(RefCell::new(value)))
    }

    pub fn new_cyclic<F: FnOnce(&WeakMut<T>) -> T>(data_fn: F) -> Self {
        Self(Rc::new_cyclic(|me| RefCell::new(data_fn(&WeakMut(me.clone())))))
    }

    pub fn into_inner(self) -> Option<T> {
        Rc::into_inner(self.0).map(|cell| cell.into_inner())
    }
}

impl<T: ?Sized> StrongMut<T> {
    pub fn read(&self) -> Ref<'_, T> {
        self.0.borrow()
    }
    pub fn write(&mut self) -> RefMut<'_, T> {
        self.0.borrow_mut()
    }

    pub fn get_mut(&mut self) -> Option<&mut T> {
        Rc::get_mut(&mut self.0).map(|cell| cell.get_mut())
    }
    pub fn make_mut(&mut self) -> &mut T where T: Clone {
        Rc::make_mut(&mut self.0).get_mut()
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

pub trait AsConst {
    type Output;
    fn as_const(self) -> Self::Output;
}

impl<'a, T: ?Sized> AsConst for &'a StrongMut<T> {
    type Output = &'a Strong<T>;
    /// Use a [`StrongMut`] as a [`Strong`]
    fn as_const(self) -> Self::Output {
        unsafe { &*(self as *const StrongMut<T> as *const Strong<T>) }
    }
}

impl<T: ?Sized> Owned<T> for StrongMut<T> {
    type Downgraded = WeakMut<T>;
    type Read<'a> = Ref<'a, T> where Self: 'a;
    type Cloned = Strong<T>;

    #[inline]
    fn read(&self) -> Self::Read<'_> {
        self.read()
    }

    #[inline]
    fn downgrade(&self) -> Self::Downgraded {
        self.downgrade()
    }

    #[inline]
    fn clone(&self) -> Self::Cloned {
        self.clone()
    }
}

impl<T: ?Sized> PartialEq for StrongMut<T> {
    fn eq(&self, other: &Self) -> bool {
        Rc::ptr_eq(&self.0, &other.0)
    }
}
impl<T: ?Sized> Eq for StrongMut<T> {}


impl<T: ?Sized> PartialEq<Strong<T>> for StrongMut<T> {
    fn eq(&self, other: &Strong<T>) -> bool {
        Rc::ptr_eq(&self.0, &other.0)
    }
}

impl<T: ?Sized> PartialEq<StrongMut<T>> for Strong<T> {
    fn eq(&self, other: &StrongMut<T>) -> bool {
        Rc::ptr_eq(&self.0, &other.0)
    }
}

// --- Unowned ---

pub trait Unowned<T: ?Sized> {
    type Upgraded: Owned<T>;
    type Cloned: Unowned<T>;
    fn upgrade(&self) -> Option<Self::Upgraded>;
    fn clone(&self) -> Self::Cloned;
}

/// An unowned reference to an immutable `T`
pub struct Weak<T: ?Sized>(rc::Weak<RefCell<T>>);

impl<T: ?Sized> Weak<T> {
    pub fn upgrade(&self) -> Option<Strong<T>> {
        self.0.upgrade().map(|rc| Strong(rc))
    }

    pub fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}

/// A weak, unowning reference
impl<T: ?Sized> Unowned<T> for Weak<T> {
    type Upgraded = Strong<T>;
    type Cloned = Self;

    #[inline]
    fn upgrade(&self) -> Option<Self::Upgraded> {
        self.upgrade()
    }

    #[inline]
    fn clone(&self) -> Self::Cloned {
        self.clone()
    }
}

impl<T: ?Sized> From<WeakMut<T>> for Weak<T> {
    fn from(value: WeakMut<T>) -> Self {
        Self(value.0)
    }
}

/// An unowned reference to a mutable `T`
pub struct WeakMut<T: ?Sized>(rc::Weak<RefCell<T>>);

impl<T: ?Sized> WeakMut<T> {
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

impl<T: ?Sized> Unowned<T> for WeakMut<T> {
    type Upgraded = StrongMut<T>;
    type Cloned = Weak<T>;

    fn upgrade(&self) -> Option<Self::Upgraded> {
        self.upgrade()
    }

    fn clone(&self) -> Self::Cloned {
        self.clone()
    }
}
