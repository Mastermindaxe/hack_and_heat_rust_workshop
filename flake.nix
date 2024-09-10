{
  inputs = {
    naersk.url = "github:nix-community/naersk/master";
    nixpkgs.url = "github:NixOS/nixpkgs/nixpkgs-unstable";
    utils.url = "github:numtide/flake-utils";
  };

  outputs = { self, nixpkgs, utils, naersk }:
    utils.lib.eachDefaultSystem (system:
      let
        pkgs = import nixpkgs { inherit system; };
        naersk-lib = pkgs.callPackage naersk { };
      in {
        packages.default = naersk-lib.buildPackage {
          pname = "benchmarks";
          src = ./part_2;
        };
        packages.benchmarks = naersk-lib.buildPackage {
          pname = "benchmarks";
          src = ./part_2;
        };
        packages.doc_tests = naersk-lib.buildPackage {
          pname = "doc_tests";
          src = ./part_2;
        };
        packages.shuttle = naersk-lib.buildPackage {
          pname = "hack-and-heat-main";
          src = ./part_2;
        };
        packages.data = naersk-lib.buildPackage {
          pname = "data";
          src = ./part_2;
        };

        checks = {
          test = naersk-lib.buildPackage {
            src = ./part_2;
            mode = "test";
          };
          clippy = naersk-lib.buildPackage {
            src = ./part_2;
            mode = "clippy";
          };
          format = naersk-lib.buildPackage {
            src = ./part_2;
            mode = "fmt";
            nativeBuildInputs = [ pkgs.rustfmt ];
          };
        };

        devShells.default = with pkgs;
          mkShell {
            buildInputs = [
              cargo
              rustc
              rustfmt
              pre-commit
              rustPackages.clippy
              cargo-shuttle
              libiconv
            ];
            RUST_SRC_PATH = rustPlatform.rustLibSrc;
          };
        formatter = pkgs.nixfmt;
      });
}
