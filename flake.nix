{
  description = "A minimal, self-hosted, ci-system for nix flakes";
  inputs = {
    utils.url = "github:numtide/flake-utils";
    naersk.url = "github:nix-community/naersk";
  };

  outputs = { self, nixpkgs, utils, naersk }:
    utils.lib.eachDefaultSystem (system: let
      pkgs = nixpkgs.legacyPackages."${system}";
      naersk-lib = naersk.lib."${system}";
      mdbook-nix = naersk-lib.buildPackage {
        pname = "mdbook-nix";
        root = ./.;
      };
    in {
      # `nix build`
      packages.mdbook-nix = mdbook-nix;
      defaultPackage = mdbook-nix;

      # `nix develop`
      devShell = pkgs.mkShell {
        buildInputs = with pkgs;
          [
            rustc cargo
            clippy rust-analyzer rustfmt
            mdbook
          ];
      };
    });
}
