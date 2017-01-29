use std::io;
use std::fs::File;

use cargo::core::Package;
use cargo::CargoResult;

use license::License;
use options::Bundle;
use util;

pub fn run(root: Package, packages: Vec<Package>, variant: Bundle) -> CargoResult<()> {
    match variant {
        Bundle::Inline { file } => {
            if let Some(file) = file {
                inline(&mut File::create(file)?, root, packages)?;
            } else {
                inline(&mut io::stdout(), root, packages)?;
            }
        }
    }

    Ok(())
}

fn inline(out: &mut io::Write, root: Package, packages: Vec<Package>) -> CargoResult<()> {
    writeln!(out, "The {} package uses some third party libraries under their own license terms:", root.name())?;
    writeln!(out, "")?;

    for (license, packages) in util::packages_by_license(packages) {
        for package in packages {
            writeln!(out, " * {} - {}", package.name(), license)?;
        }
        match license {
            License::Multiple(licenses) => {
                let mut licenses = licenses.into_iter();
                write_license_text(out, licenses.next().unwrap())?;
                for license in licenses {
                    writeln!(out, " ---")?;
                    write_license_text(out, license)?;
                }
            }
            License::Unspecified => {
            }
            license => {
                write_license_text(out, license)?;
            }
        }
        writeln!(out, "")?;
    }

    Ok(())
}

fn write_license_text(out: &mut io::Write, license: License) -> CargoResult<()> {
    for line in license.full_text()?.lines() {
        writeln!(out, "    {}", line)?;
    }
    Ok(())
}
