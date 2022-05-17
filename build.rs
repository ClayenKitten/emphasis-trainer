use anyhow::Result;
use vergen::{Config, vergen, TimestampKind};

fn main() -> Result<()> {
  let mut config = Config::default();
  *config.build_mut().kind_mut() = TimestampKind::DateAndTime;
  vergen(config)
}