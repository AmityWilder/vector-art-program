use crate::prelude::*;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
#[must_use]
pub struct IRect2 {
    pub min: UVector2,
    pub max: UVector2,
}
