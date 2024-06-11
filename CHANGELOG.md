## 0.11.1

### Added
* Introduce 'dev' feature for toggling between `kubi.zone` and `dev.kubi.zone` api groups.
* Add 'dump' example for writing out the CRDs to yaml files: `cargo run [--features dev] --example dump`

### Fixed
* Updated k8s-openapi to v0.22.0
* Updated kube-rs to v0.91.0