use std::borrow::Cow;
use std::fmt;
use std::path::PathBuf;
use std::str::FromStr;

use cargo::{ human, CargoResult };
use void::Void;

#[derive(Eq, PartialEq, Hash, Ord, PartialOrd, Debug)]
#[allow(non_camel_case_types)]
pub enum License {
    MIT,
    X11,
    BSD_3_Clause,
    Apache_2_0,
    LGPL_2_0,
    LGPL_2_1,
    LGPL_2_1Plus,
    LGPL_3_0,
    LGPL_3_0Plus,
    MPL_1_1,
    MPL_2_0,
    GPL_2_0,
    GPL_2_0Plus,
    GPL_3_0,
    GPL_3_0Plus,
    AGPL_1_0,
    Custom(String),
    File(PathBuf),
    Multiple(Vec<License>),
    Unspecified,
}

impl Default for License {
    fn default() -> License {
        License::Unspecified
    }
}

macro_rules! compatibility {
  ($s:expr, $o:expr, { $($a:pat => [$($b:pat),+])+ }) => {
    #[allow(single_match)]
    match $s {
      $(
        $a => match $o {
          $($b)|+ => return Some(true),
          _ => (),
        }
      ),*
    }
  };
}

impl License {
    pub fn can_include(&self, other: &License) -> Option<bool> {
        use self::License::*;

        if let Unspecified = *other { return Some(false); }

        if let Custom(_) = *self { return None; }
        if let Custom(_) = *other { return None; }
        if let File(_) = *self { return None; }
        if let File(_) = *other { return None; }

        if let Multiple(ref licenses) = *self {
            for license in licenses {
                if let Some(can_include) = license.can_include(other) {
                    if !can_include {
                        return Some(false);
                    }
                } else {
                    return None;
                }
            }
            return Some(true);
        }

        if let Multiple(ref licenses) = *other {
            let mut seen_none = false;
            for license in licenses {
                if let Some(can_include) = self.can_include(license) {
                    if can_include {
                        return Some(true);
                    }
                } else {
                    seen_none = true;
                }
            }
            return if seen_none { None } else { Some(false) };
        }

        if let LGPL_2_0 = *self { return None; /* TODO: unknown */ }
        if let LGPL_2_0 = *other { return None; /* TODO: unknown */ }

        compatibility!(*self, *other, {
            Unspecified         => [MIT, X11, BSD_3_Clause]

            LGPL_2_0     => [LGPL_2_0] // TODO: probably allows more

            MIT          => [MIT, X11]
            X11          => [MIT, X11]
            BSD_3_Clause => [MIT, X11, BSD_3_Clause]
            Apache_2_0   => [MIT, X11, BSD_3_Clause, Apache_2_0]
            MPL_1_1      => [MIT, X11, BSD_3_Clause, MPL_1_1]
            MPL_2_0      => [MIT, X11, BSD_3_Clause, MPL_2_0]
            LGPL_2_1Plus => [MIT, X11, BSD_3_Clause, MPL_2_0, LGPL_2_1Plus]
            LGPL_2_1     => [MIT, X11, BSD_3_Clause, MPL_2_0, LGPL_2_1Plus, LGPL_2_1]
            LGPL_3_0Plus => [MIT, X11, BSD_3_Clause, MPL_2_0, Apache_2_0, LGPL_2_1Plus, LGPL_3_0Plus]
            LGPL_3_0     => [MIT, X11, BSD_3_Clause, MPL_2_0, Apache_2_0, LGPL_2_1Plus, LGPL_3_0Plus, LGPL_3_0]
            GPL_2_0Plus  => [MIT, X11, BSD_3_Clause, MPL_2_0, LGPL_2_1Plus, LGPL_2_1, GPL_2_0Plus]
            GPL_2_0      => [MIT, X11, BSD_3_Clause, MPL_2_0, LGPL_2_1Plus, LGPL_2_1, GPL_2_0Plus, GPL_2_0]
            GPL_3_0Plus  => [MIT, X11, BSD_3_Clause, MPL_2_0, Apache_2_0, LGPL_2_1Plus, LGPL_2_1, GPL_2_0Plus, GPL_3_0Plus]
            GPL_3_0      => [MIT, X11, BSD_3_Clause, MPL_2_0, Apache_2_0, LGPL_2_1Plus, LGPL_2_1, GPL_2_0Plus, GPL_3_0Plus, GPL_3_0]
            AGPL_1_0     => [MIT, X11, BSD_3_Clause, MPL_2_0, Apache_2_0, LGPL_2_1Plus, LGPL_2_1, GPL_2_0Plus, GPL_3_0Plus, GPL_3_0, AGPL_1_0]

            // TODO: These are `unreachable!()`, can't figure out a nice way to allow this in the macro...
            Custom(_)    => [MIT]
            File(_)      => [MIT]
            Multiple(_)  => [MIT]
        });

        Some(false)
    }

    pub fn full_text(&self) -> CargoResult<Cow<'static, str>> {
        let not_supported = Err(human(format!(" \
            Bundling license {} is not supported yet. \
            If you feel like adding support you simply need to add a text file \
            and update the code to reference it for this license.
            ", self)));
        Ok(match *self {
            License::MIT           => include_str!("licenses/MIT").into(),
            License::X11           => { return not_supported }
            License::BSD_3_Clause  => { return not_supported }
            License::Apache_2_0    => include_str!("licenses/Apache-2.0").into(),
            License::LGPL_2_0      => { return not_supported }
            License::LGPL_2_1      => { return not_supported }
            License::LGPL_2_1Plus  => { return not_supported }
            License::LGPL_3_0      => { return not_supported }
            License::LGPL_3_0Plus  => { return not_supported }
            License::MPL_1_1       => { return not_supported }
            License::MPL_2_0       => { return not_supported }
            License::GPL_2_0       => { return not_supported }
            License::GPL_2_0Plus   => { return not_supported }
            License::GPL_3_0       => { return not_supported }
            License::GPL_3_0Plus   => { return not_supported }
            License::AGPL_1_0      => { return not_supported }
            License::Custom(_)     => {
                return Err(human(format!(" \
                    Bundling license {} is not supported, this is not a known \
                    OSI license (or my list of licenses is out of date) \
                    ", self)));
            }
            License::File(_)       => { return not_supported }
            License::Multiple(_)   => { return Err(human("Bundling multiple licenses happens at a higher level")) }
            License::Unspecified   => { return Err(human("Bundling unspecified license happens at a higher level")) }
        })
    }
}

impl FromStr for License {
    type Err = Void;
    fn from_str(s: &str) -> Result<License, Void> {
        Ok(match s.trim() {
            "MIT"                => License::MIT,
            "X11"                => License::X11,
            "BSD-3-Clause"       => License::BSD_3_Clause,
            "Apache-2.0"         => License::Apache_2_0,
            "LGPL-2.0"           => License::LGPL_2_0,
            "LGPL-2.1"           => License::LGPL_2_1,
            "LGPL-2.1+"          => License::LGPL_2_1Plus,
            "LGPL-3.0"           => License::LGPL_3_0,
            "LGPL-3.0+"          => License::LGPL_3_0Plus,
            "MPL-1.1"            => License::MPL_1_1,
            "MPL-2.0"            => License::MPL_2_0,
            "GPL-2.0"            => License::GPL_2_0,
            "GPL-2.0+"           => License::GPL_2_0Plus,
            "GPL-3.0"            => License::GPL_3_0,
            "GPL-3.0+"           => License::GPL_3_0Plus,
            "AGPL-1.0"           => License::AGPL_1_0,
            s if s.contains('/') => {
                let mut licenses = s.split('/')
                    .map(str::parse)
                    .map(Result::unwrap)
                    .collect::<Vec<License>>();
                licenses.sort();
                License::Multiple(licenses)
            },
            s => License::Custom(s.to_owned()),
        })
    }
}

impl fmt::Display for License {
    fn fmt(&self, w: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            License::MIT           => write!(w, "MIT"),
            License::X11           => write!(w, "X11"),
            License::BSD_3_Clause  => write!(w, "BSD-3-Clause"),
            License::Apache_2_0    => write!(w, "Apache-2.0"),
            License::LGPL_2_0      => write!(w, "LGPL-2.0"),
            License::LGPL_2_1      => write!(w, "LGPL-2.1"),
            License::LGPL_2_1Plus  => write!(w, "LGPL-2.1+"),
            License::LGPL_3_0      => write!(w, "LGPL-3.0"),
            License::LGPL_3_0Plus  => write!(w, "LGPL-3.0+"),
            License::MPL_1_1       => write!(w, "MPL-1.1"),
            License::MPL_2_0       => write!(w, "MPL-2.0"),
            License::GPL_2_0       => write!(w, "GPL-2.0"),
            License::GPL_2_0Plus   => write!(w, "GPL-2.0+"),
            License::GPL_3_0       => write!(w, "GPL-3.0"),
            License::GPL_3_0Plus   => write!(w, "GPL-3.0+"),
            License::AGPL_1_0      => write!(w, "AGPL-1.0"),
            License::Custom(ref s) => write!(w, "Custom({})", s),
            License::File(ref f)   => write!(w, "File({})", f.to_string_lossy()),
            License::Multiple(ref ls)   => {
                let mut ls = ls.iter();
                write!(w, "{}", ls.next().unwrap())?;
                for l in ls {
                    write!(w, " OR {}", l)?;
                }
                Ok(())
            },
            License::Unspecified          => write!(w, "Unlicensed"),
        }
    }
}
