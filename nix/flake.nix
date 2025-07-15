{
  description = "Nix dev environment";

  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixos-unstable";
    flake-utils.url = "github:numtide/flake-utils";
    rust-overlay.url = "github:oxalica/rust-overlay";
  };

  outputs = { nixpkgs, flake-utils, rust-overlay, ... }:
    flake-utils.lib.eachDefaultSystem (system:
      let
        overlays = [ (import rust-overlay) ];
        pkgs = import nixpkgs { inherit system overlays; };
        rust = pkgs.rust-bin.nightly.latest.complete.override {
          targets = [ "wasm32-unknown-unknown" ];
        };
        commonNativeBuildInputs = with pkgs; [ pkg-config openssl ];
        commonBuildInputs = with pkgs; [
          binaryen
          wasm-bindgen-cli
          tailwindcss
        ];
      in {
        devShells.default = pkgs.mkShell {
          nativeBuildInputs = commonNativeBuildInputs;
          buildInputs = with pkgs;
            [ dioxus-cli watchman ] ++ [ rust ] ++ commonBuildInputs;
          shellHook = ''
            echo "
            🐚 Rust dev shell ready!
            Run: dx serve"

            echo "For tailwindcss changes!  
            Run: tailwindcss -i ./tailwind.css -o ./assets/tailwind.css --watch"
          '';
        };
      });
}

