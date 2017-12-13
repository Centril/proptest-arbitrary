use std::path::*;

// TODO: Figure out PathBuf and then Box/Rc/Box<Path>.

arbitrary!(StripPrefixError; Path::new("").strip_prefix("a").unwrap_err());