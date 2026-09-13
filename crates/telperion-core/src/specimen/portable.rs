//! Native address-width sentinels have explicit portable representations.
pub mod index {
    use serde::{Deserialize, Deserializer, Serializer};
    pub fn serialize<S: Serializer>(value: &usize, serializer: S) -> Result<S::Ok, S::Error> {
        serializer.serialize_u64(if *value == usize::MAX {
            u64::MAX
        } else {
            *value as u64
        })
    }
    pub fn deserialize<'de, D: Deserializer<'de>>(deserializer: D) -> Result<usize, D::Error> {
        let value = u64::deserialize(deserializer)?;
        if value == u64::MAX {
            Ok(usize::MAX)
        } else {
            usize::try_from(value).map_err(serde::de::Error::custom)
        }
    }
}
pub mod indices {
    use crate::tree::NodeKey;
    use serde::{Deserialize, Deserializer, Serialize, Serializer};
    use slotmap::DenseSlotMap;
    // Slots are u32 already: UINT32_MAX is an impossible node-buffer index.
    pub fn serialize<S: Serializer>(
        value: &DenseSlotMap<NodeKey, usize>,
        serializer: S,
    ) -> Result<S::Ok, S::Error> {
        let mut portable = value.clone();
        for index in portable.values_mut() {
            if *index == usize::MAX {
                *index = u32::MAX as usize;
            }
        }
        portable.serialize(serializer)
    }
    pub fn deserialize<'de, D: Deserializer<'de>>(
        deserializer: D,
    ) -> Result<DenseSlotMap<NodeKey, usize>, D::Error> {
        let mut value = DenseSlotMap::<NodeKey, usize>::deserialize(deserializer)?;
        for index in value.values_mut() {
            if *index == u32::MAX as usize {
                *index = usize::MAX;
            }
        }
        Ok(value)
    }
}
