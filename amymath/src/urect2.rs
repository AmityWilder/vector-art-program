use crate::prelude::*;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
#[must_use]
pub struct URect2 {
    pub min: UVector2,
    pub max: UVector2,
}
