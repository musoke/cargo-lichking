use std::io;
use std::fs::File;

use cargo::core::Package;
use cargo::{ human, Config, CargoResult };

use licensed::Licensed;
use options::Bundle;
use util;

pub fn run(root: Package, packages: Vec<Package>, variant: Bundle) -> CargoResult<()> {
    match variant {
        Bundle::Inline { file } => {
            if let Some(file) = file {
                inline(root, packages, File::open(file)?)?;
            } else {
                inline(root, packages, io::stdout())?;
            }
        }
    }

    Ok(())
}

fn inline<W: io::Write>(root: Package, packages: Vec<Package>, mut out: W) -> CargoResult<()> {
    let packages_by_license = util::packages_by_license(packages);

    writeln!(out, "The {} package uses some third party libraries under their own license terms:", root.name())?;
    writeln!(out, "")?;
    for (license, packages) in packages_by_license {
        for package in packages {
            writeln!(out, " * {} - {}", package.name(), license)?;
        }
        writeln!(out, "    TODO: {} license contents", license)?;
        writeln!(out, "")?;
    }
    Ok(())
}
