use crate::color::Color;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::str::FromStr;

#[derive(Debug, Clone, Eq, PartialEq, Serialize, Deserialize)]
pub struct RawConfig {
  pub default: PropertyConfig,
  pub groups: HashMap<String, GroupConfig>,
  pub overrides: HashMap<String, OptionPropertyConfig>,
}

#[derive(Debug, Clone, Eq, PartialEq, Serialize, Deserialize)]
pub struct PropertyConfig {
  pub name_color: Color,
  pub separator: String,
  pub separator_color: Color,
  pub value_color: Color,
}

#[derive(Debug, Default, Eq, PartialEq, Clone, Serialize, Deserialize)]
pub struct OptionPropertyConfig {
  pub name_color: Option<Color>,
  pub separator: Option<String>,
  pub separator_color: Option<Color>,
  pub value_color: Option<Color>,
}

#[derive(Debug, Default, Eq, PartialEq, Clone, Serialize, Deserialize)]
pub struct GroupConfig {
  pub name_color: Option<Color>,
  pub separator: Option<String>,
  pub separator_color: Option<Color>,
  pub value_color: Option<Color>,
  pub properties: Vec<String>,
}

impl Default for RawConfig {
  fn default() -> Self {
    Self {
      default: PropertyConfig {
        name_color: Color::from_str("bright cyan").unwrap(),
        separator: String::from(": "),
        separator_color: Color::from_str("bright white").unwrap(),
        value_color: Color::from_str("bright white").unwrap(),
      },
      groups: HashMap::from([(
        String::from("sys"),
        GroupConfig {
          name_color: Some(Color::from_str("69").unwrap()),
          value_color: Some(Color::from_str("#FF00FF").unwrap()),
          properties: Vec::from([
            String::from("os"),
            String::from("kernel"),
            String::from("memory"),
            String::from("cpu"),
            String::from("gpu"),
          ]),
          ..Default::default()
        },
      )]),
      overrides: Default::default(),
    }
  }
}

#[derive(Debug, Clone)]
pub struct Config {
  default: PropertyConfig,
  properties: HashMap<String, OptionPropertyConfig>,
}

impl Config {
  pub fn parse(raw: RawConfig) -> Self {
    let mut config = Config {
      default: raw.default,
      properties: HashMap::new(),
    };

    for (_group, group_config) in &raw.groups {
      for property in &group_config.properties {
        config.insert(
          property,
          &OptionPropertyConfig {
            name_color: group_config.name_color,
            separator: group_config.separator.clone(),
            separator_color: group_config.separator_color,
            value_color: group_config.value_color,
          },
        );
      }
    }

    for (property, property_config) in &raw.overrides {
      config.insert(property, property_config);
    }

    config
  }

  fn insert(&mut self, property: &String, config: &OptionPropertyConfig) {
    match self.properties.get_mut(property) {
      None => {
        self.properties.insert(property.clone(), config.clone());
      }
      Some(current_config) => {
        if let Some(name_color) = config.name_color {
          current_config.name_color = Some(name_color);
        }

        if let Some(separator) = &config.separator {
          current_config.separator = Some(separator.clone());
        }

        if let Some(separator_color) = config.separator_color {
          current_config.separator_color = Some(separator_color);
        }

        if let Some(value_color) = config.value_color {
          current_config.value_color = Some(value_color);
        }
      }
    }
  }

  pub fn get(&self, property: impl ToString) -> PropertyConfig {
    match self.properties.get(&property.to_string()) {
      None => self.default.clone(),
      Some(config) => PropertyConfig {
        name_color: config.name_color.unwrap_or(self.default.name_color),
        separator: config.separator.as_ref().unwrap_or(&self.default.separator).clone(),
        separator_color: config.separator_color.unwrap_or(self.default.separator_color),
        value_color: config.value_color.unwrap_or(self.default.value_color),
      },
    }
  }
}

impl Default for Config {
  fn default() -> Self {
    Config::parse(RawConfig::default())
  }
}

#[cfg(test)]
#[test]
fn test() {
  let conf = RawConfig::default();
  let toml = toml::to_string_pretty(&conf).unwrap();
  let re_conf = toml::from_str::<RawConfig>(&toml).unwrap();

  assert_eq!(conf, re_conf);
}
