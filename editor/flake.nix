{
	description = "Cadenza music notation editor";

	inputs = {
		nixpkgs.url = "github:nixos/nixpkgs/nixos-unstable";
		naersk.url = "github:nix-community/naersk/master";
		naersk.inputs.nixpkgs.follows = "nixpkgs";
	};

	outputs = { self, nixpkgs, naersk }:
		let
			inherit (nixpkgs) lib;
			forAllSystems = lib.genAttrs lib.systems.flakeExposed;
		in
		{
			packages = forAllSystems (system:
				let
					pkgs = nixpkgs.legacyPackages.${system};
				in rec {
					default = cadenza;
					cadenza = pkgs.callPackage ./default.nix {
						naersk-lib = naersk.lib.${system};
					};
				});

			devShells = forAllSystems (system:
				let
					pkgs = nixpkgs.legacyPackages.${system};
					runtimeLibs = with pkgs; [ wayland libX11 libxkbcommon ];
				in {
					default = pkgs.mkShell {
						nativeBuildInputs = with pkgs; [ pkg-config ];
						buildInputs = runtimeLibs;

						shellHook = ''
							export LD_LIBRARY_PATH="$LD_LIBRARY_PATH:${pkgs.lib.makeLibraryPath runtimeLibs}"
						'';
					};
				});

			apps = forAllSystems (system: rec {
				default = cadenza;
				cadenza = {
					type = "app";
					program = "${lib.getBin self.packages.${system}.cadenza}/bin/cadenza";
				};
			});
		};
}
