use std::fmt;

use super::host::HostConfig;

#[derive(Default, Debug)]
pub struct SSHConfig {
    hosts: Vec<HostConfig>,
}

impl SSHConfig {
    pub fn new(hosts: Vec<HostConfig>) -> Self {
        Self { hosts }
    }
}

impl fmt::Display for SSHConfig {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        for (_index, host) in self.hosts.iter().enumerate() {
            write!(f, "{}", host)?;
        }
        Ok(())
    }
}
