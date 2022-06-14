use serde::{de::Deserializer, Deserialize};
use std::collections::HashMap;

#[derive(Deserialize, Debug)]
#[serde(untagged)]
/// Fake HashMap to convert empty lists returned by the API into empty HashMaps
/// The API currently may return a list instead of an empty map for endpoints which should
/// always return maps. This method either passes on the detected HashMap or
/// replaces the empty list with an empty HashMap.
pub enum FakeHashMap<K, V>
where
    K: std::cmp::Eq + std::hash::Hash,
{
    HashMap(HashMap<K, V>),
    EmptyList(Vec<V>),
}

impl<'a, K, V> FakeHashMap<K, V>
where
    K: std::cmp::Eq + std::hash::Hash,
{
    pub fn into_hash_map(self) -> HashMap<K, V> {
        self.into()
    }
}

impl<K, V> From<FakeHashMap<K, V>> for HashMap<K, V>
where
    K: std::cmp::Eq + std::hash::Hash,
{
    fn from(item: FakeHashMap<K, V>) -> Self {
        match item {
            FakeHashMap::HashMap(h) => h,
            FakeHashMap::EmptyList(_) => HashMap::new(),
        }
    }
}

pub fn deserialize_fake_hash_map<
    'de,
    D: Deserializer<'de>,
    K: std::cmp::Eq + std::hash::Hash + Deserialize<'de>,
    V: Deserialize<'de>,
>(
    deserializer: D,
) -> Result<HashMap<K, V>, D::Error> {
    let result = FakeHashMap::deserialize(deserializer)?;
    Ok(result.into())
}
