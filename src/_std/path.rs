use std::path::*;

impl_just!(StripPrefixError, Path::new("").strip_prefix("a").unwrap_err());

// TODO: Figure out PathBuf and then Box/Rc/Box<Path>.