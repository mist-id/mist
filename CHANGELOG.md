# Changelog
All notable changes to this project will be documented in this file. See [conventional commits](https://www.conventionalcommits.org/) for commit guidelines.

- - -
## [0.1.0](https://github.com/mist-id/mist/compare/7f62677dda2b84daf7148aefa4ca1a2e4624d545..0.1.0) - 2024-09-23
#### Bug Fixes
- **(api-docs)** change api directory (#68) - ([2bb4641](https://github.com/mist-id/mist/commit/2bb4641207fe4e235710583eeee094f4a8ccdf70)) - Danny
- **(authn)** only create a user if the identifier doesn't already exist (#69) - ([521e9d1](https://github.com/mist-id/mist/commit/521e9d1a710d47edc21414cff0b4a6fbf71ea792)) - Danny
- **(deps)** update rust crate axum-extra to v0.9.4 (#71) - ([3a12373](https://github.com/mist-id/mist/commit/3a123738741ef08c29a07fb05688c3c9e9baa62a)) - renovate[bot]
- **(deps)** update rust crate axum to v0.7.6 (#70) - ([88f085a](https://github.com/mist-id/mist/commit/88f085a5607d8fb8673ff4a5ba0ffe00926b4a6d)) - renovate[bot]
- **(deps)** update rust crate tower-http to 0.6.0 (#66) - ([d93940b](https://github.com/mist-id/mist/commit/d93940b87a35dc07e4b503e5e017615c521d418e)) - renovate[bot]
- **(deps)** update rust crate bon to v2.3.0 (#61) - ([0bbba7c](https://github.com/mist-id/mist/commit/0bbba7ccbeb261f5a92f3fc754cb4fde66734cb6)) - renovate[bot]
- **(deps)** update rust crate serde to v1.0.210 (#60) - ([bf01913](https://github.com/mist-id/mist/commit/bf01913468862ae0e88e73dffab022ef4ee38d00)) - renovate[bot]
#### Continuous Integration
- set user for release commit, but cuter (#87) - ([298cf5d](https://github.com/mist-id/mist/commit/298cf5d66b067742d7046c562e25c6703d81880f)) - Danny
- set user for release commit (#86) - ([779717d](https://github.com/mist-id/mist/commit/779717da957b271884900c13a030c232c16cf382)) - Danny
- fix docker build (#83) - ([79055da](https://github.com/mist-id/mist/commit/79055dad5c34c3848952e6297790e66fa95fc1d0)) - Danny
- set up ci workflow (#35) - ([858628a](https://github.com/mist-id/mist/commit/858628a9a9663fee918f1ee35800b6ced976d587)) - Danny
#### Documentation
- **(readme)** update logo (#76) - ([827f7b3](https://github.com/mist-id/mist/commit/827f7b3fb54a15f53ed90aa221e606d178aff2a9)) - Danny
- **(readme)** add logo (#75) - ([583d2ce](https://github.com/mist-id/mist/commit/583d2cec399395c7dd5621b4c57c260023de397d)) - Danny
- **(readme)** fix bad link (#51) - ([13d3f81](https://github.com/mist-id/mist/commit/13d3f812de3bfffc2871ba1a17ce20a452012f99)) - Danny
- **(readme)** update badges (#43) - ([ed7f9b6](https://github.com/mist-id/mist/commit/ed7f9b65940a78cd5e19bfac107c140b40afaf3c)) - Danny
- **(readme)** add roadmap link (#34) - ([1a9e7b1](https://github.com/mist-id/mist/commit/1a9e7b19d9935d0f436725514a8c5905af73d184)) - Danny
- update docs, remove demo deployment for now (#65) - ([6bd6002](https://github.com/mist-id/mist/commit/6bd6002c414fe0b3a981e0710a94002f349fc3e9)) - Danny
- clean up docs a bit, add a couple workflows (#36) - ([5c8c2af](https://github.com/mist-id/mist/commit/5c8c2af57cf92585cc526fcce5b4464ad4b8006a)) - Danny
- add dev guidelines page - ([98a7c20](https://github.com/mist-id/mist/commit/98a7c20f9d4aeff7bcfea6d431592d9ae2420357)) - [@its-danny](https://github.com/its-danny)
- add some starting documentation - ([3fd355d](https://github.com/mist-id/mist/commit/3fd355d360cf963bab14f50f9eabc76be2ac9e17)) - [@its-danny](https://github.com/its-danny)
#### Features
- **(api)** always require at least 1 active key of each type (#80) - ([e30adc5](https://github.com/mist-id/mist/commit/e30adc5e210f4afd9b402a2ac9b41e9abdc4c4f8)) - Danny
- **(api)** secure api via keys (#64) - ([2aece2f](https://github.com/mist-id/mist/commit/2aece2f538b767caadf2b73d469cd344d66d1c31)) - Danny
- **(api)** generate keys in-app (#63) - ([365d833](https://github.com/mist-id/mist/commit/365d833bee9a5dcece050f9cfa06e39ff70a36e5)) - Danny
- **(api,authn)** definitions should be optional (#79) - ([a9af01a](https://github.com/mist-id/mist/commit/a9af01a423e29bba8a15898e47a3b21db351feda)) - Danny
- **(authn)** split signing in from signing up (#73) - ([9b5a8e1](https://github.com/mist-id/mist/commit/9b5a8e17ea97d15590ea2b9e3898627fe0c7c99f)) - Danny
- **(authn)** verify token expiration (#53) - ([ee9c43f](https://github.com/mist-id/mist/commit/ee9c43f40d8e22188fe2239d8b8b1db827dde4cb)) - Danny
- **(authn)** verify `key_opts` before verifying token (#52) - ([a13d894](https://github.com/mist-id/mist/commit/a13d8941a1c01e8e5c96025f25bbe1b4346a72b9)) - Danny
- **(authn)** use env var for redis config (#41) - ([61cf49b](https://github.com/mist-id/mist/commit/61cf49b3c00abbe2646ed891e65b1f5d1208ea3c)) - Danny
- **(authn,jobs)** use worker queue for webhooks (#82) - ([d219d44](https://github.com/mist-id/mist/commit/d219d443c38ad0dcc464ca2172441edf99c420b3)) - Danny
- **(docs)** add openapi spec and docs (#45) - ([1d433ac](https://github.com/mist-id/mist/commit/1d433ac83cd3decef73380b1c29d1b5bea6a27d0)) - Danny
- add ability to end session (#48) - ([899e49d](https://github.com/mist-id/mist/commit/899e49d5d5f8fb3bfb702897cf3ab9975bb17198)) - Danny
- proof of concept - ([f6483a5](https://github.com/mist-id/mist/commit/f6483a57e13de035e6434302bdd00ef979bb580f)) - [@its-danny](https://github.com/its-danny)
#### Miscellaneous Chores
- **(common)** move key encryption to common (#62) - ([68535a6](https://github.com/mist-id/mist/commit/68535a6cdb9d9489b9f123dd264c77a448ab4553)) - Danny
- **(common)** set default values for env vars (#49) - ([fded319](https://github.com/mist-id/mist/commit/fded31934734ab24c47b34f3d6bf58e593b2d4fd)) - Danny
- **(deps)** update rust crate tower to v0.5.1 (#59) - ([730018c](https://github.com/mist-id/mist/commit/730018c40ee7fab4d9c7088c0f3a5adbe98bf522)) - renovate[bot]
- get release things set up (#84) - ([e46747a](https://github.com/mist-id/mist/commit/e46747a7f3c401345742859312ebfc38ab36d0d3)) - Danny
- move things around (#72) - ([73c2f01](https://github.com/mist-id/mist/commit/73c2f01203176110aef3d7fedfc64cb37ea056ca)) - Danny
- remove duplicate decrypt func (#67) - ([a932bd4](https://github.com/mist-id/mist/commit/a932bd4e3d54a22b1d30526ae39460d91c84098c)) - Danny
- move service creation to transaction (#58) - ([454efbc](https://github.com/mist-id/mist/commit/454efbc5b75687421de22d24c95ccfbba2983087)) - Danny
- switch to locally runnning instance of uniresolver (#54) - ([8280e2c](https://github.com/mist-id/mist/commit/8280e2cb663c4f3c62ec6d95c9a9ab7f5f16f306)) - Danny
- put demo online (#50) - ([46e83ce](https://github.com/mist-id/mist/commit/46e83ce030f703f5a9924d61480eb6db969eec8d)) - Danny
- switch to bon (#47) - ([5b7f00d](https://github.com/mist-id/mist/commit/5b7f00d98fe23c43fff72b0f7e3c7c6439947fa7)) - Danny
- switch to eyre (#44) - ([6ad3e88](https://github.com/mist-id/mist/commit/6ad3e888ccfdaaf0613965805bbee0242c701340)) - Danny
- use renovate (#38) - ([973fb64](https://github.com/mist-id/mist/commit/973fb64a10f60dff24c8f5830015cfbae3c27e60)) - Danny
- initial commit - ([7f62677](https://github.com/mist-id/mist/commit/7f62677dda2b84daf7148aefa4ca1a2e4624d545)) - [@its-danny](https://github.com/its-danny)
#### Refactoring
- **(authn)** use nats for events (#81) - ([9e39822](https://github.com/mist-id/mist/commit/9e39822ed7b26ee42878f931280cb985250b3a65)) - Danny
- create wrapper types for ids (#74) - ([bca0604](https://github.com/mist-id/mist/commit/bca06046c0d0eedd836cb0d15e2d1599f2cac775)) - Danny

- - -

Changelog generated by [cocogitto](https://github.com/cocogitto/cocogitto).