use std::fmt::Display;
use std::process::Command;
use std::str::FromStr;

enum Target {
    Default,
    Web,
}

impl Default for Target {
    fn default() -> Self {
        Self::Default
    }
}

impl FromStr for Target {
    type Err = std::io::Error;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(match s {
            "default" => Self::Default,
            "web" => Self::Web,
            _ => return Err(std::io::Error::new(std::io::ErrorKind::Other, "Unexpected")),
        })
    }
}

impl Display for Target {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Default => write!(f, "default"),
            Self::Web => write!(f, "web"),
        }
    }
}

#[derive(structopt::StructOpt)]
enum Opt {
    Run {
        #[structopt(long, default_value)]
        target: Target,
    },
    Check,
}

fn exec(cmd: &mut Command) -> Result<(), std::io::Error> {
    if cmd.status()?.success() {
        Ok(())
    } else {
        Err(std::io::Error::new(std::io::ErrorKind::Other, "Failure"))
    }
}

fn main() -> Result<(), std::io::Error> {
    let mut args: Vec<_> = std::env::args().collect();
    if args.len() >= 2 && args[1] == "geng" {
        args.remove(1);
    }
    let opt: Opt = structopt::StructOpt::from_iter(args);
    match opt {
        Opt::Run { target } => match target {
            Target::Default => {
                exec(Command::new("cargo").arg("run"))?;
            }
            Target::Web => {
                exec(
                    Command::new("cargo")
                        .arg("web")
                        .arg("start")
                        .arg("--release")
                        .arg("--open"),
                )?;
            }
        },
        Opt::Check => {
            exec(Command::new("cargo").arg("check"))?;
            exec(Command::new("cargo").arg("web").arg("check"))?;
        }
    }
    Ok(())
}
