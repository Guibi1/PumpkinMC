use async_trait::async_trait;
use pumpkin_util::math::position::BlockPos;

use super::BlockEntity;

pub struct BedBlockEntity {
    pub position: BlockPos,
}

impl BedBlockEntity {
    pub const ID: &'static str = "minecraft:bed";
}

#[async_trait]
impl BlockEntity for BedBlockEntity {
    fn identifier(&self) -> &'static str {
        Self::ID
    }

    fn get_position(&self) -> BlockPos {
        self.position
    }

    fn from_nbt(_nbt: &pumpkin_nbt::compound::NbtCompound, position: BlockPos) -> Self
    where
        Self: Sized,
    {
        Self { position }
    }

    async fn write_nbt(&self, _nbt: &mut pumpkin_nbt::compound::NbtCompound) {}

    fn as_any(&self) -> &dyn std::any::Any {
        self
    }
}
