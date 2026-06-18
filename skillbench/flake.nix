{
  description = "skillbench — token benchmark for the caveman SKILL.md (rust-script)";

  inputs.nixpkgs.url = "github:NixOS/nixpkgs/nixos-25.05";

  outputs = { self, nixpkgs }:
    let
      systems = [ "aarch64-darwin" "x86_64-darwin" "aarch64-linux" "x86_64-linux" ];
      forAll = f: nixpkgs.lib.genAttrs systems (s: f nixpkgs.legacyPackages.${s});
    in {
      devShells = forAll (pkgs: {
        default = pkgs.mkShell {
          # rust-script pulls cargo+rustc through its closure; curl/jq for the
          # impure boundary. rageveil stays on the ambient PATH (user's own tool).
          packages = [ pkgs.rust-script pkgs.cargo pkgs.rustc pkgs.curl pkgs.jq pkgs.python3 ];
        };
      });

      # `nix run .#mock` / `nix run .#bench`
      apps = forAll (pkgs:
        let run = mode: {
          type = "app";
          program = toString (pkgs.writeShellScript "skillbench-${mode}" ''
            export PATH=${pkgs.lib.makeBinPath [ pkgs.rust-script pkgs.cargo pkgs.rustc pkgs.curl pkgs.python3 ]}:$PATH
            exec ${pkgs.rust-script}/bin/rust-script ${toString ./bench.rs} ${mode}
          '');
        };
        in { mock = run "mock"; bench = run "run"; default = run "run"; });
    };
}
