# nmm-cli

###### the nix-mod-manager cli

- [x] lockfile generation
- [ ] ~~submitting lockfile to store~~
- [ ] use `nmm get-store` every home manager generation
- [ ] use `fetchStoreMod` with provider + game id + mod id args to retrieve store mod

nix-mod-manager is a currently presented with a problem that
forces users to have to fetch the hash of a request (a de facto orobouros problem).

since internet is not provided during derivation realisation,
this means that API services are a direct roadblock when
fetching mod files through an API.

this is an interface directly intertwined with the `nix` commands
(which means non-nix users, shoo!!) to access specific popular mod
websites that require an api to generate download links.

nmm-cli also supports standalone usage to fetch mods from the command
line to place into the nix store.
