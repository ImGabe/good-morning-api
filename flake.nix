{
  inputs = {
    flake-utils.url = "github:numtide/flake-utils";
    naersk.url = "github:nmattia/naersk";
  };

  outputs = { self, nixpkgs, flake-utils, naersk }:
    flake-utils.lib.eachDefaultSystem (
      system: let
        pkgs = nixpkgs.legacyPackages."${system}";
        naersk-lib = naersk.lib."${system}";
      in
        rec {
          # `nix build`
          packages.good-morning-api = naersk-lib.buildPackage {
            pname = "good-morning-api";
            root = ./.;
          };
          defaultPackage = packages.good-morning-api;

          # `nix run`
          apps.good-morning-api = flake-utils.lib.mkApp {
            drv = packages.good-morning-api;
          };
          defaultApp = apps.good-morning-api;

          # `nix develop`
          devShell = pkgs.mkShell {
            nativeBuildInputs = with pkgs; [ rustc cargo rust-analyzer docker-compose ];
          };
        }
    );
}
