{
  inputs = {
    fenix = {
      url = "github:nix-community/fenix";
      inputs.nixpkgs.follows = "nixpkgs";
    };
    nixpkgs.url = "nixpkgs/master";
  };
  outputs = { self, fenix, nixpkgs }:
  let
    system = "x86_64-linux";
    pkgs = import nixpkgs {
      inherit system;
      overlays = [ fenix.overlays.default ];
    };
    toolchain = with fenix.packages.${system}; combine [
      complete.cargo
      complete.rustc
      complete.llvm-tools
      targets.thumbv7em-none-eabihf.latest.rust-std
      targets.thumbv7m-none-eabi.latest.rust-std
      targets.x86_64-unknown-linux-gnu.latest.rust-std
      # targets.wasm32-unknown-unknown.latest.rust-std
      targets.wasm32v1-none.latest.rust-std
    ];
    gst_webrtc = (pkgs.gst_all_1.gst-plugins-rs
      .override {plugins = [ "webrtc" "rtp" ]; })
      .overrideAttrs (oldAttrs: {
        doCheck = false;
      });
    gstPluginPaths = pkgs.lib.makeSearchPathOutput "lib" "/lib/gstreamer-1.0" (
    with pkgs;
    [
      gst_all_1.gstreamer
      gst_all_1.gst-plugins-base
      gst_all_1.gst-plugins-good
      gst_all_1.gst-plugins-ugly
      gst_all_1.gst-plugins-bad
#      gst_all_1.gst-plugins-rs
      gst_webrtc
      libnice
    ]
  );

  in
    {
    inherit gst_webrtc;
    test = fenix.packages.x86_64-linux;
    inherit pkgs;
    devShells.x86_64-linux.default = pkgs.mkShell {
      name = "rust-devshell";
      nativeBuildInputs = [ pkgs.pkg-config ];
      buildInputs = with pkgs; [
        toolchain
        probe-rs-tools
        cargo-binutils
        cargo-bloat
        flip-link
        cargo-edit
        rust-analyzer
        dfu-util
        evcxr
        # wasm-bindgen-cli_0_2_126 # for std
        twiggy # analyse .wasm
        wabt # webassembly tools wasm-strip target/wasm32-unknown-unknown/release/dein_projekt.wasm
        # remote-adapter / gstreamer
        glib
#        glibc.dev
        gst_all_1.gstreamer.dev
        gst_all_1.gst-plugins-base.dev
        gst_all_1.gst-plugins-good.dev
        gst_all_1.gst-plugins-bad.dev
        gst_all_1.gst-plugins-ugly.dev
#        gst_all_1.gst-plugins-rs.dev
        gst_webrtc.dev
      ];

      CARGO_BUILD_TARGET = "thumbv7em-none-eabihf";
      CARGO_TARGET_AARCH64_UNKNOWN_LINUX_GNU_LINKER =   let
              inherit (pkgs.pkgsCross.aarch64-multiplatform.stdenv) cc;
            in
       "${cc}/bin/${cc.targetPrefix}cc";

      shellHook = ''
        export RUST_SRC_PATH="${toolchain}/lib/rustlib/src/rust/library"
        export PATH="${toolchain}/bin:$PATH"
        export EVCXR_COMPLETION_TYPE=circular
        export LD_LIBRARY_PATH="${pkgs.lib.makeLibraryPath [
          pkgs.glib
          pkgs.gst_all_1.gstreamer
          pkgs.gst_all_1.gst-plugins-base
          pkgs.gst_all_1.gst-plugins-good
          pkgs.gst_all_1.gst-plugins-bad
          pkgs.gst_all_1.gst-plugins-ugly
#         pkgs.gst_all_1.gst-plugins-rs
          gst_webrtc
        ]}:$LD_LIBRARY_PATH"
        export GST_PLUGIN_SYSTEM_PATH_1_0=${gstPluginPaths}
        echo "Rust DevShell aktiv!"
        echo "Rust-Version: $(rustc --version)"
      '';
    };
    packages.x86_64-linux.default = pkgs.rustPlatform.buildRustPackage rec {
      pname = "rig_to_usb";
      version = "0.0.0";
      src = ./.;

      cargoLock = {
        lockFile = ./Cargo.lock;
        outputHashes = {
        };
      };
      cargoBuildFlags = [
        "--bin" "stdin_to_cbor"
        "--features" "std-json"
        "--target" pkgs.stdenv.hostPlatform.config
      ];
      cargoTestFlags = [
        "--bin" "stdin_to_cbor"
        "--features" "std-json"
        "--target" pkgs.stdenv.hostPlatform.config
      ];
      nativeBuildInputs = [ pkgs.pkg-config ];
      buildInputs = [
      ];
    };
  };
}
