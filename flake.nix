{
  inputs = {
    cargo-geng.url = "github:geng-engine/cargo-geng";
  };

  outputs = { self, cargo-geng }:
    { inherit (cargo-geng) eachDefaultSystem makeFlakeSystemOutputs; } //
    cargo-geng.eachDefaultSystem (system:
      {
        inherit
          (cargo-geng.makeFlakeSystemOutputs system { src = ./.; })
          devShell formatter lib;
      }
    );
}
  
