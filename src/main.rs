mod ssh;

use std::fs;

use anyhow::{Context, Result};
use clap::Parser;
use ssh::{config::SSHConfig, host::HostConfig, host_builder::Builder};

/// SSH Config Generator
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None, disable_help_flag = true)]
struct Args {
    /// SSH user
    #[arg(short, long)]
    user: String,

    /// SSH hostname
    #[arg(short, long, required = true)]
    host: Vec<String>,

    /// Indentity file
    #[arg(short, long)]
    identity_file: Option<std::path::PathBuf>,

    /// Output file
    #[arg(short, long)]
    output: Option<std::path::PathBuf>,

    /// Print help
    #[arg(long, action = clap::ArgAction::HelpLong)]
    help: Option<bool>,
}

fn main() -> Result<()> {
    let args = Args::parse();
    let config = SSHConfig::new(
        args.host
            .iter()
            .enumerate()
            .map(|(index, s)| {
                let mut builder = HostConfig::builder();
                builder.set_name(format!("node{}", index + 1));
                builder.set_user(args.user.to_string());
                builder.set_hostname(s.to_string());
                builder.set_identity_file(args.identity_file.clone());
                builder.build()
            })
            .collect(),
    );

    if let Some(output_file) = args.output {
        fs::write(output_file.clone(), format!("{}", config))
            .with_context(|| format!("could not write file `{}`", output_file.display()))?;
    } else {
        println!("# generated via `scg` - SSH Config Generator:");
        println!("{}", config);
    }

    Ok(())
}
