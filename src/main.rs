use crate::properties::{kernel, os};
use sysinfo::{CpuRefreshKind, RefreshKind, SystemExt};

mod args;
mod color;
mod config;
mod properties;

fn main() {
  let sys = sysinfo::System::new_with_specifics(
    RefreshKind::new()
      .with_memory()
      .with_cpu(CpuRefreshKind::new()),
  );

  colored::control::set_override(true);

  println!("{}", os(&sys));
  println!("{}", kernel(&sys));
}
