use std::{
    io::{self, Write},
    net::{TcpListener, TcpStream},
};

#[derive(Debug, Clone)]
pub struct IPV4 {
    address: String,
}
impl IPV4 {
    fn address(&self, port: String) -> String {
        format!("{}:{}", self.address, port)
    }
    fn new(address: String) -> IPV4 {
        IPV4 { address }
    }
}
pub struct Client {
    host_ip: IPV4,
    port: String,
}
impl Client {
    pub fn new(host_ip: IPV4, port: String) -> Client {
        Client { host_ip, port }
    }
    pub fn address(&self) -> String {
        self.host_ip.address(self.port.clone())
    }
    /// Client starts, forms a connection and then returns an iterator over the response.
    pub fn start_gen(&self) -> Result<(), io::Error> {
        let address = self.address();
        println!("Connecting to {}", address);
        let mut stream = TcpStream::connect(&address)?;
        stream.write_all("start".as_bytes())?;
        Ok(())
    }
}

#[cfg(test)]
mod client_test {
    use super::*;
    use dotenv::dotenv;
    use std::{
        env::var,
        io::{BufRead, BufReader},
    };
    #[test]
    fn server_test() {
        dotenv().ok();
        let ip = IPV4::new(var("IPV4").expect("Can't find .evn variable IPV4"));
        let port = var("PORT").expect("Can't find .evn variable IPV4");
        let client = Client::new(ip, port);
        let listner = client.start_gen().expect("Failed to connect to host");
    }
}
