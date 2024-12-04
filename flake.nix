{
  description = "rust shell";

  inputs = {
    nixpkgs.url = "github:nixos/nixpkgs/nixos-unstable";
    rust-overlay.url = "github:oxalica/rust-overlay";
  };

  outputs = { self, nixpkgs, rust-overlay, ... }:
  let
    overlays = [ (import rust-overlay)];
    supportedSystems = ["x86_64-linux" "aarch64-linux" "x86_64-darwin" "aarch64-darwin" ];
    forEachSupportedSystem = f: nixpkgs.lib.genAttrs supportedSystems (system: f {
      pkgs = import nixpkgs { inherit system overlays; };
    });
  in
  {
    devShells = forEachSupportedSystem ({ pkgs }: {
      name = "rust";
      default = pkgs.mkShell {
        packages = with pkgs; [
          pkg-config
          openssl
        ];
        buildInputs = with pkgs; [
          (rust-bin.beta.latest.default.override {
            extensions = [ "rust-src" ];
          })
        ];
      };
    });
  };
}
