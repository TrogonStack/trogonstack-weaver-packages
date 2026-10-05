# Changelog

## [0.1.1](https://github.com/TrogonStack/trogonstack-weaver-packages/compare/v0.1.0...v0.1.1) (2026-10-05)


### Features

* Keep Rust telemetry aligned with registry conventions ([#40](https://github.com/TrogonStack/trogonstack-weaver-packages/issues/40)) ([aa15622](https://github.com/TrogonStack/trogonstack-weaver-packages/commit/aa15622facb39e52cf439beda06b6e58b78bf6d4))

## 0.1.0 (2026-10-01)


### Features

* Add go codegen template package ([#6](https://github.com/TrogonStack/trogonstack-weaver-packages/issues/6)) ([9202a6b](https://github.com/TrogonStack/trogonstack-weaver-packages/commit/9202a6bcf88993a3ef0933047a2acf919dceb209))
* Add go_codegen policy package ([#5](https://github.com/TrogonStack/trogonstack-weaver-packages/issues/5)) ([25e29cf](https://github.com/TrogonStack/trogonstack-weaver-packages/commit/25e29cf0dedf8333e2ce2d193aa7c498aaff38c7))
* **go:** Allow registries to describe their root package ([#10](https://github.com/TrogonStack/trogonstack-weaver-packages/issues/10)) ([2cbb7fb](https://github.com/TrogonStack/trogonstack-weaver-packages/commit/2cbb7fb3947d5087e8168fada14c8f80208bf7eb))
* **go:** Default histogram boundaries by unit ([#34](https://github.com/TrogonStack/trogonstack-weaver-packages/issues/34)) ([e337004](https://github.com/TrogonStack/trogonstack-weaver-packages/commit/e337004faea16b27985dce3658749f94373e8eb8))
* **go:** Document requirement levels on the generated options ([#20](https://github.com/TrogonStack/trogonstack-weaver-packages/issues/20)) ([a0c2502](https://github.com/TrogonStack/trogonstack-weaver-packages/commit/a0c2502fe375ec1111b28cd6ea728d6424976f8c))
* **go:** Generate a logger that carries the schema URL ([#28](https://github.com/TrogonStack/trogonstack-weaver-packages/issues/28)) ([f217f15](https://github.com/TrogonStack/trogonstack-weaver-packages/commit/f217f15991cf2009446473b2e832f894bc0fbc8e))
* **go:** Generate a tracer that carries the schema URL ([#27](https://github.com/TrogonStack/trogonstack-weaver-packages/issues/27)) ([dc351ab](https://github.com/TrogonStack/trogonstack-weaver-packages/commit/dc351ab297e847b45663cb2626137a76e152c590))
* **go:** Generate an instrument for each metric refinement ([#21](https://github.com/TrogonStack/trogonstack-weaver-packages/issues/21)) ([170ed7c](https://github.com/TrogonStack/trogonstack-weaver-packages/commit/170ed7c124b3cdc4b23e73594100641b5ab53c30))
* **go:** Generate events that have no namespace ([#33](https://github.com/TrogonStack/trogonstack-weaver-packages/issues/33)) ([accb05f](https://github.com/TrogonStack/trogonstack-weaver-packages/commit/accb05fefbeaf60314c6ea7c58bb6a3845628c8e))
* **go:** Generate observable forms of counters and gauges ([#22](https://github.com/TrogonStack/trogonstack-weaver-packages/issues/22)) ([c13f78d](https://github.com/TrogonStack/trogonstack-weaver-packages/commit/c13f78d015f00eb6bc8c8b26399eb5bc08b88763))
* **go:** Generate typed entities ([#38](https://github.com/TrogonStack/trogonstack-weaver-packages/issues/38)) ([f44dfce](https://github.com/TrogonStack/trogonstack-weaver-packages/commit/f44dfce2b0662f07b210a9536ee5e47a9e14c2da))
* **go:** Generate typed log events ([#31](https://github.com/TrogonStack/trogonstack-weaver-packages/issues/31)) ([36f5052](https://github.com/TrogonStack/trogonstack-weaver-packages/commit/36f5052e3cffb01190bd32366645a13262dac444))
* **go:** Generate typed span starters ([#30](https://github.com/TrogonStack/trogonstack-weaver-packages/issues/30)) ([7e00cc7](https://github.com/TrogonStack/trogonstack-weaver-packages/commit/7e00cc7a0fece05e3b050b1a6ca5f5a724b217f6))
* **go:** Let histograms declare their bucket boundaries ([#15](https://github.com/TrogonStack/trogonstack-weaver-packages/issues/15)) ([1e42467](https://github.com/TrogonStack/trogonstack-weaver-packages/commit/1e42467fc24f547ac822df2933dc7a12c179ffd4))
* **go:** Move the meter into its own package ([#26](https://github.com/TrogonStack/trogonstack-weaver-packages/issues/26)) ([1ac7644](https://github.com/TrogonStack/trogonstack-weaver-packages/commit/1ac76447d1b975aa464a01ddcdaa21e7de197046))
* **go:** Name the root package after its import path ([#25](https://github.com/TrogonStack/trogonstack-weaver-packages/issues/25)) ([4087917](https://github.com/TrogonStack/trogonstack-weaver-packages/commit/408791767ba00673e6cf593502de744470f349c6))
* **go:** Read the value type from the upstream code generation annotation ([#18](https://github.com/TrogonStack/trogonstack-weaver-packages/issues/18)) ([84f72a3](https://github.com/TrogonStack/trogonstack-weaver-packages/commit/84f72a3780d5d83a314c6b6de7334508cb11bda3))
* **go:** Require a versioned schema URL ([#16](https://github.com/TrogonStack/trogonstack-weaver-packages/issues/16)) ([fb133f4](https://github.com/TrogonStack/trogonstack-weaver-packages/commit/fb133f460f6892c47ba8f91b405d4338ed5298a0))
* **go:** Tie generated instruments to the registry schema URL ([#12](https://github.com/TrogonStack/trogonstack-weaver-packages/issues/12)) ([342fa99](https://github.com/TrogonStack/trogonstack-weaver-packages/commit/342fa99f0d3bc10923da777e3666e7f7ad4c106f))
* **go:** Type attributes imported from generated dependencies ([#35](https://github.com/TrogonStack/trogonstack-weaver-packages/issues/35)) ([15db8a2](https://github.com/TrogonStack/trogonstack-weaver-packages/commit/15db8a2344efcd8f6190495c8884c0c6ccc13dc5))


### Bug Fixes

* **go:** Fail generation when filtering drops a required attribute ([#14](https://github.com/TrogonStack/trogonstack-weaver-packages/issues/14)) ([39ecfce](https://github.com/TrogonStack/trogonstack-weaver-packages/commit/39ecfcece2272a7188bf6494f68e371c42370b41))
* **go:** Render every enum member type the registry allows ([#19](https://github.com/TrogonStack/trogonstack-weaver-packages/issues/19)) ([350accc](https://github.com/TrogonStack/trogonstack-weaver-packages/commit/350acccb6029268cc93bdd488cc3d80c309232bc))
* **go:** Support metrics that reference imported attributes ([#11](https://github.com/TrogonStack/trogonstack-weaver-packages/issues/11)) ([043e750](https://github.com/TrogonStack/trogonstack-weaver-packages/commit/043e75055019c8f3c128e619dbff66da48c8fce9))
