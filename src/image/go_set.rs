//Adding it here cause original code was unlicensed.
//Credits: https://github.com/ordovicia/oci-image-rs/blob/master/image-spec/src/go_set.rs
//Author of Goset: @ordovicia 

use std::{collections::HashSet, default::Default, hash::Hash, iter::FromIterator};

#[cfg(feature = "serde")]
use std::collections::HashMap;

#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

/// Set of `T` values. Results of serialization and de-serialization are equivalent to those of Go
/// language type `map[string]struct{}`.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct GoSet<T: Eq + Hash> {
    inner: HashSet<T>,
}

impl<T: Eq + Hash> GoSet<T> {
    /// Returns whether this set is empty.
    pub fn is_empty(&self) -> bool {
        self.inner.is_empty()
    }
}

impl<T: Eq + Hash> Default for GoSet<T> {
    fn default() -> Self {
        Self {
            inner: HashSet::new(),
        }
    }
}

impl<T: Eq + Hash> FromIterator<T> for GoSet<T> {
    fn from_iter<I: IntoIterator<Item = T>>(iter: I) -> Self {
        GoSet {
            inner: iter.into_iter().collect(),
        }
    }
}

impl<T: Eq + Hash> From<HashSet<T>> for GoSet<T> {
    fn from(set: HashSet<T>) -> Self {
        Self { inner: set }
    }
}

impl<T: Eq + Hash> Into<HashSet<T>> for GoSet<T> {
    fn into(self) -> HashSet<T> {
        self.inner
    }
}

#[cfg(feature = "serde")]
impl<T: Eq + Hash + Serialize> serde::Serialize for GoSet<T> {
    fn serialize<S: serde::Serializer>(&self, ser: S) -> Result<S::Ok, S::Error> {
        use serde::ser::SerializeMap;

        let empty: HashMap<(), ()> = HashMap::new();
        let mut map = ser.serialize_map(Some(self.inner.len()))?;
        for e in &self.inner {
            map.serialize_entry(e, &empty)?;
        }
        map.end()
    }
}

#[cfg(feature = "serde")]
impl<'de, T: Eq + Hash + Deserialize<'de>> serde::Deserialize<'de> for GoSet<T> {
    fn deserialize<D: serde::Deserializer<'de>>(deser: D) -> Result<Self, D::Error> {
        let map: HashMap<T, HashMap<(), ()>> = HashMap::deserialize(deser)?;
        let set = map.into_iter().map(|(e, _)| e).collect::<HashSet<T>>();
        Ok(set.into())
    }
}

#[cfg(all(feature = "serde", test))]
mod tests {
    use super::*;

    #[test]
    fn test_go_set_deser() {
        const JSON: &str = r#"{ "zero": {}, "one": {}, "two": {} }"#;
        let go_set: GoSet<String> = serde_json::from_str(JSON).unwrap();

        assert_eq!(
            go_set,
            ["zero", "one", "two"]
                .iter()
                .map(ToString::to_string)
                .collect::<GoSet<_>>()
        );
    }

    #[test]
    fn test_go_set_ser() {
        let go_set = GoSet {
            inner: [0, 1].iter().map(ToString::to_string).collect(),
        };

        let actual = serde_json::to_string(&go_set).unwrap();
        assert!(actual == r#"{"0":{},"1":{}}"# || actual == r#"{"1":{},"0":{}}"#);
    }
}

