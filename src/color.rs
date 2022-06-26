#![allow(unused)]

use serde::{Deserialize, Deserializer, Serialize, Serializer};
use std::borrow::Cow;
use std::num::ParseIntError;
use std::str::FromStr;

pub enum ColorMode {
  Auto,
  Always,
  None,
}

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub enum Color {
  Black,
  Red,
  Green,
  Yellow,
  Blue,
  Magenta,
  Cyan,
  White,
  BrightBlack,
  BrightRed,
  BrightGreen,
  BrightYellow,
  BrightBlue,
  BrightMagenta,
  BrightCyan,
  BrightWhite,
  Reset,
  Color256(u8),
  TrueColor(u8, u8, u8),
}

impl Color {
  fn as_fg_str(&self) -> Cow<'static, str> {
    match *self {
      Color::Black => "30".into(),
      Color::Red => "31".into(),
      Color::Green => "32".into(),
      Color::Yellow => "33".into(),
      Color::Blue => "34".into(),
      Color::Magenta => "35".into(),
      Color::Cyan => "36".into(),
      Color::White => "37".into(),
      Color::BrightBlack => "90".into(),
      Color::BrightRed => "91".into(),
      Color::BrightGreen => "92".into(),
      Color::BrightYellow => "93".into(),
      Color::BrightBlue => "94".into(),
      Color::BrightMagenta => "95".into(),
      Color::BrightCyan => "96".into(),
      Color::BrightWhite => "97".into(),
      Color::Reset => "39;49".into(),
      Color::Color256(n) => format!("38;5;{}m", n).into(),
      Color::TrueColor(r, g, b) => format!("38;2;{};{};{}", r, g, b).into(),
    }
  }

  fn as_bg_str(&self) -> Cow<'static, str> {
    match *self {
      Color::Black => "40".into(),
      Color::Red => "41".into(),
      Color::Green => "42".into(),
      Color::Yellow => "43".into(),
      Color::Blue => "44".into(),
      Color::Magenta => "45".into(),
      Color::Cyan => "46".into(),
      Color::White => "47".into(),
      Color::BrightBlack => "100".into(),
      Color::BrightRed => "101".into(),
      Color::BrightGreen => "102".into(),
      Color::BrightYellow => "103".into(),
      Color::BrightBlue => "104".into(),
      Color::BrightMagenta => "105".into(),
      Color::BrightCyan => "106".into(),
      Color::BrightWhite => "107".into(),
      Color::Reset => "39;49".into(),
      Color::Color256(n) => format!("48;5;{}", n).into(),
      Color::TrueColor(r, g, b) => format!("48;2;{};{};{}", r, g, b).into(),
    }
  }

  pub fn fg(&self) -> String {
    format!("\x1B[{}m", self.as_fg_str())
  }

  pub fn bg(&self) -> String {
    format!("\x1B[{}m", self.as_bg_str())
  }
}

impl FromStr for Color {
  type Err = String;

  fn from_str(s: &str) -> Result<Self, Self::Err> {
    let src = s.to_lowercase();
    let mut result = Err(format!("'{}' is not a valid color", s));

    result = match src.as_ref() {
      "black" => Ok(Color::Black),
      "red" => Ok(Color::Red),
      "green" => Ok(Color::Green),
      "yellow" => Ok(Color::Yellow),
      "blue" => Ok(Color::Blue),
      "magenta" | "purple" => Ok(Color::Magenta),
      "cyan" => Ok(Color::Cyan),
      "gray" | "grey" | "white" => Ok(Color::White),
      "bright_black" | "bright black" => Ok(Color::BrightBlack),
      "bright_red" | "bright red" => Ok(Color::BrightRed),
      "bright_green" | "bright green" => Ok(Color::BrightGreen),
      "bright_yellow" | "bright yellow" => Ok(Color::BrightYellow),
      "bright_blue" | "bright blue" => Ok(Color::BrightBlue),
      "bright_magenta" | "bright magenta" => Ok(Color::BrightMagenta),
      "bright_cyan" | "bright cyan" => Ok(Color::BrightCyan),
      "bright_white" | "bright white" => Ok(Color::BrightWhite),
      "reset" => Ok(Color::Reset),
      _ => result,
    };

    if result.is_ok() {
      return result;
    }

    result = match u8::from_str(&src) {
      Ok(n) => Ok(Color::Color256(n)),
      Err(err) => Err(format!("{err:?}")),
    };

    if result.is_ok() {
      return result;
    }

    result = match u32::from_str_radix(src.trim_start_matches('#'), 16) {
      Ok(n) => {
        let [r, g, b, _] = n.to_le_bytes();

        Ok(Color::TrueColor(r, g, b))
      }
      Err(err) => Err(format!("{err:?}")),
    };

    result
  }
}

impl ToString for Color {
  fn to_string(&self) -> String {
    match *self {
      Color::Black => "black".to_string(),
      Color::Red => "red".to_string(),
      Color::Green => "green".to_string(),
      Color::Yellow => "yellow".to_string(),
      Color::Blue => "blue".to_string(),
      Color::Magenta => "magenta".to_string(),
      Color::Cyan => "cyan".to_string(),
      Color::White => "white".to_string(),
      Color::BrightBlack => "bright_black".to_string(),
      Color::BrightRed => "bright_red".to_string(),
      Color::BrightGreen => "bright_green".to_string(),
      Color::BrightYellow => "bright_yellow".to_string(),
      Color::BrightBlue => "bright_blue".to_string(),
      Color::BrightMagenta => "bright_magenta".to_string(),
      Color::BrightCyan => "bright_cyan".to_string(),
      Color::BrightWhite => "bright_white".to_string(),
      Color::Reset => "reset".to_string(),
      Color::Color256(n) => n.to_string(),
      Color::TrueColor(r, g, b) => format!("#{:x}", u32::from_le_bytes([r, g, b, 0])),
    }
  }
}

impl<'de> Deserialize<'de> for Color {
  fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
  where
    D: Deserializer<'de>,
  {
    let str = <&str>::deserialize(deserializer)?;

    Self::from_str(str).map_err(serde::de::Error::custom)
  }
}

impl Serialize for Color {
  fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
  where
    S: Serializer,
  {
    serializer.serialize_str(&self.to_string())
  }
}

pub trait Colorize: Sized {
  fn with_fg(self, color: Color) -> String;
  fn with_bg(self, color: Color) -> String;
}

impl Colorize for &str {
  fn with_fg(self, color: Color) -> String {
    format!("{}{}{}", color.fg(), self, Color::Reset.fg())
  }

  fn with_bg(self, color: Color) -> String {
    format!("{}{}{}", color.bg(), self, Color::Reset.bg())
  }
}

impl Colorize for String {
  fn with_fg(self, color: Color) -> String {
    format!("{}{}", color.fg(), self)
  }

  fn with_bg(self, color: Color) -> String {
    format!("{}{}", color.bg(), self)
  }
}
