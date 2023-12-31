{
  description = "Optimization as a service TUI";

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

        buildInputs = [
          pkgs.cargo-nextest
          pkgs.mold
          pkgs.openssl
          pkgs.openssl.dev
          pkgs.pkg-config
          pkgs.rustup
        ] ++ lib.optionals pkgs.stdenv.isLinux [
          pkgs.cargo-llvm-cov
        ] ++ lib.optionals pkgs.stdenv.isDarwin [
          pkgs.darwin.apple_sdk.frameworks.SystemConfiguration
        ];

        napali = args: (rustPackageSet.workspace.napali ({ } // args)).overrideAttrs {
          inherit buildInputs;
        };

        workspaceShell = rustPackageSet.workspaceShell {
          RUSTFLAGS = "--cfg tokio_unstable";
          packages = buildInputs;
        };

      in
      rec
      {
        packages = {
          default = napali { };
          tests = napali { compileMode = "test"; };
          ci = pkgs.rustBuilder.runTests napali { };
        };

        devShell = workspaceShell;

        image = pkgs.dockerTools.buildLayeredImage {
          name = "napali";
          tag = "latest";
          maxLayers = 120;
          contents = [
            pkgs.cacert
            packages.default
          ];
          config.Cmd = [ "napali" ];
        };
      }
    );
}
