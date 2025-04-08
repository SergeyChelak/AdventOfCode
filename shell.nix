{ 
  pkgs ? import <nixpkgs> {} 
}: pkgs.mkShell {
  RUST_SRC_PATH = "${pkgs.rust.packages.stable.rustPlatform.rustLibSrc}";
  buildInputs = with pkgs; [ 
    rustc 
    cargo 
    rustfmt 
    bacon
    rustPackages.clippy 
    gcc
    rust-analyzer
  ];  
  RUST_BACKTRACE = 1;
}
