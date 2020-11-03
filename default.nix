{ system ? builtins.currentSystem }:

let
  sources = import ./nix/sources.nix;
  pkgs = import sources.nixpkgs { };
  helloworld = import ./helloworld.nix { inherit sources pkgs; };

  name = "sansculotte/images";
  tag = "latest";

in pkgs.dockerTools.buildLayeredImage {
  inherit name tag;
  contents = [ images ];

  config = {
    Cmd = [ "/bin/images" ];
    Env = [ "ROCKET_PORT=5000" ];
    WorkingDir = "/";
  };
}
