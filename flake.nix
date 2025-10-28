{
  inputs.nixpkgs.url = "nixpkgs";

  outputs = {
    self,
    nixpkgs,
    ...
  }: let
    version = builtins.substring 0 7 self.lastModifiedDate;

    systems = [
      "x86_64-linux"
      "aarch64-linux"
      "x86_64-darwin"
      "aarch64-darwin"
    ];

    forAllSystems = nixpkgs.lib.genAttrs systems;
    nixpkgsFor = forAllSystems (system: import nixpkgs {inherit system;});

    packageFn = pkgs:
      pkgs.rustPlatform.buildRustPackage {
        pname = "goober-bot";
        inherit version;

        nativeBuildInputs = with pkgs; [
          pkg-config
        ];

        buildInputs = with pkgs; [
          openssl
          git
        ];

        src = builtins.path {
          name = "source";
          path = ./.;
        };

        fetchCargoVendor = true;
        cargoHash = "sha256-y/rRIZzz6bO7pY5IlI/p6x1TzJ0eaL3xJmxDYXxhlec=";

        separateDebugInfo = true;
      };
  in {
    packages = forAllSystems (s: let
      pkgs = nixpkgsFor.${s};
    in rec {
      goober-bot = packageFn pkgs;
      default = goober-bot;
    });

    devShells = forAllSystems (s: let
      pkgs = nixpkgsFor.${s};
      inherit (pkgs) mkShell;
    in {
      default = mkShell {
        packages = with pkgs; [
          pkg-config
          openssl
          git
        ];
      };
    });
  };
}
