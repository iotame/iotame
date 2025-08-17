{
  pkgs,
  lib,
  config,
  inputs,
  ...
}: {
  env.PROTOC = "${pkgs.protobuf}/bin/protoc";
  env.PROTOC_INCLUDE = "${pkgs.protobuf}/include";

  packages = with pkgs; [
    git
    protobuf
  ];

  languages.rust.enable = true;
  languages.javascript.bun.enable = true;

  # https://devenv.sh/tasks/
  # tasks = {
  #   "myproj:setup".exec = "mytool build";
  #   "devenv:enterShell".after = [ "myproj:setup" ];
  # };

  # https://devenv.sh/git-hooks/
  git-hooks.hooks = {
    # TODO: Determine how to run clippy only in ./core
    # clippy.enable = true;
    alejandra.enable = true;
  };
}
