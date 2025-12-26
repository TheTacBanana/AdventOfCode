{
  pkgs ? import <nixpkgs> { },
}:

pkgs.mkShell {
  buildInputs = [
    pkgs.z3
    pkgs.llvmPackages.clang
    pkgs.llvmPackages.libclang
    pkgs.llvmPackages.bintools
  ];

  LIBCLANG_PATH = "${pkgs.llvmPackages.libclang.lib}/lib";

  BINDGEN_EXTRA_CLANG_ARGS = "-I${pkgs.glibc.dev}/include";
}
