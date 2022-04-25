use crate::Expr;
use indexmap::map::IndexMap;
use std::collections::{BTreeMap, HashMap};
use std::iter::FromIterator;
use std::ops::{Deref, DerefMut};

pub struct Association {
    /// key -> (is_delayed, value)
    records: IndexMap<Expr, (bool, Expr)>,
}

impl Deref for Association {
    type Target = IndexMap<Expr, (bool, Expr)>;

    fn deref(&self) -> &Self::Target {
        &self.records
    }
}

impl DerefMut for Association {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.records
    }
}

impl Association {
    /// Inserts a key-value pair into the association.
    pub fn insert<K, V>(&mut self, key: K, value: V)
    where
        K: Into<Expr>,
        V: Into<Expr>,
    {
        let key = key.into();
        let value = value.into();
        self.records.insert(key, (false, value));
    }
    /// Inserts a delayed key-value pair into the association.
    pub fn insert_delayed<K, V>(&mut self, key: K, value: V)
    where
        K: Into<Expr>,
        V: Into<Expr>,
    {
        let key = key.into();
        let value = value.into();
        self.records.insert(key, (true, value));
    }
}

macro_rules! map_like {
    ($($t:tt),*) => {
        $(
            impl<K, V> From<$t<K, V>> for Association
            where
                K: Into<Expr>,
                V: Into<Expr>,
            {
                fn from(map: $t<K, V>) -> Self {
                    Self {
                        records: IndexMap::from_iter(
                            map.into_iter().map(|(k, v)| (k.into(), (false, v.into()))),
                        ),
                    }
                }
            }
        )*
    };
}

map_like![HashMap, BTreeMap, IndexMap];


impl From<Association> for Expr {
    fn from(map: Association) -> Self {
        let mut elements = vec![];
        for (key, (rule, value)) in map.records {
            let item = match rule {
                true => Expr::rule(key, value),
                false => Expr::rule_delayed(key, value),
            };
            elements.push(item)
        }
        Expr::function("System`Association", elements)
    }
}
