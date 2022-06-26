use std::fmt::{Display, Formatter};

use colored::{Color, ColoredString, Colorize};
use sysinfo::{System, SystemExt};

pub struct Property {
  pub name: String,
  pub value: String,
  pub name_color: Color,
  pub value_color: Color,
}

impl Display for Property {
  fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
    let name = ColoredString::from(&*self.name).color(self.name_color);
    let value = ColoredString::from(&*self.value).color(self.value_color);
    let sep = ColoredString::from(": ");

    Display::fmt(&name, f)?;
    Display::fmt(&sep, f)?;
    Display::fmt(&value, f)
  }
}

pub fn os(sys: &System) -> Property {
  Property {
    name: "OS".to_string(),
    value: sys.long_os_version().unwrap(),
    name_color: Color::BrightCyan,
    value_color: Color::BrightWhite,
  }
}

pub fn kernel(sys: &System) -> Property {
  Property {
    name: "Kernel".to_string(),
    value: sys.kernel_version().unwrap(),
    name_color: Color::BrightCyan,
    value_color: Color::BrightWhite,
  }
}
