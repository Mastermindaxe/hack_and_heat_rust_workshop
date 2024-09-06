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
        packages.default = naersk-lib.buildPackage ./part_2/benchmarks;
        packages.benchmarks = naersk-lib.buildPackage ./part_2/benchmarks;
        packages.doc_tests = naersk-lib.buildPackage ./part_2/doc_tests;
        packages.macros_and_meta_programming =
          naersk-lib.buildPackage ./part_2/macros_and_meta_programming;

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
            buildInputs =
              [ cargo rustc rustfmt pre-commit rustPackages.clippy ];
            RUST_SRC_PATH = rustPlatform.rustLibSrc;
          };
        formatter = pkgs.nixfmt;
      });
}
