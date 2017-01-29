use cargo::core::Package;
use cargo::{ Config, CargoResult };

use util;

pub fn run(packages: Vec<Package>, config: &Config) -> CargoResult<()> {
    let packages_by_license = util::packages_by_license(packages);

    for (license, packages) in packages_by_license {
        let packages = packages.iter().map(|package| package.name()).collect::<Vec<&str>>().join(", ");
        config.shell().say(format!("{}: {}", license, packages), 0)?;
    }

    Ok(())
}
