//! Internal storage backing for [`View`](super::view::View).
//!
//! Abstracts over a borrowed reference and a heap-owned copy so that both live
//! views and snapshots share the same `View<T>` type.

/// Internal storage for a view — either a borrow from shared memory or an owned copy.
pub enum Storage<'a, T: Copy> {
    /// Borrows the page directly from the shared-memory mapping.
    Borrowed(&'a T),
    /// Owns a heap-allocated copy of the page (produced by [`Storage::snapshot`]).
    Owned(Box<T>),
}

impl<'a, T: Copy> Storage<'a, T> {
    /// Returns a shared reference to the contained value regardless of variant.
    pub fn as_ref(&self) -> &T {
        match self {
            Storage::Borrowed(r) => r,
            Storage::Owned(b) => b,
        }
    }

    /// Copies the current value into a `Box` and returns an `Owned` storage with
    /// a `'static` lifetime.
    pub fn snapshot(&self) -> Storage<'static, T> {
        let owned = Box::new(*self.as_ref());
        Storage::Owned(owned)
    }
}
