{ pkgs }: {
  deps = [
    # Rust toolchain
    pkgs.rustc
    pkgs.cargo
    pkgs.rust-analyzer
    pkgs.clippy
    pkgs.rustfmt

    # Build dependencies
    pkgs.openssl
    pkgs.pkg-config

    # Python (for scripts)
    pkgs.python311
    pkgs.python311Packages.pip

    # Node.js (for future frontend)
    pkgs.nodejs_20
  ];

  env = {
    RUST_SRC_PATH = "${pkgs.rust.packages.stable.rustPlatform.rustLibSrc}";
  };
}
