mod data;
use std::sync::Arc;

pub use data::Data;

use crate::Result;
use ruma::{RoomAliasId, RoomId};

pub struct Service {
    db: Arc<dyn Data>,
}

impl Service {
    #[tracing::instrument(skip(self))]
    pub fn set_alias(&self, alias: &RoomAliasId, room_id: &RoomId) -> Result<()> {
        self.db.set_alias(alias, room_id)
    }

    #[tracing::instrument(skip(self))]
    pub fn remove_alias(&self, alias: &RoomAliasId) -> Result<()> {
        self.db.remove_alias(alias)
    }

    #[tracing::instrument(skip(self))]
    pub fn resolve_local_alias(&self, alias: &RoomAliasId) -> Result<Option<Box<RoomId>>> {
        self.db.resolve_local_alias(alias)
    }

    #[tracing::instrument(skip(self))]
    pub fn local_aliases_for_room<'a>(
        &'a self,
        room_id: &RoomId,
    ) -> impl Iterator<Item = Result<Box<RoomAliasId>>> + 'a {
        self.db.local_aliases_for_room(room_id)
    }
}
