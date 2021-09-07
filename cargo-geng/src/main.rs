use clap::Clap;
use std::process::Command;

const SERVE_PORT: u16 = 8000;

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
        let server = hyper::server::Server::bind(&addr).serve(make_service);
        let addr = format!("http://{}/", addr);
        eprintln!("Server running on {}", addr);
        open::that(&addr).expect("Failed to open browser");
        server.await.expect("Server failed");
    });
}

#[derive(Clap, PartialEq, Eq)]
enum Sub {
    Build,
    Run,
    Check,
}

impl std::str::FromStr for Sub {
    type Err = anyhow::Error;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(match s {
            "build" => Self::Build,
            "run" => Self::Run,
            "check" => Self::Check,
            _ => anyhow::bail!("Failed to parse subcommand"),
        })
    }
}

impl ToString for Sub {
    fn to_string(&self) -> String {
        match self {
            Self::Build => "build",
            Self::Run => "run",
            Self::Check => "check",
        }
        .to_owned()
    }
}

#[derive(Clap)]
struct Opt {
    sub: Sub,
    #[clap(long, short = 'p')]
    package: Option<String>,
    #[clap(long)]
    target: Option<String>,
    #[clap(long)]
    release: bool,
    #[clap(long)]
    all_features: bool,
    #[clap(long)]
    example: Option<String>,
    #[clap(long, short = 'j')]
    jobs: Option<usize>,
    #[clap(long)]
    index_file: Option<String>,
    passthrough_args: Vec<String>,
}

fn to_arg(arg: &Option<String>, name: &str) -> impl Iterator<Item = String> {
    if let Some(arg) = arg {
        vec![name.to_owned(), arg.to_owned()]
    } else {
        vec![]
    }
    .into_iter()
}

impl Opt {
    fn args_for_metadata(&self) -> impl Iterator<Item = String> {
        std::iter::empty()
    }
    fn args_without_target(&self) -> impl Iterator<Item = String> {
        self.args_for_metadata()
            .chain(to_arg(&self.package, "--package"))
            .chain(if self.release {
                Some("--release".to_owned())
            } else {
                None
            })
            .chain(to_arg(&self.example, "--example"))
            .chain(if self.all_features {
                Some("--all-features".to_owned())
            } else {
                None
            })
            .chain(if let Some(jobs) = self.jobs {
                Some(format!("--jobs={}", jobs))
            } else {
                None
            })
    }
    fn all_args(&self) -> impl Iterator<Item = String> {
        self.args_without_target()
            .chain(to_arg(&self.target, "--target"))
    }
}

fn main() -> Result<(), anyhow::Error> {
    let mut args: Vec<_> = std::env::args().collect();
    if args.len() >= 2 && args[1] == "geng" {
        args.remove(1);
    }
    if args.is_empty() {
        todo!("Help");
    }
    let opt: Opt = Clap::parse_from(args);
    match opt.sub {
        Sub::Build | Sub::Run => {
            exec(Command::new("cargo").arg("build").args(opt.all_args()))?;
            let metadata = cargo_metadata::MetadataCommand::new()
                .other_options(
                    opt.args_for_metadata()
                        .map(|arg| arg.to_owned())
                        .collect::<Vec<_>>(),
                )
                .exec()?;
            let package = metadata
                .packages
                .iter()
                .find(|package| {
                    if let Some(name) = &opt.package {
                        package.name == *name
                    } else {
                        package.id
                            == *metadata
                                .resolve
                                .as_ref()
                                .unwrap()
                                .root
                                .as_ref()
                                .expect("No root package or --package")
                    }
                })
                .unwrap();
            let out_dir = metadata.target_directory.join("geng");
            if out_dir.exists() {
                std::fs::remove_dir_all(&out_dir)?;
            }
            let mut bin_root_dir = std::path::Path::new(&package.manifest_path)
                .parent()
                .unwrap()
                .to_owned();
            if let Some(example) = &opt.example {
                bin_root_dir = bin_root_dir.join("examples").join(example);
            }
            let static_dir = bin_root_dir.join("static");
            if static_dir.is_dir() {
                fs_extra::dir::copy(static_dir, &out_dir, &{
                    let mut options = fs_extra::dir::CopyOptions::new();
                    options.copy_inside = true;
                    options
                })?;
            }
            std::fs::create_dir_all(&out_dir)?;

            let mut command = Command::new("cargo")
                .arg("build")
                .args(opt.all_args())
                .arg("--message-format=json")
                .stdout(std::process::Stdio::piped())
                .stderr(std::process::Stdio::null())
                .spawn()?;
            let reader = std::io::BufReader::new(command.stdout.take().unwrap());
            let mut executable = None;
            for message in cargo_metadata::Message::parse_stream(reader) {
                if let cargo_metadata::Message::CompilerArtifact(cargo_metadata::Artifact {
                    executable: Some(path),
                    ..
                }) = message.unwrap()
                {
                    if executable.is_some() {
                        anyhow::bail!("Found several executable files");
                    }
                    executable = Some(path);
                }
            }
            command.wait()?;
            let executable = executable.ok_or(anyhow::anyhow!("executable not found"))?;

            if executable.extension() == Some("wasm".as_ref()) {
                exec(
                    Command::new("wasm-bindgen")
                        .arg("--target=web")
                        .arg("--no-typescript")
                        .arg("--out-dir")
                        .arg(&out_dir)
                        .arg(&executable),
                )?;
                std::fs::write(
                    out_dir.join(
                        opt.index_file
                            .as_ref()
                            .map(|s| s.as_str())
                            .unwrap_or("index.html"),
                    ),
                    include_str!("index.html")
                        .replace("<app-name>", executable.file_stem().unwrap()),
                )?;
                if opt.sub == Sub::Run {
                    serve(&out_dir);
                }
            } else {
                std::fs::copy(&executable, out_dir.join(executable.file_name().unwrap()))?;
                if opt.sub == Sub::Run {
                    exec(Command::new(&executable).args(opt.passthrough_args).env(
                        "CARGO_MANIFEST_DIR",
                        package.manifest_path.parent().unwrap(),
                    ))?;
                }
            }
        }
        Sub::Check => {
            exec(
                Command::new("cargo")
                    .arg("check")
                    .args(opt.args_without_target()),
            )?;
            exec(
                Command::new("cargo")
                    .arg("check")
                    .args(opt.args_without_target())
                    .arg("--target=wasm32-unknown-unknown"),
            )?;
        }
    }
    Ok(())
}
