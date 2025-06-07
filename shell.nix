let
    pkgs = import <nixpkgs> {};

  runtimeLibs = with pkgs; [
    wayland
    libxkbcommon
    xorg.libX11
    xorg.libXrandr
    xorg.libXinerama
    xorg.libXcursor
    xorg.libXi
    xorg.libXfixes

  ];

in pkgs.mkShell {

  nativeBuildInputs = with pkgs; [
    pkg-config
    cargo
    rustc
    rust-analyzer
  ];

  buildInputs = runtimeLibs; 

  RUST_SRC_PATH = "${pkgs.rust.packages.stable.rustPlatform.rustLibSrc}";

  shellHook = ''
    export LD_LIBRARY_PATH="${pkgs.lib.makeLibraryPath runtimeLibs}:$LD_LIBRARY_PATH"
    export RUST_BACKTRACE=1 # Good for debugging
    echo "LD_LIBRARY_PATH set to: $LD_LIBRARY_PATH" # For verification
    unset TMP TEMP TEMPDIR TMPDIR

  '';
}  