use serde::Deserialize;
use std::io;
use std::path::Path;

use anyhow::{Context, Result};

use crate::utils::utils;

#[derive(Clone, Debug, Deserialize, PartialEq)]
pub struct FallbackSettings {
    pub default_toolchain: Option<String>,
}

impl Default for FallbackSettings {
    fn default() -> Self {
        Self {
            default_toolchain: None,
        }
    }
}

impl FallbackSettings {
    pub(crate) fn new<P: AsRef<Path>>(path: P) -> Result<Option<Self>> {
        // Users cannot fix issues with missing/unreadable/invalid centralised files, but logging isn't setup early so
        // we can't simply trap all errors and log diagnostics. Ideally we would, and then separate these into different
        // sorts of issues, logging messages about errors that should be fixed. Instead we trap some conservative errors
        // that hopefully won't lead to too many tickets.
        match utils::read_file("fallback settings", path.as_ref()) {
            Err(e) => match e.downcast_ref::<io::Error>() {
                Some(io_err) => match io_err.kind() {
                    io::ErrorKind::NotFound | io::ErrorKind::PermissionDenied => Ok(None),
                    _ => Err(e),
                },
                None => Err(e),
            },
            Ok(file_contents) => Ok(Some(
                toml::from_str(&file_contents).context("error parsing settings")?,
            )),
        }
    }
}
