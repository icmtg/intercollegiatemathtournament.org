{ lib, stdenvNoCC, just, tailwindcss_4, zola, baseUrl ? null }:

stdenvNoCC.mkDerivation {
  pname = "berkeley.mt";
  version = "1.0.0";
  src = ./old;
  nativeBuildInputs = [ just tailwindcss_4 zola ];
  buildPhase = ''
    just build ${lib.optionalString (baseUrl != null) "--base-url ${baseUrl}"}
  '';
  installPhase = ''
    mkdir -p "$out"
    cp -r public/* "$out"
  '';
}
