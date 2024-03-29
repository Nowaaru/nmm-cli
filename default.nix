{
  lib,
  rustPlatform,


  rustc, 
  cargo,
  pkg-config,
  openssl,
  stdenv
}:
stdenv.mkDerivation {
  name = "nmm-cli";
  version = "0.0.1";

  src = ./target/release/nmm-cli;

  installPhase = ''
    cp $src $out
  '';
}

# rustPlatform.buildRustPackage rec {
#   pname = "nmm-cli";
#   version = "0.0.1";
#
#   src = lib.cleanSource ./.;
#
#   nativeBuildInputs = [ rustc cargo openssl.dev pkg-config ];
#
#   cargoLock.lockFile = ./Cargo.lock;
#
#   buildPhase = ''
#     export HOME=$(pwd)
#   '';
#
#   meta = with lib; {
#     description = "A fast line-oriented regex search tool, similar to ag and ack";
#     homepage = "https://github.com/BurntSushi/ripgrep";
#     license = licenses.unlicense;
#     maintainers = [];
#   };
# }
#
