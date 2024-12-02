{
  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixpkgs-unstable";
    utils.url = "github:numtide/flake-utils";
  };

  outputs =
    { self
    , nixpkgs
    , utils
    ,
    }:
    utils.lib.eachDefaultSystem (system:
    let
      pkgs = import nixpkgs { inherit system; };
      haskell = pkgs.haskellPackages.ghcWithPackages (pkgs: with pkgs; [
        
      ]);
    in
    {
      devShells."2023" = with pkgs;
        mkShell {
          buildInputs = [
            cargo
            rustc
            rustfmt
            rust-analyzer
          ];
          RUST_SRC_PATH = rustPlatform.rustLibSrc;
        };

      devShells."2024" = with pkgs;
        mkShell {
          packages = [
            haskell
          ];
        };
    }
    );
}
