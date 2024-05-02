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
        extensions = [
          "rust-analyzer"
          "clippy"
        ];
      };
      buildInputs = with pkgs; [
        rust_toolchain
        gst_all_1.gstreamer
        gst_all_1.gst-plugins-base
        gst_all_1.gst-plugins-good
        gst_all_1.gst-plugins-bad
        gst_all_1.gst-plugins-ugly
        gst_all_1.gst-libav
        gst_all_1.gst-vaapi
        pkg-config
        pango
        cacert
        gdk-pixbuf
        webkitgtk_4_1
        gcc
        gtk3
	libsoup
        cairo
        haskellPackages.webkit2gtk3-javascriptcore
        openssl_3
      ];
      nativeBuildInputs = with pkgs; [
        openssl.dev
        pkg-config
        pango
        glib
        glib-networking
        gdk-pixbuf
        webkitgtk_4_1
        gcc
        # gtk3
        haskellPackages.webkit2gtk3-javascriptcore
        cairo
        openssl_3
        librsvg
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
      packages.${system}.pluto-browser =
        let
          manifest = (lib.importTOML ./Cargo.toml).package;
        in
        rustPlatform.buildRustPackage {
          pname = "${manifest.name}";
          version = manifest.version;
          inherit buildInputs nativeBuildInputs;
          cargoLock.lockFile = ./Cargo.lock;
	  doCheck = false;
          src = fetchFromGitHub {
            owner = "pluto-collections";
            repo = "pluto-browser";
            rev = "master";
            hash = "sha256-mQorimpJipM4tbGzkX7OEbsNGeEm9ntXxTuDtd4KzuE=";
          };
	  shellHook = ''
          	export GIO_MODULE_DIR=${glib-networking}/lib/gio/modules/
	  '';
        };
    };
}
