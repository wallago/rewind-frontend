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
      in {
        devShells.default = pkgs.mkShell {
          nativeBuildInputs = with pkgs; [ pkg-config openssl ];
          buildInputs = with pkgs;
            [ dioxus-cli binaryen wasm-bindgen-cli tailwindcss watchman ]
            ++ [ rust ];
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

