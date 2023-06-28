# https://scvalex.net/posts/63/
{
  inputs = {
    # This must be the stable nixpkgs if you're running the app on a
    # stable NixOS install.  Mixing EGL library versions doesn't work.
    nixpkgs.url = "github:NixOS/nixpkgs/nixpkgs-unstable";
    rust-overlay.url = "github:oxalica/rust-overlay";
    utils.url = "github:numtide/flake-utils";
    crane.url = "github:ipetkov/crane";
    android.url = "github:tadfisher/android-nixpkgs";
  };
  outputs = { self, nixpkgs, rust-overlay, crane, android, utils }:
    {
      makeFlakeSystemOutputs = system: { src, buildInputs ? [ ], rust ? { } }:
        let
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
          crane-lib = (crane.lib.${system}).overrideToolchain rust-toolchain;
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
            buildInputs ++
            waylandDeps ++
            xorgDeps ++
            [
              alsa-lib
              udev
              libGL
              xorg.libxcb
            ];
          nativeBuildDeps = with pkgs; [ pkg-config ];
          buildDeps = with pkgs; libDeps ++ [ xorg.libxcb ];
          libPath = pkgs.lib.makeLibraryPath libDeps;
          package =
            let
              commonArgs = {
                inherit src;
                nativeBuildInputs = nativeBuildDeps ++ [ pkgs.makeWrapper ];
                buildInputs = buildDeps;
              };
              package = crane-lib.buildPackage (commonArgs // {
                cargoArtifacts = crane-lib.buildDepsOnly commonArgs;
              });
              finalPackage = package.overrideAttrs (finalAttrs: prevAttrs: {
                postPhases = [ "copyAssetsPhase" "wrapProgramPhase" ];
                copyAssetsPhase = ''
                  cp -r ${src + "/assets"} $out/bin/assets
                '';
                wrapProgramPhase = ''
                  wrapProgram "$out/bin/${finalAttrs.pname}" \
                    --set WINIT_UNIX_BACKEND x11 \
                    --prefix LD_LIBRARY_PATH : "${libPath}"
                '';
              });
            in
            finalPackage;
          lib = {
            crane = crane-lib;
            cargo-geng = crane-lib.buildPackage {
              pname = "cargo-geng";
              # cargoVendorDir = null;
              src = ./.;
              cargoExtraArgs = "--package cargo-geng";
            };
            cargo-apk = crane-lib.buildPackage {
              pname = "cargo-apk";
              version = "0.9.7";
              src = builtins.fetchGit {
                url = "https://github.com/geng-engine/cargo-apk";
                rev = "03814af67622d7f5f8f048081b407c3909b6b289";
              };
              cargoExtraArgs = "--package cargo-apk";
              cargoVendorDir = crane-lib.vendorCargoDeps {
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
        {
          inherit lib;
          defaultPackage = package;
          defaultApp = utils.lib.mkApp {
            drv = package;
          };
          devShell = with pkgs; mkShell {
            nativeBuildInputs = nativeBuildDeps;
            buildInputs = buildDeps ++ [
              rust-toolchain
              rust-analyzer
              lib.cargo-geng
              # wineWowPackages.waylandFull
              # pkgsCross.mingwW64.windows.pthreads
              lib.cargo-apk
              lib.androidsdk
              jre
            ];
            shellHook =
              let
                libPath = pkgs.lib.makeLibraryPath (libDeps ++ [ pkgsCross.mingwW64.windows.pthreads ]);
                androidSdkRoot = "${lib.androidsdk}/libexec/android-sdk";
                androidNdkRoot = "${androidSdkRoot}/ndk-bundle";
              in
              ''
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
  
