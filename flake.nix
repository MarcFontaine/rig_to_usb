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
  in
    {
    test = fenix.packages.x86_64-linux;
    inherit pkgs;  
    packages.x86_64-linux.default = fenix.packages.x86_64-linux.minimal.toolchain;
    devShells.x86_64-linux.default = pkgs.mkShell {
      name = "rust-devshell";

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
        echo "Rust DevShell aktiv!"
        echo "Rust-Version: $(rustc --version)"
      '';
    };
  };
}
