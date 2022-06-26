use crate::config::Config;
use crate::properties::{cpu, kernel, memory, os};
use sysinfo::{CpuRefreshKind, RefreshKind, SystemExt};

mod args;
mod color;
mod config;
mod properties;

fn main() {
  let sys = sysinfo::System::new_with_specifics(
    RefreshKind::new()
      .with_memory()
      .with_cpu(CpuRefreshKind::new().with_frequency())
  );

  let config = Config::default();

  println!("{}", os(&sys, &config));
  println!("{}", kernel(&sys, &config));
  println!("{}", cpu(&sys, &config));
  println!("{}", memory(&sys, &config));
}
