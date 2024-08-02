 let
   nixpkgs = fetchTarball https://github.com/nixos/nixpkgs/archive/nixpkgs-unstable.tar.gz;
   pkgs = import nixpkgs { config = {}; overlays = []; };
 in

 pkgs.mkShellNoCC {
   packages = with pkgs; [
     rustup
     fermyon-spin
     # hurl
   ];
 }
