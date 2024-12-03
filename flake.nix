{
	description = "Advent of Code";
	
	inputs = {
		# nixpkgs.url = "github:NixOS/nixpkgs/nixpkgs-unstable";
		nixpkgs.url = "github:NixOS/nixpkgs/09801c985b4449de0736603b1c2d8652bfab4b93";
		flake-utils.url = "github:numtide/flake-utils";
		
		crane = {
			url = "github:ipetkov/crane";
		};
		
		fenix = {
			url = "github:nix-community/fenix";
			inputs.nixpkgs.follows = "nixpkgs";
		};
	};
	
	outputs = { self, nixpkgs, flake-utils, ... } @ inputs: let
		makeCraneLib = pkgs: let
			fenix = inputs.fenix.packages.${pkgs.system};
			fenixChannel = fenix.stable;
			fenixToolchain = fenixChannel.withComponents [
				"rustc"
				"cargo"
				"rust-std"
				"rust-docs"
				"clippy"
			];
		in (inputs.crane.mkLib pkgs).overrideToolchain fenixToolchain;
	in {
		overlays = {
			adventOfCode = final: prev: {
				adventOfCode = prev.callPackage ./nix/package.nix {
					craneLib = makeCraneLib final;
				};
			};
			
			default = self.overlays.adventOfCode;
		};
	} // flake-utils.lib.eachDefaultSystem (system:
		let
			pkgs = import nixpkgs {
				inherit system;
				overlays = [self.overlays.default];
			};
			craneLib = makeCraneLib pkgs;
		in {
			packages = {
				inherit (pkgs) adventOfCode;
				default = self.packages.${system}.adventOfCode;
			};
			
			devShells.default = craneLib.devShell {
				packages = with pkgs; [
					rust-analyzer
					pkg-config
					clang # needed to use lld
					llvmPackages.bintools # lld
					openssl
				];
			};
		}
	);
}
