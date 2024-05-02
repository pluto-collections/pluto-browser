{
  description = "Pluto browser flake";

  inputs = {
    nixpkgs.url = "github:nixos/nixpkgs?ref=nixpkgs-unstable";
    rust-overlay = {
      url = "github:oxalica/rust-overlay";
      inputs = {
        nixpkgs.follows = "nixpkgs";
      };
    };
  };
  outputs =
    {
      self,
      nixpkgs,
      rust-overlay,
    }:
    let
      system = "x86_64-linux";
      version = "1.77.0";
      overlays = [ (import rust-overlay) ];
      pkgs = import nixpkgs { inherit system overlays; };
      rust_toolchain = pkgs.pkgsBuildHost.rust-bin.stable.${version}.default.override {
      	extensions = [ "rust-analyzer" "clippy" ];
      };
      buildInputs = with pkgs; [
        rust_toolchain
        pkg-config
        gst_all_1.gstreamer
        gst_all_1.gst-plugins-base
        gst_all_1.gst-plugins-good
        gst_all_1.gst-plugins-bad
        gst_all_1.gst-plugins-ugly
        gst_all_1.gst-libav
        gst_all_1.gst-vaapi
      ];
      nativeBuildInputs = with pkgs; [
        openssl
        pango
        glib
        cacert
        glib-networking
        webkitgtk_4_1
        gcc
      ];
    in
    with pkgs;
    {
      devShells.${system}.default = mkShell {
        inherit buildInputs nativeBuildInputs;
        shellHook = ''
          export GIO_MODULE_DIR=${glib-networking}/lib/gio/modules/
        '';
      };
    };
}
