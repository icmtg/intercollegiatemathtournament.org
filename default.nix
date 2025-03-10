{ lib, stdenvNoCC, tailwindcss, zola, baseUrl ? null }:

stdenvNoCC.mkDerivation {
  pname = "berkeley.mt";
  version = "1.0.0";
  src = ./.;
  nativeBuildInputs = [ tailwindcss zola ];
  buildPhase = ''
    tailwindcss -i ./static/input.css -o ./static/style.css
    zola build -o "$out" ${lib.optionalString (baseUrl != null) "--base-url ${baseUrl}"}
  '';
}
