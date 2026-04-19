//! Typed view wrappers over the three AC Evo shared-memory pages.
//!
//! Each view is a thin wrapper around a [`View<T>`](view::View) that adds
//! domain-specific accessor methods. Views borrow directly from the shared-memory
//! mapping and carry a lifetime tied to the [`crate::ACEvoSharedMemoryMapper`].
//!
//! | View | Underlying struct | Update rate |
//! |------|------------------|-------------|
//! | [`PhysicsView`] | `SPageFilePhysics` | Every simulation step |
//! | [`GraphicsView`] | `SPageFileGraphicEvo` | Every rendered frame |
//! | [`StaticView`] | `SPageFileStaticEvo` | Once per session load |

mod graphics_view;
mod physics_view;
mod static_view;
pub(crate) mod storage;
pub(crate) mod utils;
mod view;

pub use graphics_view::GraphicsView;
pub use physics_view::PhysicsView;
pub use static_view::StaticView;
pub use view::View;
