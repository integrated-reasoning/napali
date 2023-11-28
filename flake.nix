{
  inputs = {
    nixpkgs.url = "https://flakehub.com/f/NixOS/nixpkgs/*.tar.gz";
    cargo2nix.url = "github:cargo2nix/cargo2nix/release-0.11.0";
    flake-utils.url = "github:numtide/flake-utils/v1.0.0";
  };

  outputs = inputs: with inputs;
    flake-utils.lib.eachDefaultSystem (system:
      let
        pkgs = import nixpkgs { inherit system; overlays = [ cargo2nix.overlays.default ]; };
        inherit (pkgs) lib;

        rustPackageSet = pkgs.rustBuilder.makePackageSet {
          rustVersion = "1.71.1";
          packageFun = import ./Cargo.nix;
          extraRustComponents = [ "rustfmt" "clippy" ];
        };

        isDarwin = lib.strings.hasSuffix "-darwin" system;
        darwinDependencies = lib.optionals isDarwin [
          pkgs.darwin.apple_sdk.frameworks.SystemConfiguration
        ];

        generalBuildInputs = [
          pkgs.cargo-nextest
          pkgs.gitlab-clippy
          pkgs.mold
          pkgs.openssl
          pkgs.openssl.dev
          pkgs.pkg-config
          pkgs.rustup
        ] ++ darwinDependencies;

        napali = args: (rustPackageSet.workspace.napali ({ } // args)).overrideAttrs {
          buildInputs = generalBuildInputs;
        };

        workspaceShell = rustPackageSet.workspaceShell {
          RUSTFLAGS = "--cfg tokio_unstable";
          packages = generalBuildInputs;
        };

      in
      {
        packages = {
          default = napali { };
          tests = napali { compileMode = "test"; };
          ci = pkgs.rustBuilder.runTests napali { };
        };
        devShell = workspaceShell;
      }
    );
}
