# https://scvalex.net/posts/63/
{
  inputs = {
    # This must be the stable nixpkgs if you're running the app on a
    # stable NixOS install.  Mixing EGL library versions doesn't work.
    nixpkgs.url = "github:NixOS/nixpkgs/nixpkgs-unstable";
    rust-overlay.url = "github:oxalica/rust-overlay";
    utils.url = "github:numtide/flake-utils";
    nix-filter.url = "github:numtide/nix-filter";
    crane-flake.url = "github:ipetkov/crane";
    android.url = "github:tadfisher/android-nixpkgs";
  };

  outputs = { self, nixpkgs, rust-overlay, crane-flake, android, utils, nix-filter }:
    {
      makeFlakeSystemOutputs = system: { src, extraBuildInputs ? [ ], rust ? { } }:
        let
          filter = nix-filter.lib;
          overlays = [ (import rust-overlay) ];
          pkgs = import nixpkgs {
            inherit system overlays;
            config = {
              allowUnfree = true;
              android_sdk.accept_license = true;
            };
          };
          rust-version = ({ version = "latest"; } // rust).version;
          rust-toolchain = pkgs.rust-bin.stable.${rust-version}.default.override
            {
              extensions = [ "rust-src" ];
              targets = [
                "wasm32-unknown-unknown"
                "x86_64-pc-windows-gnu"
                "aarch64-linux-android"
              ];
            } // rust;
          crane = (crane-flake.lib.${system}).overrideToolchain rust-toolchain;
          waylandDeps = with pkgs; [
            libxkbcommon
            wayland
          ];
          xorgDeps = with pkgs; [
            xorg.libX11
            xorg.libXcursor
            xorg.libXi
            xorg.libXrandr
          ];
          libDeps = with pkgs;
            extraBuildInputs ++
            waylandDeps ++
            xorgDeps ++
            [
              openssl
              alsa-lib
              udev
              libGL
              xorg.libxcb
            ];
          nativeBuildInputs = with pkgs; [ pkg-config ];
          buildInputs = with pkgs; libDeps ++ [ xorg.libxcb ];
          libPath = pkgs.lib.makeLibraryPath libDeps;
          lib = rec {
            inherit crane;
            inherit filter;
            cargo-geng = crane.buildPackage {
              pname = "cargo-geng";
              src = filter {
                root = ./.;
                include = [
                  "crates/cargo-geng"
                  ./Cargo.lock
                  ./Cargo.toml
                ];
              };
              cargoExtraArgs = "--package cargo-geng";
            };
            buildGengPackage =
              { target ? null
              , ...
              }@origArgs:
              let
                cleanedArgs = builtins.removeAttrs origArgs [
                  "installPhase"
                  "installPhaseCommand"
                  "target"
                ];

                crateName = crane.crateNameFromCargoToml cleanedArgs;

                # Avoid recomputing values when passing args down
                args = cleanedArgs // {
                  pname = cleanedArgs.pname or crateName.pname;
                  version = cleanedArgs.version or crateName.version;
                  cargoVendorDir = cleanedArgs.cargoVendorDir or (crane.vendorCargoDeps cleanedArgs);
                };
              in
              crane.mkCargoDerivation (args // {
                # pnameSuffix = "-trunk";
                cargoArtifacts = args.cargoArtifacts or (crane.buildDepsOnly (args // {
                  CARGO_BUILD_TARGET = args.CARGO_BUILD_TARGET or (if target == "web" then "wasm32-unknown-unknown" else target);
                  installCargoArtifactsMode = args.installCargoArtifactsMode or "use-zstd";
                  doCheck = args.doCheck or false;
                  inherit nativeBuildInputs;
                  inherit buildInputs;
                }));

                buildPhaseCargoCommand = args.buildPhaseCommand or (
                  let
                    args = if builtins.isNull target then "" else "--" + target;
                  in
                  ''
                    local args="${args}"
                    if [[ "$CARGO_PROFILE" == "release" ]]; then
                      args="$args --release"
                    fi

                    cargo geng build $args
                  ''
                );

                installPhaseCommand = args.installPhaseCommand or ''
                  cp -r target/geng $out
                '';

                # Installing artifacts on a distributable dir does not make much sense
                doInstallCargoArtifacts = args.doInstallCargoArtifacts or false;

                nativeBuildInputs = (args.nativeBuildInputs or [ ]) ++ nativeBuildInputs ++ [
                  cargo-geng
                ];
                buildInputs = buildInputs ++ (args.buildInputs or [ ]);
              });
            cargo-apk = crane.buildPackage {
              pname = "cargo-apk";
              version = "0.9.7";
              src = builtins.fetchGit {
                url = "https://github.com/geng-engine/cargo-apk";
                allRefs = true;
                rev = "fc7f7fd19cdde19119136e7e726c85d101ca37db";
              };
              cargoExtraArgs = "--package cargo-apk";
              cargoVendorDir = crane.vendorCargoDeps {
                cargoLock = ./cargo-apk.Cargo.lock;
              };
            };
            androidsdk = (pkgs.androidenv.composeAndroidPackages {
              cmdLineToolsVersion = "8.0";
              toolsVersion = "26.1.1";
              platformToolsVersion = "34.0.1";
              buildToolsVersions = [ "30.0.3" ];
              includeEmulator = false;
              emulatorVersion = "33.1.6";
              platformVersions = [ "33" ];
              includeSources = false;
              includeSystemImages = false;
              systemImageTypes = [ "google_apis_playstore" ];
              abiVersions = [
                "armeabi-v7a"
                "arm64-v8a"
              ];
              cmakeVersions = [ "3.10.2" ];
              includeNDK = true;
              ndkVersions = [ "25.2.9519653" ];
              useGoogleAPIs = false;
              useGoogleTVAddOns = false;
              includeExtras = [
                # "extras;google;gcm"
              ];
            }).androidsdk;
          };
        in
        rec {
          inherit lib;
          # Executed by `nix build .`
          packages.default = lib.buildGengPackage { inherit src; };
          # Executed by `nix build .#web"
          packages.web = lib.buildGengPackage { inherit src; target = "web"; };
          # Executed by `nix run . -- <args?>`
          apps.default =
            {
              type = "app";
              program = "${packages.default}/linksider";
            };
          devShell = with pkgs; mkShell {
            inherit nativeBuildInputs;
            buildInputs = buildInputs ++ [
              rust-toolchain
              rust-analyzer
              lib.cargo-geng
              # wineWowPackages.waylandFull
              # pkgsCross.mingwW64.windows.pthreads
              lib.cargo-apk
              lib.androidsdk
              jre
              yad # for tinyfiledialogs
            ];
            shellHook =
              let
                libPath = pkgs.lib.makeLibraryPath (libDeps ++ [ pkgsCross.mingwW64.windows.pthreads ]);
                androidSdkRoot = "${lib.androidsdk}/libexec/android-sdk";
                androidNdkRoot = "${androidSdkRoot}/ndk-bundle";
              in
              ''
                export CARGO_TARGET_X86_64_PC_WINDOWS_GNU_RUSTFLAGS="-C link-args=''$(echo $NIX_LDFLAGS | tr ' ' '\n' | grep -- '^-L' | tr '\n' ' ')"
                export LD_LIBRARY_PATH="$LD_LIBRARY_PATH:${libPath}"
                export WINIT_UNIX_BACKEND=x11 # TODO fix
                export CARGO_TARGET_X86_64_PC_WINDOWS_GNU_LINKER="${pkgsCross.mingwW64.stdenv.cc}/bin/x86_64-w64-mingw32-gcc"
                export CARGO_TARGET_X86_64_PC_WINDOWS_GNU_RUNNER="wine64"
                export ANDROID_SDK_ROOT="${androidSdkRoot}";
                export ANDROID_NDK_ROOT="${androidNdkRoot}"; 
              '';
          };
          formatter = pkgs.nixpkgs-fmt;
        };
      makeFlakeOutputs = f: utils.lib.eachDefaultSystem (system: self.makeFlakeSystemOutputs system (f system));
    } // utils.lib.eachDefaultSystem (system:
      let
        flakeOutputs = (self.makeFlakeSystemOutputs system { src = ./.; });
      in
      {
        inherit (flakeOutputs) devShell formatter lib;
      }
    );
}
  
