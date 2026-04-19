//! Generic view type that wraps a shared-memory page with borrow or owned storage.

use crate::views::storage::Storage;

/// A typed, lifetime-aware view over a shared-memory page `T`.
///
/// `View` is the common foundation for [`PhysicsView`](super::PhysicsView),
/// [`GraphicsView`](super::GraphicsView), and [`StaticView`](super::StaticView).
/// It can either borrow a page directly from a live shared-memory mapping
/// ([`View::borrowed`]) or own a heap-allocated snapshot copy
/// ([`View::snapshot`]).
///
/// # Lifetimes
///
/// A borrowed `View<'a, T>` is valid only as long as the
/// [`ACEvoSharedMemoryMapper`](crate::ACEvoSharedMemoryMapper) that produced it.
/// A snapshot `View<'static, T>` is independent and can be stored, sent across
/// threads, etc.
///
/// # Serde
///
/// With the `serde` feature enabled:
///
/// - **Serialize** is available for any `View<'a, T>` where `T: Serialize`. The view
///   serializes transparently as `T` — the storage wrapper is invisible to the output.
/// - **Deserialize** produces a `View<'static, T>` (owned snapshot). Deserializing a
///   borrowed view is not possible because the data must be heap-allocated.
///
/// Typical usage: call [`snapshot`](View::snapshot) to capture a frame, then serialize
/// it for logging or replay, and deserialize it later without the game running.
///
/// # Example
///
/// ```no_run
/// use acevo_shared_memory::ACEvoSharedMemoryMapper;
///
/// let mapper = ACEvoSharedMemoryMapper::open().unwrap();
///
/// // Borrowed view — zero copy, tied to `mapper`.
/// let live = mapper.physics();
/// println!("Live speed: {:.1} km/h", live.raw().speedKmh);
///
/// // Snapshot — heap copy, independent of `mapper`.
/// let snap = live.snapshot();
/// drop(mapper);
/// println!("Captured speed: {:.1} km/h", snap.raw().speedKmh);
/// ```
pub struct View<'a, T: Copy> {
    pub(crate) data: Storage<'a, T>,
}

impl<'a, T: Copy> View<'a, T> {
    /// Creates a view that borrows `page` for lifetime `'a`.
    ///
    /// Intended for internal use by the mapper; prefer the typed constructors on
    /// [`ACEvoSharedMemoryMapper`](crate::ACEvoSharedMemoryMapper).
    pub fn borrowed(page: &'a T) -> Self {
        Self {
            data: Storage::Borrowed(page),
        }
    }

    /// Returns a shared reference to the underlying page struct.
    pub fn inner(&self) -> &T {
        self.data.as_ref()
    }

    /// Returns a shared reference to the underlying raw C struct.
    ///
    /// Equivalent to [`View::inner`]. Use this when you need fields that are not
    /// yet covered by a typed accessor method.
    ///
    /// # Example
    ///
    /// ```no_run
    /// use acevo_shared_memory::ACEvoSharedMemoryMapper;
    ///
    /// let mapper = ACEvoSharedMemoryMapper::open().unwrap();
    /// let graphics = mapper.graphics();
    /// let raw = graphics.raw();
    /// println!("Fuel: {:.2} L", raw.fuel_liter_current_quantity);
    /// ```
    pub fn raw(&self) -> &T {
        self.inner()
    }

    /// Returns a heap-allocated snapshot with a `'static` lifetime.
    ///
    /// The current state of the page is copied into a `Box<T>`. The resulting
    /// view is completely independent of the mapper and can outlive it.
    ///
    /// # Example
    ///
    /// ```no_run
    /// use acevo_shared_memory::ACEvoSharedMemoryMapper;
    ///
    /// let mapper = ACEvoSharedMemoryMapper::open().unwrap();
    /// let snap = mapper.physics().snapshot();
    /// drop(mapper); // mapper closed, snapshot still valid
    /// println!("RPM: {}", snap.raw().rpms);
    /// ```
    pub fn snapshot(&self) -> View<'static, T> {
        let inner = self.data.snapshot();
        View { data: inner }
    }
}

#[cfg(feature = "serde")]
impl<'a, T: Copy + serde::Serialize> serde::Serialize for View<'a, T> {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        self.inner().serialize(serializer)
    }
}

#[cfg(feature = "serde")]
impl<'de, T: Copy + serde::de::DeserializeOwned> serde::Deserialize<'de> for View<'static, T> {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let t = T::deserialize(deserializer)?;
        Ok(View {
            data: Storage::Owned(Box::new(t)),
        })
    }
}
