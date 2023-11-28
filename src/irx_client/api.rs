#![allow(clippy::redundant_closure_call)]
#![allow(clippy::needless_lifetimes)]
#![allow(clippy::match_single_binding)]
#![allow(clippy::clone_on_copy)]

use serde::Serialize;

#[doc = "... "]
#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize)]
pub struct ApiKey(String);
impl std::ops::Deref for ApiKey {
  type Target = String;
  fn deref(&self) -> &String {
    &self.0
  }
}
impl From<ApiKey> for String {
  fn from(value: ApiKey) -> Self {
    value.0
  }
}
impl From<&ApiKey> for ApiKey {
  fn from(value: &ApiKey) -> Self {
    value.clone()
  }
}
impl std::str::FromStr for ApiKey {
  type Err = &'static str;
  fn from_str(value: &str) -> Result<Self, &'static str> {
    if value.len() > 40usize {
      return Err("longer than 40 characters");
    }
    if value.len() < 40usize {
      return Err("shorter than 40 characters");
    }
    if regress::Regex::new("[0-9a-zA-Z]{40}")
      .unwrap()
      .find(value)
      .is_none()
    {
      return Err("doesn't match pattern \"[0-9a-zA-Z]{40}\"");
    }
    Ok(Self(value.to_string()))
  }
}
impl std::convert::TryFrom<&str> for ApiKey {
  type Error = &'static str;
  fn try_from(value: &str) -> Result<Self, &'static str> {
    value.parse()
  }
}
impl std::convert::TryFrom<&String> for ApiKey {
  type Error = &'static str;
  fn try_from(value: &String) -> Result<Self, &'static str> {
    value.parse()
  }
}
impl std::convert::TryFrom<String> for ApiKey {
  type Error = &'static str;
  fn try_from(value: String) -> Result<Self, &'static str> {
    value.parse()
  }
}
impl<'de> serde::Deserialize<'de> for ApiKey {
  fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
  where
    D: serde::Deserializer<'de>,
  {
    String::deserialize(deserializer)?
      .parse()
      .map_err(|e: &'static str| {
        <D::Error as serde::de::Error>::custom(e.to_string())
      })
  }
}
