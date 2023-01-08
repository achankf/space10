use std::{
    collections::HashMap,
    ops::{Deref, DerefMut},
};

use super::{ResourceKey, ResourceSpace};

impl ResourceSpace {}

impl Deref for ResourceSpace {
    type Target = HashMap<ResourceKey, u32>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for ResourceSpace {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
