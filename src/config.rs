#[derive(serde::Deserialize, Debug)]
pub struct Server {
    pub strategy: LbStrategy,
    pub listener: Listener,
    pub upstreams: Vec<Upstream>,
}

#[derive(serde::Deserialize, Debug)]
#[serde(rename_all = "snake_case")]
pub enum LbStrategy {
    RoundRobin,
    LeastConnection,
}

#[derive(serde::Deserialize, Debug)]
pub struct Upstream {
    address: String,
    port: u16,
}

impl crate::pool::ToSock for Upstream {
    fn to_sock(&self) -> std::net::SocketAddr {
        self.connection_str().parse().unwrap()
    }
}

impl Upstream {
    pub fn connection_str(&self) -> String {
        format!("{}:{}", self.address, self.port)
    }
}

#[derive(serde::Deserialize, Debug)]
pub struct Listener {
    address: String,
    port: u16,
}

impl Listener {
    pub fn connection_str(&self) -> String {
        format!("{}:{}", self.address, self.port)
    }
}

#[derive(serde::Deserialize, Debug)]
pub struct Config {
    pub server: Server,
}

impl Default for Config {
    fn default() -> Self {
        Self::new()
    }
}

impl Config {
    pub fn new() -> Self {
        let path = "/usr/local/bin/config.yml";
        //let path = "config.yml";
        let f = std::fs::File::open(path).expect("Could not open file.");
        serde_yaml::from_reader::<_, Config>(f).expect("Could not read values.")
    }
}
