# https://scvalex.net/posts/63/
{
  inputs = {
    # This must be the stable nixpkgs if you're running the app on a
    # stable NixOS install.  Mixing EGL library versions doesn't work.
    nixpkgs.url = "github:NixOS/nixpkgs/nixpkgs-unstable";
    rust-overlay.url = "github:oxalica/rust-overlay";
    utils.url = "github:numtide/flake-utils";
    crane.url = "github:ipetkov/crane";
  };
  outputs = { self, nixpkgs, rust-overlay, crane, utils }:
    {
      makeFlakeSystemOutputs = system: { src, buildInputs ? [ ], rust ? { } }:
        let
          overlays = [ (import rust-overlay) ];
          pkgs = import nixpkgs { inherit system overlays; };
          rust-version = ({ version = "latest"; } // rust).version;
          rust-toolchain = pkgs.rust-bin.stable.${rust-version}.default.override rust;
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
          libDeps = with pkgs; buildInputs
          ++ waylandDeps ++ xorgDeps ++ [
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
          cargo-geng = crane-lib.buildPackage {
            src = ./.;
            cargoExtraArgs = "--package cargo-geng";
          };
        in
        {
          defaultPackage = package;
          defaultApp = utils.lib.mkApp {
            drv = package;
          };
          devShell = with pkgs; mkShell {
            nativeBuildInputs = nativeBuildDeps;
            buildInputs = buildDeps ++ [
              rust-toolchain
              rust-analyzer
              # cargo-geng
            ];
            shellHook = ''
              export LD_LIBRARY_PATH="$LD_LIBRARY_PATH:${libPath}"
              export WINIT_UNIX_BACKEND=x11 # TODO fix
            '';
          };
          formatter = pkgs.nixpkgs-fmt;
        };
      makeFlakeOutputs = f: utils.lib.eachDefaultSystem (system: self.makeFlakeSystemOutputs system (f system));
    } // utils.lib.eachDefaultSystem (system:
      let
        pkgs = import nixpkgs { inherit system; };
        flakeOutputs = (self.makeFlakeSystemOutputs system { src = ./.; });
      in
      {
        devShell = flakeOutputs.devShell;
        formatter = pkgs.nixpkgs-fmt;
      }
    );
}
  