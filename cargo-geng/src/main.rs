use std::fmt::Display;
use std::process::Command;
use std::str::FromStr;

const SERVE_PORT: u16 = 8000;

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

fn exec<C: std::borrow::BorrowMut<Command>>(mut cmd: C) -> Result<(), anyhow::Error> {
    if cmd.borrow_mut().status()?.success() {
        Ok(())
    } else {
        anyhow::bail!("Failure")
    }
}

fn serve<P>(dir: P)
where
    std::path::PathBuf: From<P>,
{
    use futures::future;
    use hyper::service::{make_service_fn, service_fn};
    use hyper::{Body, Request, Response};
    use hyper_staticfile::Static;
    use std::io::Error as IoError;

    async fn handle_request<B>(
        req: Request<B>,
        static_: Static,
    ) -> Result<Response<Body>, IoError> {
        static_.clone().serve(req).await
    }

    tokio::runtime::Runtime::new().unwrap().block_on(async {
        let static_ = Static::new(dir);

        let make_service = make_service_fn(|_| {
            let static_ = static_.clone();
            future::ok::<_, hyper::Error>(service_fn(move |req| {
                handle_request(req, static_.clone())
            }))
        });

        let addr = ([127, 0, 0, 1], SERVE_PORT).into();
        let server = hyper::Server::bind(&addr).serve(make_service);
        eprintln!("Server running on http://{}/", addr);
        server.await.expect("Server failed");
    });
}

fn main() -> Result<(), anyhow::Error> {
    let mut args: Vec<_> = std::env::args().collect();
    if args.len() >= 2 && args[1] == "geng" {
        args.remove(1);
    }
    let opt: Opt = structopt::StructOpt::from_iter(args);
    match opt {
        Opt::Run { target } => {
            let metadata = cargo_metadata::MetadataCommand::new().exec()?;
            let out_dir = metadata.target_directory.join("geng");
            if out_dir.exists() {
                std::fs::remove_dir_all(&out_dir)?;
            }
            let static_dir = std::path::Path::new(
                &metadata
                    .packages
                    .iter()
                    .find(|package| {
                        package.id == *metadata.resolve.as_ref().unwrap().root.as_ref().unwrap()
                    })
                    .unwrap()
                    .manifest_path,
            )
            .parent()
            .unwrap()
            .join("static");
            fs_extra::dir::copy(dbg!(static_dir), &out_dir, &{
                let mut options = fs_extra::dir::CopyOptions::new();
                options.copy_inside = true;
                options
            })?;
            match target {
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

                    let mut command = command()
                        .arg("--message-format=json")
                        .stdout(std::process::Stdio::piped())
                        .stderr(std::process::Stdio::null())
                        .spawn()?;
                    let reader = std::io::BufReader::new(command.stdout.take().unwrap());
                    let mut wasm_file = None;
                    for message in cargo_metadata::Message::parse_stream(reader) {
                        if let cargo_metadata::Message::CompilerArtifact(
                            cargo_metadata::Artifact {
                                executable: Some(path),
                                ..
                            },
                        ) = message.unwrap()
                        {
                            if wasm_file.is_some() {
                                anyhow::bail!("Found several wasm files");
                            }
                            wasm_file = Some(path);
                        }
                    }
                    command.wait()?;
                    let wasm_file = wasm_file.ok_or(anyhow::anyhow!("wasm not found"))?;

                    exec(
                        Command::new("wasm-bindgen")
                            .arg("--target=web")
                            .arg("--no-typescript")
                            .arg("--out-dir")
                            .arg(&out_dir)
                            .arg(wasm_file),
                    )?;

                    serve(&out_dir);
                }
            }
        }
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
