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
      makeFlakeSystemOutputs = system: { src, buildInputs ? [ ] }:
        let
          overlays = [ (import rust-overlay) ];
          pkgs = import nixpkgs { inherit system overlays; };
          rust-toolchain = pkgs.rust-bin.stable."1.69.0".minimal;
          crane-lib = crane.lib.${system};
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
        in
        {
          defaultPackage = package;
          defaultApp = utils.lib.mkApp {
            drv = package;
          };
          devShell = with pkgs; mkShell {
            nativeBuildInputs = nativeBuildDeps;
            buildInputs = buildDeps ++ [
              cargo
              rustPackages.clippy
              rustfmt
              rust-analyzer
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
      let pkgs = import nixpkgs { inherit system; };
      in {
        formatter = pkgs.nixpkgs-fmt;
      }
    );
}
  
