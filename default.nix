{ lib, stdenvNoCC, just, tailwindcss_4, zola, baseUrl ? null }:

stdenvNoCC.mkDerivation {
  pname = "berkeley.mt";
  version = "1.0.0";
  src = ./.;
  nativeBuildInputs = [ just tailwindcss_4 zola ];
  buildPhase = ''
    just build -o "$out" ${lib.optionalString (baseUrl != null) "--base-url ${baseUrl}"}
  '';
}
