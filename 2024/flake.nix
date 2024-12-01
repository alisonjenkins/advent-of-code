{
  description = "Terraform project flake";

  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixos-24.11";
    nixpkgs_unstable.url = "github:NixOS/nixpkgs/nixpkgs-unstable";

    rust-overlay = {
      url = "github:oxalica/rust-overlay";
      inputs.nixpkgs.follows = "nixpkgs";
    };
  };

  outputs = { ... } @ inputs:
    let
      forEachShellSystem = f: lib.genAttrs shellSystems (system: f system);
      forEachSupportedSystem = f: lib.genAttrs supportedSystems (system: f system);

      imageName = "advent-of-code";
      imageTag = "latest";
      lib = inputs.nixpkgs.lib;

      supportedSystems = [
        "aarch64-linux"
        "x86_64-linux"
      ];

      shellSystems = [
        "aarch64-darwin"
        "aarch64-linux"
        "x86_64-darwin"
        "x86_64-linux"
      ];

      mkDockerImage =
        pkgs: targetSystem:
        let
          archSuffix = if targetSystem == "x86_64-linux" then "amd64" else "arm64";

          container_packages = { pkgs }: with pkgs.pkgsStatic; [
            coreutils
            dockerTools.binSh
            dockerTools.caCertificates
            go
            unstable.pkgsStatic.just
            unstable.terraform
            unstable.pkgsStatic.terraformer
          ];
        in
        pkgs.dockerTools.buildLayeredImage {
          name = imageName;
          tag = "${imageTag}-${archSuffix}";
          contents = pkgs.buildEnv {
            name = "image-root";
            paths = container_packages { inherit pkgs; };
            pathsToLink = [ "/bin" ];
          };
        };
    in
    {
      packages = forEachSupportedSystem (
        system:
        let
          pkgs = import inputs.nixpkgs {
            inherit system;

            config = {
              allowUnfree = true;
            };

            overlays = [
              (
                final: _prev: {
                  unstable = import inputs.nixpkgs_unstable
                    {
                      system = final.system;
                      config.allowUnfree = true;
                    };
                }
              )
            ];
          };

          buildForLinux =
            targetSystem:
            if system == targetSystem then
              mkDockerImage pkgs targetSystem
            else
              mkDockerImage
                (import inputs.nixpkgs {
                  localSystem = system;
                  crossSystem = targetSystem;

                  config = {
                    allowUnfree = true;
                  };

                  overlays = [
                    (
                      final: _prev: {
                        unstable = import inputs.nixpkgs_unstable
                          {
                            system = final.system;
                            config.allowUnfree = true;
                          };
                      }
                    )
                  ];
                })
                targetSystem;
        in
        {
          "amd64" = buildForLinux "x86_64-linux";
          "arm64" = buildForLinux "aarch64-linux";
        }
      );

      devShells = forEachShellSystem
        (system:
          let
            pkgs = import inputs.nixpkgs {
              inherit system;

              config = {
                allowUnfree = true;
              };

              overlays = [
                (
                  final: _prev: {
                    unstable = import inputs.nixpkgs_unstable
                      {
                        system = final.system;
                        config.allowUnfree = true;
                      };
                  }
                )
                (import inputs.rust-overlay)
              ];
            };
          in
          {
            default = pkgs.mkShellNoCC {
              packages = with pkgs; [
                dive
                hyperfine
                just
                libiconv
                unstable.delve
                unstable.go
                lldb
                (rust-bin.stable.latest.default.override {
                  extensions = [ "rust-src" "llvm-tools-preview" ];
                })
              ] ++ (if pkgs.stdenv.isDarwin then with pkgs.darwin.apple_sdk.frameworks;[
                CoreFoundation
                CoreServices
                Security
                SystemConfiguration
              ] else [ ]);

              shellHook = ''
                export LIBCLANG_PATH="${pkgs.llvmPackages.libclang}/lib";
              '';
            };
          });
    };
}

