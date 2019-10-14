use failure::Fail;
use std::env;
use std::ffi::OsString;

#[derive(Debug, Fail)]
pub enum EnvError {
    #[fail(display = "env variable '{}' not found", name)]
    NotPresent { name: String },

    #[fail(display = "env variable '{}' cannot be expressed in UTF-8", name)]
    NotUnicode { name: String, value: OsString },
}

pub fn var(name: &str) -> Result<String, EnvError> {
    match env::var(name) {
        Ok(value) => Ok(value),
        Err(env::VarError::NotPresent) => Err(EnvError::NotPresent {
            name: name.to_string(),
        }),
        Err(env::VarError::NotUnicode(value)) => Err(EnvError::NotUnicode {
            name: name.to_string(),
            value,
        }),
    }
}
