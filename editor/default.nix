{ lib
, pkg-config
, naersk-lib
# Add dependencies
, wayland
, libX11
, libxkbcommon
, makeWrapper
}:

naersk-lib.buildPackage {
	pname = "cadenza";
	version = "0.1.0-dev";

	root = ./..;
	src = ./..;

	cargoBuildFlags = [ "-p" "cadenza" ];
	cargoTestFlags = [ "-p" "cadenza" ];

	nativeBuildInputs = [ pkg-config makeWrapper ];
	buildInputs = [ wayland libX11 libxkbcommon ];

	# Link runtime libs for winit
	postInstall = ''
		wrapProgram $out/bin/cadenza \
			--prefix LD_LIBRARY_PATH : "${lib.makeLibraryPath [ wayland libX11 libxkbcommon ]}"
	'';

	meta = let inherit (lib) licenses platforms; in {
		description = "Music notation editor";
		homepage = "https://codeberg.org/twilit-jack/cadenza";
		license = licenses.agpl3Plus;
		platforms = platforms.unix;
	};
}
