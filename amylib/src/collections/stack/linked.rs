use super::Stack;

struct Link<T> {
    value: T,
    next: Option<Box<Link<T>>>,
}

pub struct LinkedStack<T>(Option<Box<Link<T>>>);

impl<T> LinkedStack<T> {
    pub const fn new() -> Self {
        Self(None)
    }

    pub fn push(&mut self, value: T) {
        self.0 = Some(Box::new(Link { value, next: self.0.take() }));
    }

    pub fn pop(&mut self) -> Option<T> {
        if let Some(curr) = self.0.take() {
            let Link { value, next } = *curr;
            self.0 = next;
            return Some(value);
        }
        None
    }

    pub fn top(&mut self) -> Option<&T> {
        self.0.as_ref().map(|x| &x.value)
    }
    pub fn top_mut(&mut self) -> Option<&mut T> {
        self.0.as_mut().map(|x| &mut x.value)
    }

    pub fn clear(&mut self) {
        _ = self.0.take();
    }

    pub fn is_empty(&self) -> bool {
        self.0.is_none()
    }
}

impl<T> Stack<T> for LinkedStack<T> {
    #[inline] fn new() -> Self { Self::new() }
    #[inline] fn push(&mut self, value: T) { self.push(value); }
    #[inline] fn pop(&mut self) -> Option<T> { self.pop() }
    #[inline] fn top(&mut self) -> Option<&T> { self.top() }
    #[inline] fn top_mut(&mut self) -> Option<&mut T> { self.top_mut() }
    #[inline] fn clear(&mut self) { self.clear(); }
    #[inline] fn is_empty(&self) -> bool { self.is_empty() }
}
