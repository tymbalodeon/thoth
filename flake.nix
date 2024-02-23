{
  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixos-unstable";
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

          buildPackages = [
            libiconv
          ];

          darwinBuildPackages = [
            zlib.dev
            darwin.apple_sdk.frameworks.CoreFoundation
            darwin.apple_sdk.frameworks.CoreServices
            darwin.apple_sdk.frameworks.SystemConfiguration
            darwin.IOKit
          ];

          linuxBuildPackages = [
            pkg-config
            openssl
          ];
        in
        {
          default = craneLib.buildPackage {
            src = craneLib.cleanCargoSource (craneLib.path ./.);

            buildInputs = buildPackages ++ (
              if stdenv.isDarwin
              then darwinBuildPackages
              else
                (
                  if stdenv.isLinux
                  then linuxBuildPackages
                  else [ ]
                )
            );
          };
        });

      devShells = forEachSupportedSystem ({ pkgs, system }:
        with pkgs;
        let
          buildPackages = [
            libiconv
          ];

          darwinBuildPackages = [
            zlib.dev
            darwin.apple_sdk.frameworks.CoreFoundation
            darwin.apple_sdk.frameworks.CoreServices
            darwin.apple_sdk.frameworks.SystemConfiguration
            darwin.IOKit
          ];

          linuxBuildPackages = [
            pkg-config
            openssl
          ];

          devPackages = [
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
            nil
            lilypond-unstable
          ];

          FONTCONFIG_FILE = makeFontsConf
            {
              fontDirectories = [ pkgs.freefont_ttf ];
            };
        in
        {
          default = pkgs.mkShell {
            packages = buildPackages ++ devPackages ++ (
              if stdenv.isDarwin
              then darwinBuildPackages
              else
                (
                  if stdenv.isLinux
                  then linuxBuildPackages
                  else [ ]
                )
            );

            env = {
              inherit FONTCONFIG_FILE;

              RUST_BACKTRACE = "1";
            };
          };
        });
    };
}
