use std::fmt;

use super::host_builder::HostConfigBuilder;

#[derive(Debug)]
#[allow(dead_code)]
pub struct HostConfig {
    /// The name of SSH host
    name: String,

    /// SSH user
    user: String,

    /// SSH hostname
    hostname: String,

    /// Identity file
    identity_file: Option<std::path::PathBuf>,
}

impl fmt::Display for HostConfig {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "Host {}\n  Hostname {}\n  User {}\n  {}",
            self.name,
            self.hostname,
            self.user,
            match self.identity_file.clone() {
                Some(file) => format!("Identity file {}\n", file.display()),
                None => "".to_owned(),
            },
        )
    }
}

#[allow(dead_code)]
impl HostConfig {
    pub fn builder() -> HostConfigBuilder {
        HostConfigBuilder::default()
    }

    pub fn new(
        name: String,
        user: String,
        hostname: String,
        identity_file: Option<std::path::PathBuf>,
    ) -> Self {
        Self {
            name,
            user,
            hostname,
            identity_file,
        }
    }

    pub fn get_name(&self) -> String {
        self.name.to_string()
    }

    pub fn get_user(&self) -> String {
        self.user.to_string()
    }

    pub fn get_hostname(&self) -> String {
        self.hostname.to_string()
    }

    pub fn get_identity_file(&self) -> Option<std::path::PathBuf> {
        self.identity_file.clone()
    }
}
