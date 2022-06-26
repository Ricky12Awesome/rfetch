use crate::color::Color;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::str::FromStr;

pub type PropertyName = String;
pub type GroupName = String;
pub type Separator = String;

#[derive(Debug, Clone, Eq, PartialEq, Serialize, Deserialize)]
pub struct Config {
  pub default: DefaultPropertyConfig,
  pub groups: HashMap<GroupName, GroupConfig>,
  pub overrides: HashMap<PropertyName, PropertyConfig>,
}

#[derive(Debug, Clone, Eq, PartialEq, Serialize, Deserialize)]
pub struct DefaultPropertyConfig {
  pub name_color: Color,
  pub separator: Separator,
  pub separator_color: Color,
  pub value_color: Color,
}

#[derive(Debug, Default, Eq, PartialEq, Clone, Serialize, Deserialize)]
pub struct PropertyConfig {
  pub name_color: Option<Color>,
  pub separator: Option<Separator>,
  pub separator_color: Option<Color>,
  pub value_color: Option<Color>,
}

#[derive(Debug, Default, Eq, PartialEq, Clone, Serialize, Deserialize)]
pub struct GroupConfig {
  pub name_color: Option<Color>,
  pub separator: Option<Separator>,
  pub separator_color: Option<Color>,
  pub value_color: Option<Color>,
  pub properties: Vec<PropertyName>,
}

impl Default for Config {
  fn default() -> Self {
    Self {
      default: DefaultPropertyConfig {
        name_color: Color::from_str("bright cyan").unwrap(),
        separator: Separator::from(": "),
        separator_color: Color::from_str("bright white").unwrap(),
        value_color: Color::from_str("bright white").unwrap(),
      },
      groups: HashMap::from([(
        GroupName::from("sys"),
        GroupConfig {
          name_color: Some(Color::from_str("69").unwrap()),
          value_color: Some(Color::from_str("#FFFFFF").unwrap()),
          properties: Vec::from([
            PropertyName::from("os"),
            PropertyName::from("kernel"),
            PropertyName::from("memory"),
            PropertyName::from("cpu"),
            PropertyName::from("gpu"),
          ]),
          ..Default::default()
        },
      )]),
      overrides: Default::default(),
    }
  }
}

#[cfg(test)]
#[test]
fn test() {
  let conf = Config::default();
  let toml = toml::to_string_pretty(&conf).unwrap();
  println!("{toml}");

  let re_conf = toml::from_str::<Config>(&toml).unwrap();
  println!("{re_conf:#?}");

  assert_eq!(conf, re_conf);
}
