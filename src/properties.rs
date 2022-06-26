use std::fmt::{Display, Formatter};

use crate::color::Color;
use crate::config::PropertyConfig;
use crate::Config;
use sysinfo::{CpuExt, System, SystemExt};

pub struct Property {
  pub name: String,
  pub value: String,
  pub config: PropertyConfig,
}

impl Display for Property {
  fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
    f.write_str(&self.config.name_color.fg())?;
    f.write_str(&self.name)?;
    f.write_str(&self.config.separator_color.fg())?;
    f.write_str(&self.config.separator)?;
    f.write_str(&self.config.value_color.fg())?;
    f.write_str(&self.value)?;
    f.write_str(&Color::Reset.fg())?;

    Ok(())
  }
}

pub fn os(sys: &System, config: &Config) -> Property {
  Property {
    name: "OS".to_string(),
    value: sys.long_os_version().unwrap(),
    config: config.get("os"),
  }
}

pub fn kernel(sys: &System, config: &Config) -> Property {
  Property {
    name: "Kernel".to_string(),
    value: sys.kernel_version().unwrap(),
    config: config.get("kernel"),
  }
}

pub fn cpu(sys: &System, config: &Config) -> Property {
  let cpu_info = sys.global_cpu_info();

  Property {
    name: "CPU".to_string(),
    value: format!(
      "{} @ {:.3}Ghz",
      cpu_info.brand(),
      cpu_info.frequency() as f64 / 1000f64
    ),
    config: config.get("cpu"),
  }
}

pub fn memory(sys: &System, config: &Config) -> Property {
  let used = sys.used_memory() as f64 / 1024f64;
  let total = sys.total_memory() as f64 / 1024f64;

  Property {
    name: "Memory".to_string(),
    value: format!(
      "{used:.0} MiB / {total:.0} MiB"
    ),
    config: config.get("memory"),
  }
}
