{
  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixos-23.05";
    devenv.url = "github:cachix/devenv";
  };

  outputs = {
    nixpkgs,
    devenv,
    ...
  } @ inputs: let
    pkgs = nixpkgs.legacyPackages."x86_64-linux";
    name = "leafslug";
    server-internal-address = "127.0.0.1";
    server-internal-port = 8081;
    server-image-port = 8081;
    environment = "development";
    postgres_user = "prma";
    postgres_password = "localpassword";
    postgres_database = "leafslug";
    postgres_address = "127.0.0.1";
    postgres_port = 5432;
    postgres_host_auth_method = "trust";
    postgres_image_name = "postgres:15.3-alpine";
    postgres-container_name = "pg-leafslug";
    database_url = "postgres://${postgres_user}:${postgres_password}@${postgres_address}:${toString postgres_port}/${postgres_database}";
  in {
    devShells.x86_64-linux.default = devenv.lib.mkShell {
      inherit inputs pkgs;
      modules = [
        (
          {pkgs, ...}: {
            env = {
              SERVER_ADDRESS = server-internal-address;
              DATABASE_URL = database_url;
              DATABASE_PORT = postgres_port;
              DATABASE_ADDRESS = postgres_address;
              DATABASE_NAME = postgres_database;
              DATABASE_PASS = postgres_password;
              DATABASE_USER = postgres_user;
              SERVER_PORT = toString server-internal-port;
              DEPLOY_ENV = environment;

              SQLX_OFFLINE_DIR = "./.sqlx";
            };

            languages = {
              rust = {
                enable = true;
              };
              nix.enable = true;
            };

            packages = let
              sql-packages = with pkgs; [
                sqlx-cli
                pgcli
                pgweb
                sqlfluff
              ];
            in
              sql-packages;

            pre-commit = {
              hooks = {
                # for nix
                deadnix.enable = true;
                alejandra.enable = true;
                nil.enable = true;

                # for markdown
                markdownlint.enable = true;

                # for github
                actionlint.enable = false; # gives some stupid ass error

                # for git
                commitizen.enable = true;

                # for docker
                hadolint.enable = true;

                # for rust
              };
            };

            scripts = {
              image-build.exec = "podman build --rm -t ${name}:alpha . --no-cache";
              image-run.exec = "podman run -d -p ${toString server-internal-port}:${toString server-image-port} --name ${name} ${name}:alpha";
              image-remove.exec = "podman container rm ${name}";
              postgres-create.exec = "podman run -d  --name ${postgres-container_name} -e POSTGRES_HOST_AUTH_METHOD=${postgres_host_auth_method} -e  POSTGRES_USER=${postgres_user} -e POSTGRES_PASSWORD=${postgres_password} -e POSTGRES_DB=${postgres_database} -p ${postgres_address}:${toString postgres_port}:5432 ${postgres_image_name}";
              postgres-run.exec = "podman container start ${postgres-container_name}";
              postgres-open--shell.exec = "podman exec -it ${postgres-container_name} psql -U ${postgres_user}  -d ${postgres_database}";
            };
          }
        )
      ];
    };
  };
}
