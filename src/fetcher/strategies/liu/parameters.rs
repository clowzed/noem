pub struct ImapCredentials {
    login: String,
    password: String,
}

impl ImapCredentials {
    pub fn new(login: String, password: String) -> Self {
        Self { login, password }
    }

    pub fn login(&self) -> &str {
        &self.login
    }

    pub fn password(&self) -> &str {
        &self.password
    }
}

pub struct ImapConnectionParameters {
    host: String,
    port: u16,
}

impl ImapConnectionParameters {
    pub fn new(host: String, port: u16) -> Self {
        Self { host, port }
    }

    pub fn host(&self) -> &str {
        &self.host
    }

    pub fn port(&self) -> u16 {
        self.port
    }
}

pub struct StrategyParameters {
    connection_parameters: ImapConnectionParameters,
    credentials: ImapCredentials,
    folder: String,
}

impl StrategyParameters {
    pub fn new(
        connection_parameters: ImapConnectionParameters,
        credentials: ImapCredentials,
        folder: String,
    ) -> Self {
        Self {
            connection_parameters,
            credentials,
            folder,
        }
    }

    pub fn connection_parameters(&self) -> &ImapConnectionParameters {
        &self.connection_parameters
    }

    pub fn credentials(&self) -> &ImapCredentials {
        &self.credentials
    }

    pub fn folder(&self) -> &str {
        &self.folder
    }
}
