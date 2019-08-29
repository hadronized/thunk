use std::cell::RefCell;
use std::rc::Rc;

enum Thunk<'a, T> {
  Unevaluated(Box<dyn FnOnce() -> T + 'a>),
  Evaluated(T)
}

impl<'a, T> Thunk<'a, T> {
  /// Evaluate the thunk and make it strict.
  fn evaluate(self) -> Self {
    match self {
      Thunk::Unevaluated(get) => Thunk::Evaluated(get()),
      _ => self
    }
  }

  /// Consider the thunk evaluated and get the underlying value.
  fn evaluated(&self) -> &T {
    match *self {
      Thunk::Evaluated(ref value) => value,
      _ => panic!("not evaluated thunk")
    }
  }
}

pub struct Lazy<'a, T>(Rc<RefCell<Thunk<'a, T>>>);

impl<'a, T> Lazy<'a, T> {
  /// Create a new thunk from a closure (laziness).
  pub fn lazy<F>(f: F) -> Self where F: FnOnce() -> T + 'a {
    Lazy(Rc::new(RefCell::new(Thunk::Unevaluated(Box::new(f)))))
  }

  /// Create a new thunk from a value (strictness).
  pub fn strict(x: T) -> Self {
    Lazy(Rc::new(RefCell::new(Thunk::Evaluated(x))))
  }

  /// Get a reference on the lazy value.
  pub fn as_ref(&self) -> &T {
    unsafe {
      let mut thunk = self.0.borrow_mut();
      let rethunk = std::ptr::read(&*thunk);
      std::ptr::write(&mut *thunk, rethunk.evaluate());
    }

    unsafe {
      &*(self.0.borrow().evaluated() as *const _) // mega lol
    }
  }
}
