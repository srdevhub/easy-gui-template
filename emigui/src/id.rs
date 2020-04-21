//! Emigui tracks widgets frame-to-frame using `Id`s.
//!
//! For instance, if you start dragging a slider one frame, emigui stores
//! the sldiers Id as the current interact_id so that next frame when
//! you move the mouse the same slider changes, even if the mouse has
//! moved outside the slider.
//!
//! For some widgets `Id`s are also used to GUIpersist some state about the
//! widgets, such as Window position or wether not a Foldable region is open.
//!
//! This implicated that the `Id`s must be unqiue.
//!
//! For simple things like sliders and buttons that don't have any memory and
//! doesn't move we can use the location of the widget as a source of identity.
//! For instance, a slider only needs a unique and persistent ID while you are
//! dragging the sldier. As long as it is still while moving, that is fine.
//!
//! For things that need to persist state even after moving (windows, foldables)
//! the location of the widgets is obviously not good enough. For instance,
//! a fodlable region needs to remember wether or not it is open even
//! if the layout next frame is different and the foldable is not lower down
//! on the screen.
//!
//! Then there are widgets that need no identifiers at all, like labels,
//! because they have no state nor are interacted with.
//!
//! So we have two type of Ids: PositionId and UniqueId.
//! TODO: have separate types for PositionId and UniqueId.

use std::{collections::hash_map::DefaultHasher, hash::Hash};

use crate::math::Pos2;

#[derive(Clone, Copy, Debug, Hash, Eq, PartialEq)]
pub struct Id(u64);

impl Id {
    pub fn background() -> Self {
        Self(0)
    }

    pub fn popup() -> Self {
        Self(1)
    }

    pub fn new<H: Hash>(source: H) -> Id {
        use std::hash::Hasher;
        let mut hasher = DefaultHasher::new();
        source.hash(&mut hasher);
        Id(hasher.finish())
    }

    pub fn with<H: Hash>(self, child: H) -> Id {
        use std::hash::Hasher;
        let mut hasher = DefaultHasher::new();
        hasher.write_u64(self.0);
        child.hash(&mut hasher);
        Id(hasher.finish())
    }

    pub fn from_pos(p: Pos2) -> Id {
        let x = p.x.round() as i32;
        let y = p.y.round() as i32;
        Id::new(&x).with(&y)
    }
}
