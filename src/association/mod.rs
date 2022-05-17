use crate::Expr;
use indexmap::map::IndexMap;
use std::collections::{BTreeMap, HashMap};
use std::fmt::{Debug, Display, Formatter};
use std::iter::FromIterator;
use std::ops::{Deref, DerefMut};

/// A map from variable names to expressions.
#[derive(Debug, Clone)]
pub struct Association {
    /// key -> (is_delayed, value)
    pub(crate) records: IndexMap<Expr, (bool, Expr)>,
}

impl Default for Association {
    fn default() -> Self {
        Self {
            records: IndexMap::new(),
        }
    }
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
    /// Build a new Association
    pub fn new() -> Self {
        Self::default()
    }
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
    /// Convert to [`Expr`]
    pub fn as_expr(&self) -> Expr {
        Expr::from(self.clone())
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

impl Display for Association {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let mut indent = 0;
        self.fmt_indent(f, f.alternate(), &mut indent)
    }
}

impl Association {
    pub(crate) fn fmt_indent(
        &self,
        f: &mut Formatter<'_>,
        alternate: bool,
        indent: &mut usize,
    ) -> std::fmt::Result {
        write!(f, "<|")?;
        if alternate {
            *indent += 4;
            writeln!(f)?
        }
        for (i, (key, (rule, value))) in self.records.iter().enumerate() {
            let is_last = i == self.records.len() - 1;
            if alternate {
                write!(f, "{}", " ".repeat(*indent))?
            }
            match rule {
                true => write!(f, "{} :> {}", key, value)?,
                false => write!(f, "{} -> {}", key, value)?,
            }
            if !is_last {
                match alternate {
                    true => writeln!(f, ",")?,
                    false => write!(f, ", ")?,
                }
            }
        }
        if alternate {
            *indent -= 4;
            writeln!(f)?
        }
        write!(f, "|>")
    }
}
