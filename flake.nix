{
  inputs = {
    nixpkgs.url = "github:nixos/nixpkgs/nixos-unstable";
    systems.url = "github:nix-systems/default";
    geng.url = "github:geng-engine/cargo-geng";
    geng.inputs.nixpkgs.follows = "nixpkgs";
  };
  outputs = { geng, nixpkgs, systems, ... }:
    let
      pkgsFor = system: import nixpkgs {
        inherit system;
      };
      forEachSystem = f: nixpkgs.lib.genAttrs (import systems) (system:
        let pkgs = pkgsFor system;
        in f system pkgs);
    in
    {
      devShells = forEachSystem (system: pkgs:
        {
          default = geng.lib.mkShell {
            inherit system;
            target.linux.enable = true;
            target.android.enable = true;
            target.windows.enable = true;
            target.web.enable = true;
          };
        });
      formatter = forEachSystem (system: pkgs: pkgs.nixpkgs-fmt);
    };
}
