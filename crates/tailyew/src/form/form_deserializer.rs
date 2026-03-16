use serde::Deserialize;
use serde::de::{Deserializer, Error, SeqAccess, Visitor};
use std::fmt;
use yew::prelude::*;

/// Deserialize a JSON string → AttrValue
pub fn de_attr<'de, D>(de: D) -> Result<AttrValue, D::Error>
where
    D: Deserializer<'de>,
{
    let s = String::deserialize(de)?;
    Ok(s.into())
}

/// Deserialize Option<AttrValue> from JSON string or null/missing
pub fn de_option_attr<'de, D>(de: D) -> Result<Option<AttrValue>, D::Error>
where
    D: Deserializer<'de>,
{
    let opt = Option::<String>::deserialize(de)?;
    Ok(opt.map(Into::into))
}

/// Deserialize yew::Classes from either a string ("foo bar") or a list `["foo","bar"]`.
pub fn de_classes<'de, D>(de: D) -> Result<Classes, D::Error>
where
    D: Deserializer<'de>,
{
    struct ClassesVisitor;
    impl<'de> Visitor<'de> for ClassesVisitor {
        type Value = Classes;

        fn expecting(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            f.write_str("a whitespace-separated string or a list of class names")
        }

        fn visit_str<E>(self, s: &str) -> Result<Self::Value, E>
        where
            E: Error,
        {
            let mut classes = Classes::new();
            // split on whitespace, clone each slice into a String so nothing borrows `s`
            for part in s.split_whitespace() {
                classes.push(part.to_string());
            }
            Ok(classes)
        }

        fn visit_seq<A>(self, mut seq: A) -> Result<Self::Value, A::Error>
        where
            A: SeqAccess<'de>,
        {
            let mut classes = Classes::new();
            while let Some(class_str) = seq.next_element::<String>()? {
                classes.push(class_str);
            }
            Ok(classes)
        }
    }

    de.deserialize_any(ClassesVisitor)
}
