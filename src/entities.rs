#[cfg(feature = "hashbrown")]
mod inline {
    use hashbrown::{HashMap, HashSet};
    pub type NodeIdSet = HashSet<crate::NodeId>;
    pub type HashSetFx<K> = HashSet<K>;
    pub type InnerHashMap<K, V> = HashMap<K, V>;
}

#[cfg(not(feature = "hashbrown"))]
mod inline {
    use foldhash::{HashMap, HashSet};
    pub type NodeIdSet = HashSet<crate::NodeId>;
    pub type HashSetFx<K> = HashSet<K>;
    pub type InnerHashMap<K, V> = HashMap<K, V>;
}

use std::ops::{Deref, DerefMut};

pub(crate) use inline::{HashSetFx, InnerHashMap, NodeIdSet};

//pub type DString = tendril::Tendril<tendril::fmt::UTF8, tendril::Atomic>;

use html5ever::{Attribute, QualName};
use tendril::{StrTendril, Tendril};

#[derive(PartialEq, Eq, PartialOrd, Ord, Clone, Debug, Default)]
pub struct StrWrap(pub(crate) Tendril<tendril::fmt::UTF8, tendril::Atomic>);

impl StrWrap {
    pub fn new() -> Self {
        StrWrap(Tendril::new())
    }
}

impl Deref for StrWrap {
    type Target = Tendril<tendril::fmt::UTF8, tendril::Atomic>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for StrWrap {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl From<StrTendril> for StrWrap {
    fn from(value: StrTendril) -> Self {
        StrWrap(value.into_send().into())
    }
}

impl From<String> for StrWrap {
    fn from(value: String) -> Self {
        let v = Tendril::from(value);
        StrWrap(v)
    }
}

impl From<&str> for StrWrap {
    fn from(value: &str) -> Self {
        let v = Tendril::from(value);
        StrWrap(v)
    }
}

impl From<StrWrap> for StrTendril {
    fn from(value: StrWrap) -> Self {
        value.0.into_send().into()
    }
}

/*impl<T> From<T> for DString where T: Into<StrTendril> {
    fn from(value: T) -> Self {
        let v: StrTendril = value.into();
        DString(v.into_send().into())
    }
}*/

/// A tag attribute, e.g. `class="test"` in `<div class="test" ...>`.
///
/// The namespace on the attribute name is almost always ns!("").
/// The tokenizer creates all attributes this way, but the tree
/// builder will adjust certain attribute names inside foreign
/// content (MathML, SVG).
#[derive(PartialEq, Eq, PartialOrd, Ord, Clone, Debug)]
pub struct DAttribute {
    /// The name of the attribute (e.g. the `class` in `<div class="test">`)
    pub name: QualName,
    /// The value of the attribute (e.g. the `"test"` in `<div class="test">`)
    pub value: StrWrap,
}

impl From<Attribute> for DAttribute {
    fn from(val: Attribute) -> Self {
        let v = val.value.into();
        Self {
            name: val.name,
            value: v,
        }
    }
}

impl From<DAttribute> for Attribute {
    fn from(val: DAttribute) -> Self {
        Self {
            name: val.name,
            value: val.value.into(),
        }
    }
}
