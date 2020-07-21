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
    type Err = anyhow::Error;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(match s {
            "default" => Self::Default,
            "web" => Self::Web,
            _ => anyhow::bail!("Unexpected"),
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

fn exec<C: AsMut<Command>>(cmd: C) -> Result<(), anyhow::Error> {
    if cmd.as_mut().status()?.success() {
        Ok(())
    } else {
        anyhow::bail!("Failure")
    }
}

fn main() -> Result<(), anyhow::Error> {
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
                let command = || {
                    let mut command = Command::new("cargo");
                    command
                        .arg("build")
                        .arg("--target=wasm32-unknown-unknown")
                        .arg("--release");
                    command
                };
                exec(command())?;

                use cargo_metadata::Message;

                let mut command = command()
                    .arg("--message-format=json")
                    .stdout(std::process::Stdio::piped())
                    .spawn()?;

                let reader = std::io::BufReader::new(command.stdout.take().unwrap());
                for message in cargo_metadata::Message::parse_stream(reader) {
                    match message.unwrap() {
                        Message::CompilerMessage(msg) => {
                            println!("{:?}", msg);
                        }
                        Message::CompilerArtifact(artifact) => {
                            println!("{:?}", artifact);
                        }
                        Message::BuildScriptExecuted(script) => {
                            println!("{:?}", script);
                        }
                        Message::BuildFinished(finished) => {
                            println!("{:?}", finished);
                        }
                        _ => (), // Unknown message
                    }
                }

                let output = command.wait().expect("Couldn't get cargo's exit status");
            }
        },
        Opt::Check => {
            exec(Command::new("cargo").arg("check"))?;
            exec(
                Command::new("cargo")
                    .arg("check")
                    .arg("--target=wasm32-unknown-unknown"),
            )?;
        }
    }
    Ok(())
}
