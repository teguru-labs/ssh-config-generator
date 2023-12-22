use super::host::HostConfig;

pub trait Builder<T> {
    fn set_name(&mut self, name: String);
    fn set_user(&mut self, user: String);
    fn set_hostname(&mut self, hostname: String);
    fn set_identity_file(&mut self, identity_file: Option<std::path::PathBuf>);
    fn build(self) -> T;
}

#[derive(Default, Debug)]
pub struct HostConfigBuilder {
    /// Name of the host config
    name: Option<String>,

    /// SSH user
    user: Option<String>,

    /// SSH hostname
    hostname: Option<String>,

    /// Identity file
    identity_file: Option<std::path::PathBuf>,
}

impl Builder<HostConfig> for HostConfigBuilder {
    fn set_name(&mut self, name: String) {
        self.name = Some(name);
    }

    fn set_user(&mut self, user: String) {
        self.user = Some(user);
    }

    fn set_hostname(&mut self, hostname: String) {
        self.hostname = Some(hostname);
    }

    fn set_identity_file(&mut self, identity_file: Option<std::path::PathBuf>) {
        self.identity_file = identity_file;
    }

    fn build(self) -> HostConfig {
        let hostname = self.hostname.expect("Please, set hostname");

        HostConfig::new(
            self.name.unwrap_or(hostname.to_string()),
            self.user.expect("Please, set user"),
            hostname.to_string(),
            self.identity_file.clone(),
        )
    }
}
