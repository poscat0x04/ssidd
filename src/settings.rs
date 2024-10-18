use ipnet::Ipv4Net;
use regex::bytes::Regex;
use regex::escape;
use serde::de::{Error, MapAccess, SeqAccess, Visitor};
use serde::{Deserialize, Deserializer};
use std::borrow::Cow;
use std::fmt::Formatter;

#[derive(Debug, Eq, PartialEq, Copy, Clone, Deserialize)]
pub enum Backend {
    NetworkManager,
    Networkd,
    Connman,
}

#[derive(Debug, Clone, Deserialize)]
pub struct Settings {
    pub backend: Backend,
    #[serde(default)]
    pub invert_match: bool,
    #[serde(deserialize_with = "deserialize_selector")]
    pub ssids: Regex,
    #[serde(default)]
    pub ethernet: EtherSettings,
}

#[derive(Debug, Clone, Default, Deserialize)]
pub struct EtherSettings {
    pub invert_match: bool,
    #[serde(default)]
    pub subnets: Vec<Ipv4Net>,
}


fn deserialize_selector<'de, D>(deserializer: D) -> Result<Regex, D::Error>
where
    D: Deserializer<'de>,
{
    struct SsidMatchVisitor;

    impl<'de> Visitor<'de> for SsidMatchVisitor {
        type Value = Regex;

        fn expecting(&self, formatter: &mut Formatter) -> std::fmt::Result {
            let e =
                "a string, a list of string or a map of the form {regex: regex pattern string}";
            write!(formatter, "{}", e)
        }

        fn visit_str<E>(self, pat: &str) -> Result<Self::Value, E>
        where
            E: Error,
        {
            Ok(Regex::new(&escape(pat)).unwrap_or_else(|e| {
                panic!("impossible: invalid regex pattern: {}", e)
            }))
        }

        fn visit_seq<A>(self, mut seq: A) -> Result<Self::Value, A::Error>
        where
            A: SeqAccess<'de>,
        {
            let mut pat = String::with_capacity(100);

            pat.push('(');

            if let Some(ssid) = seq.next_element::<Cow<str>>()? {
                pat += &escape(&ssid);
            }

            while let Some(ssid) = seq.next_element::<Cow<str>>()? {
                pat.push('|');
                pat += &escape(&ssid);
            }

            pat.push(')');

            Ok(Regex::new(&pat).unwrap_or_else(|e|
                panic!("impossible: invalid regex pattern: {}", e)
            ))
        }

        fn visit_map<A>(self, mut map: A) -> Result<Self::Value, A::Error>
        where
            A: MapAccess<'de>,
        {
            let mut regex = None;
            while let Some(key) = map.next_key::<Cow<str>>()? {
                if key == "regex" {
                    if regex.is_some() {
                        Err(A::Error::duplicate_field("regex"))?
                    }
                    let value = map.next_value::<Cow<str>>()?;
                    regex = match Regex::new(&value) {
                        Ok(r) => { Some(r) }
                        Err(e) => { Err(A::Error::custom(e))? }
                    };
                } else {
                    Err(A::Error::unknown_field(&key, &["regex"]))?
                }
            }
            if let Some(r) = regex {
                Ok(r)
            } else {
                Err(A::Error::missing_field("regex"))
            }
        }
    }

    deserializer.deserialize_any(SsidMatchVisitor)
}
