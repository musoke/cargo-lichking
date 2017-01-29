use std::collections::HashMap;

use cargo::core::Package;
use license::License;
use licensed::Licensed;

pub fn packages_by_license(packages: Vec<Package>) -> HashMap<License, Vec<Package>> {
    let mut packages_by_license = HashMap::new();

    for package in packages {
        packages_by_license
            .entry(package.license())
            .or_insert_with(Vec::new)
            .push(package);
    }

    packages_by_license
}
