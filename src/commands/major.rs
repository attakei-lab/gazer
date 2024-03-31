use crate::config::Config;
use crate::{app, versioning::up_major};
use anyhow::Result;
use clap::Args;

#[derive(Args)]
pub(crate) struct Arguments {}

pub(crate) fn execute(_args: &Arguments, config: &Config) -> Result<()> {
    let new_version = up_major(&config.current_version);

    app::update(&config, &new_version)
}
