use serde::de;
use serde::Deserializer;
use std::str::FromStr;
use std::fmt;
use std::marker::PhantomData;
use serde::de::Visitor;

/// Deserialize T  from String, if String is "", return the T::default()
pub fn deserde_from_str<'de, D, T>(deserializer: D) -> std::result::Result<T, D::Error>
    where
        D: Deserializer<'de>,
        T: FromStr + Default,
        T::Err: fmt::Display,
{
    struct Helper<S>(PhantomData<S>);

    impl<'de, S> Visitor<'de> for Helper<S>
        where
            S: FromStr + Default,
            <S as FromStr>::Err: fmt::Display,
    {
        type Value = S;

        fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
            write!(formatter, "valid json object")
        }

        fn visit_str<E>(self, value: &str) -> std::result::Result<Self::Value, E>
            where
                E: de::Error,
        {
            if value.is_empty() {
                return Ok(S::default());
            }
            let v = value.parse::<S>().map_err(de::Error::custom)?;
            Ok(v)
        }
    }

    deserializer.deserialize_str(Helper(PhantomData))
}