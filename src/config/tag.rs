use crate::config::{error, Error};
use std::fmt;
use std::str::FromStr;

#[derive(Debug)]
pub struct TagFormat {
    segments: Vec<Segment>,
}

#[derive(Debug)]
enum Segment {
    Literal(String),
    Name,
    Version,
}

impl TagFormat {
    /// Formats tags as `v{version}`.
    pub fn version_only() -> TagFormat {
        use Segment::*;

        TagFormat {
            segments: vec![
                Literal("v".to_string()),
                Version,
            ],
        }
    }

    /// Formats tags as `{name}-{version}`.
    pub fn name_version() -> TagFormat {
        use Segment::*;

        TagFormat {
            segments: vec![
                Name,
                Literal("-".to_string()),
                Version,
            ],
        }
    }

    /// Set of common tag formats
    pub fn common() -> impl Iterator<Item = TagFormat> {
        vec![TagFormat::version_only(), TagFormat::name_version()].into_iter()
    }

    /// Returns `true` if the tag format includes the crate name.
    pub fn includes_name(&self) -> bool {
        use Segment::Name;

        self.segments.iter()
            .any(|segment| match segment {
                Name => true,
                _ => false,
            })
    }

    pub fn format<T: ToString>(&self, name: &str, version: &T) -> String {
        use Segment::*;

        let mut out = String::new();

        for segment in &self.segments {
            match segment {
                Literal(lit) => out.push_str(lit),
                Name => out.push_str(name),
                Version => out.push_str(&version.to_string()),
            }
        }

        out
    }
}

impl Default for TagFormat {
    fn default() -> Self {
        TagFormat::version_only()
    }
}

impl FromStr for TagFormat {
    type Err = Error;

    fn from_str(mut s: &str) -> Result<Self, Self::Err> {
        use Segment::*;

        let mut segments = vec![];

        while !s.is_empty() {
            let start_idx = match s.find("{") {
                Some(idx) => idx,
                None => {
                    segments.push(Literal(s.to_string()));
                    break;
                }
            };

            if start_idx > 0 {
                segments.push(Literal(s[..start_idx].to_string()));
            }

            s = &s[start_idx+1..];

            let end_idx = match s.find("}") {
                Some(idx) => idx,
                None => {
                    unimplemented!();
                }
            };

            match &s[..end_idx] {
                "name" => segments.push(Name),
                "version" => segments.push(Version),
                _ => unimplemented!(),
            }

            s = &s[end_idx+1..];
        }

        // Must contain a version
        let contains_version = segments.iter()
            .find(|seg| match seg {
                Version => true,
                _ => false,
            })
            .is_some();

        if !contains_version {
            return Err(error::InvalidTagFormat.into());
        }

        Ok(TagFormat { segments })
    }
}

impl fmt::Display for TagFormat {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        use Segment::*;

        for segment in &self.segments {
            match segment {
                Literal(lit) => write!(fmt, "{}", lit)?,
                Name => write!(fmt, "{{name}}")?,
                Version => write!(fmt, "{{version}}")?,
            }
        }

        Ok(())
    }
}

#[cfg(test)]
mod test {
    use super::TagFormat;
    use assertive::assert_ok;

    #[test]
    fn literal() {
        let res: Result<TagFormat, _> = "literal".parse();
        assert!(res.is_err());
    }

    #[test]
    fn version_only() {
        let f: TagFormat = assert_ok!("{version}".parse());
        assert_eq!("0.2.3", f.format("hello", &"0.2.3"));
        assert!(!f.includes_name());
    }

    #[test]
    fn prefix_version() {
        let f: TagFormat = assert_ok!("v{version}".parse());
        assert_eq!("v0.2.3", f.format("hello", &"0.2.3"));
        assert!(!f.includes_name());
    }

    #[test]
    fn suffix_version() {
        let f: TagFormat = assert_ok!("{version}-final".parse());
        assert_eq!("0.2.3-final", f.format("hello", &"0.2.3"));
        assert!(!f.includes_name());
    }

    #[test]
    fn name_version() {
        let f: TagFormat = assert_ok!("{name}-{version}".parse());
        assert_eq!("hello-0.2.3", f.format("hello", &"0.2.3"));
        assert!(f.includes_name());
    }
}
