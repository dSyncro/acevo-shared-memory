//! Shared-memory mapper — opens and owns the three AC Evo telemetry segments.

use win_shared_memory::{SharedMemoryError, SharedMemoryLink};

use crate::{
    bindings::root::ks::{SPageFileGraphicEvo, SPageFilePhysics, SPageFileStaticEvo},
    views::{GraphicsView, PhysicsView, StaticView},
};

const STATIC_FILE: &str = "Local\\acevo_pmf_static";
const PHYSICS_FILE: &str = "Local\\acevo_pmf_physics";
const GRAPHICS_FILE: &str = "Local\\acevo_pmf_graphics";

/// Struct that owns the three Windows named shared-memory mappings used by AC Evo and provides
/// typed views over each one.
///
/// Create one `Mapper` per process — it keeps the OS handles open for as long as it
/// lives. Dropping it closes the handles.
///
/// # Errors
///
/// [`Mapper::open`] returns [`SharedMemoryError`] if AC Evo is not running or the
/// named objects cannot be found.
///
/// # Example
///
/// ```no_run
/// use acevo_shared_memory::ACEvoSharedMemoryMapper;
///
/// let mapper = ACEvoSharedMemoryMapper::open().expect("AC Evo must be running");
///
/// let physics  = mapper.physics();
/// let graphics = mapper.graphics();
/// let session  = mapper.static_data();
/// ```
#[derive(Debug)]
pub struct Mapper {
    physics_memory: SharedMemoryLink<SPageFilePhysics>,
    graphics_memory: SharedMemoryLink<SPageFileGraphicEvo>,
    static_memory: SharedMemoryLink<SPageFileStaticEvo>,
}

impl Mapper {
    /// Opens all three shared-memory segments (`physics`, `graphics`, `static`).
    ///
    /// AC Evo must be running and have written the named objects before this is called.
    ///
    /// # Errors
    ///
    /// Returns [`SharedMemoryError`] if any segment cannot be opened.
    pub fn open() -> Result<Self, SharedMemoryError> {
        let output = Self {
            static_memory: SharedMemoryLink::open(STATIC_FILE)?,
            physics_memory: SharedMemoryLink::open(PHYSICS_FILE)?,
            graphics_memory: SharedMemoryLink::open(GRAPHICS_FILE)?,
        };
        Ok(output)
    }

    /// Returns a [`PhysicsView`] that borrows directly from the physics segment.
    ///
    /// The view is valid for the lifetime of this `Mapper`. Use
    /// [`PhysicsView::snapshot`](crate::View::snapshot) if you need an
    /// owned copy.
    pub fn physics(&self) -> PhysicsView<'_> {
        let raw = self.physics_raw();
        PhysicsView::borrowed(raw)
    }

    /// Returns a [`GraphicsView`] that borrows directly from the graphics segment.
    ///
    /// The view is valid for the lifetime of this `Mapper`. Use
    /// [`GraphicsView::snapshot`](crate::View::snapshot) if you need an
    /// owned copy.
    pub fn graphics(&self) -> GraphicsView<'_> {
        let raw = self.graphics_raw();
        GraphicsView::borrowed(raw)
    }

    /// Returns a [`StaticView`] that borrows directly from the static segment.
    ///
    /// The static segment is written once when a session loads and does not change
    /// while driving. Use
    /// [`StaticView::snapshot`](crate::View::snapshot) if you need an
    /// owned copy.
    pub fn static_data(&self) -> StaticView<'_> {
        let raw = self.static_data_raw();
        StaticView::borrowed(raw)
    }

    /// Returns a raw pointer-derived reference to the physics page.
    ///
    /// Prefer [`Mapper::physics`] for safe, typed access. Use this only when you need
    /// a direct reference to the underlying C struct.
    ///
    /// # Safety
    ///
    /// The caller must ensure the data is not read while the simulator is in the middle
    /// of a write. In practice the simulator writes atomically per packet; for most
    /// telemetry use-cases this is acceptable.
    pub fn physics_raw(&self) -> &SPageFilePhysics {
        unsafe { self.physics_memory.get() }
    }

    /// Returns a raw pointer-derived reference to the graphics page.
    ///
    /// Prefer [`Mapper::graphics`] for safe, typed access.
    ///
    /// # Safety
    ///
    /// See [`Mapper::physics_raw`] for the safety note.
    pub fn graphics_raw(&self) -> &SPageFileGraphicEvo {
        unsafe { self.graphics_memory.get() }
    }

    /// Returns a raw pointer-derived reference to the static page.
    ///
    /// Prefer [`Mapper::static_data`] for safe, typed access.
    ///
    /// # Safety
    ///
    /// See [`Mapper::physics_raw`] for the safety note.
    pub fn static_data_raw(&self) -> &SPageFileStaticEvo {
        unsafe { self.static_memory.get() }
    }
}
