use std::collections::HashSet;

use cargo::core::dependency::Kind;
use cargo::core::registry::PackageRegistry;
use cargo::core::{ Package, Workspace };
use cargo::ops;
use cargo::util::important_paths::find_root_manifest_for_wd;
use cargo::{ Config, CargoResult };

pub fn resolve_packages(
        manifest_path: Option<String>,
        config: &Config) -> CargoResult<(Package, Vec<Package>)> {
    let root = find_root_manifest_for_wd(manifest_path, config.cwd())?;
    let workspace = Workspace::new(&root, config)?;
    let current = workspace.current()?;
    let mut registry = PackageRegistry::new(config)?;
    registry.add_sources(&[current.package_id().source_id().clone()])?;
    let resolve = ops::resolve_ws(&mut registry, &workspace)?;

    let packages = ops::get_resolved_packages(&resolve, registry);

    let mut result = HashSet::new();
    let mut to_check = vec![current.package_id()];
    while let Some(id) = to_check.pop() {
        if let Ok(package) = packages.get(id) {
            if result.insert(package) {
                let deps = resolve.deps(id);
                for dep_id in deps {
                    let dep = package.dependencies().iter()
                        .find(|d| d.matches_id(dep_id))
                        .expect("The dependency tree should be fully resolved");
                    if let Kind::Normal = dep.kind() {
                        to_check.push(dep_id);
                    }
                }
            }
        }
    }

    Ok((current.clone(), result.into_iter().cloned().collect()))
}
