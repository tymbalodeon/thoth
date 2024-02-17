
{
  inputs = {
    nixpkgs.url = "https://flakehub.com/f/NixOS/nixpkgs/0.1.*.tar.gz";
    schemas.url = "https://flakehub.com/f/DeterminateSystems/flake-schemas/*.tar.gz";

    rust-overlay = {
      url = "github:oxalica/rust-overlay";
      inputs.nixpkgs.follows = "nixpkgs";
    };

    crane = {
      url = "github:ipetkov/crane";
      inputs.nixpkgs.follows = "nixpkgs";
    };
  };

  outputs = { self, nixpkgs, schemas, rust-overlay, crane }:
    let
      overlays = [
        rust-overlay.overlays.default
        (final: prev: {
          rustToolchain = final.rust-bin.nightly.latest.default;
        })
      ];

      supportedSystems = [
        "x86_64-linux"
        "aarch64-darwin"
        "x86_64-darwin"
        "aarch64-linux"
      ];

      forEachSupportedSystem = f: nixpkgs.lib.genAttrs supportedSystems (
        system: f {
          inherit system;

          pkgs = import nixpkgs { inherit overlays system; };
        }
      );
    in
    {
      inherit schemas;

      packages = forEachSupportedSystem ({ pkgs, system }:
        with pkgs;
        let
          craneLib = crane.lib.${system};
          allPackages = [ libiconv ];

          darwinPackages = [
            zlib.dev
            darwin.apple_sdk_11_0.frameworks.CoreFoundation
            darwin.apple_sdk_11_0.frameworks.SystemConfiguration
            darwin.IOKit
          ];
        in
        {
          default = craneLib.buildPackage {
            src = craneLib.cleanCargoSource (craneLib.path ./.);

            buildInputs =
              if system == "x86_64-darwin"
              then allPackages ++ darwinPackages
              else allPackages;
          };
        });

      devShells = forEachSupportedSystem ({ pkgs, system }: {
        default = pkgs.mkShell {
          packages = with pkgs;
            let
              allPackages = [
                rustToolchain
                cargo-bloat
                cargo-edit
                cargo-outdated
                cargo-udeps
                cargo-watch
                rust-analyzer
                git
                just
                python311Packages.pre-commit-hooks
                nixpkgs-fmt
                libiconv
              ];

              darwinPackages = [
                zlib.dev
                darwin.apple_sdk_11_0.frameworks.CoreFoundation
                darwin.apple_sdk_11_0.frameworks.SystemConfiguration
                darwin.IOKit
              ];
            in
            if system == "x86_64-darwin"
            then allPackages ++ darwinPackages
            else allPackages;

          env = {
            RUST_BACKTRACE = "1";
          };
        };
      });
    };
}
