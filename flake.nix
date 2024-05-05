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
      rust-version = "1.77.0";
      overlays = [ (import rust-overlay) ];
      pkgs = import nixpkgs { inherit system overlays; };
      rust_toolchain = pkgs.pkgsBuildHost.rust-bin.stable.${rust-version}.default;
      buildInputs = with pkgs; [
        pkg-config
        webkitgtk_4_1
        openssl.dev
        glib-networking
	dbus
      ];
      nativeBuildInputs = with pkgs; [
        gst_all_1.gstreamer
	rust_toolchain
        gst_all_1.gst-plugins-base
        gst_all_1.gst-plugins-good
        gst_all_1.gst-plugins-bad
        gst_all_1.gst-plugins-ugly
        gst_all_1.gst-libav
        gst_all_1.gst-vaapi
        openssl.dev
        pkg-config
        pango
        glib
        gdk-pixbuf
        webkitgtk_4_1
        gcc
        gtk3
        haskellPackages.webkit2gtk3-javascriptcore
        cairo
        librsvg
        makeWrapper
      ];
    in
    with pkgs;
    {
      devShells.${system}.default = mkShell {
        inherit nativeBuildInputs;
        buildInputs = buildInputs ++ [
          rust_toolchain.override
          {
            extensions = [
              "rust-analyzer"
              "clippy"
            ];
          }
        ];
        shellHook = ''
        	export GIO_MODULE_DIR=${glib-networking}/lib/gio/modules/
          	export GST_PLUGIN_SYSTEM_PATH_1_0=${gst_all_1.gstreamer.out}/lib/gstreamer-1.0:${gst_all_1.gst-plugins-base}/lib/gstreamer-1.0:${gst_all_1.gst-plugins-good}/lib/gstreamer-1.0:${gst_all_1.gst-plugins-bad}/lib/gstreamer-1.0:${gst_all_1.gst-plugins-ugly}/lib/gstreamer-1.0
		export XDG_DATA_DIRS=${pkgs.gsettings-desktop-schemas}/share/gsettings-schemas/${pkgs.gsettings-desktop-schemas.name}:${pkgs.gtk3}/share/gsettings-schemas/${pkgs.gtk3.name}:$XDG_DATA_DIRS
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
          doCheck = false;
          src = fetchFromGitHub {
            owner = "pluto-collections";
            repo = "pluto-browser";
            rev = "master";
            hash = "sha256-mQorimpJipM4tbGzkX7OEbsNGeEm9ntXxTuDtd4KzuE=";
          };
          buildPhase = ''
                export GIO_MODULE_DIR="${glib-networking}/lib/gio/modules/"
		cargo build --release --target x86_64-unknown-linux-gnu
            	  '';
          cargoLock.lockFile = ./Cargo.lock;
          installPhase = ''
		mkdir -p $out/bin
                mv target/x86_64-unknown-linux-gnu/release/pluto-browser $out/bin/
                wrapProgram $out/bin/pluto-browser \
                	--set GIO_MODULE_DIR "${glib-networking}/lib/gio/modules/" \
            		--set GST_PLUGIN_SYSTEM_PATH_1_0 "${gst_all_1.gstreamer.out}/lib/gstreamer-1.0:${gst_all_1.gst-plugins-base}/lib/gstreamer-1.0:${gst_all_1.gst-plugins-good}/lib/gstreamer-1.0:${gst_all_1.gst-plugins-bad}/lib/gstreamer-1.0:${gst_all_1.gst-plugins-ugly}/lib/gstreamer-1.0"
                        '';
        };
    };
}
