{
  inputs = {
    fenix = {
      url = "github:nix-community/fenix";
      inputs.nixpkgs.follows = "nixpkgs";
    };
    nixpkgs.url = "nixpkgs/nixos-26.05";
  };
  outputs = { self, fenix, nixpkgs }:
  let
    system = "x86_64-linux";
    pkgs = import nixpkgs {
      inherit system;
      overlays = [ fenix.overlays.default ];
    };
    toolchain = with fenix.packages.${system}; combine [
      minimal.cargo
      minimal.rustc
      targets.thumbv7em-none-eabihf.latest.rust-std
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
        pkgs.probe-rs-tools
        pkgs.flip-link
        rust-analyzer
      ];

      CARGO_BUILD_TARGET = "thumbv7em-none-eabihf";
      CARGO_TARGET_AARCH64_UNKNOWN_LINUX_GNU_LINKER =   let
              inherit (pkgs.pkgsCross.aarch64-multiplatform.stdenv) cc;
            in
       "${cc}/bin/${cc.targetPrefix}cc";

      shellHook = ''
        echo "Rust DevShell aktiv!"
        echo "Rust-Version: $(rustc --version)"
      '';
    };
  };
}
