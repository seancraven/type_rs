use std::{
    io::{self, Write},
    net::{TcpListener, TcpStream},
};

#[derive(Debug, Clone)]
struct IPV4 {
    address: String,
}
impl IPV4 {
    fn address(&self, port: u16) -> String {
        format!("{}:{}", self.address, port)
    }
}
pub struct Client {
    host_ip: IPV4,
    port: String,
}
impl Client {
    pub fn new(host_ip: IPV4, port: u16) -> Client {
        Client { host_ip, port }
    }
    pub fn address(&self) -> String {
        self.host_ip.address(self.port)
    }
    /// Client starts, forms a connection and then returns an iterator over the response.
    pub fn start_gen(&self) -> Result<TcpListener, io::Error> {
        let address = self.address();
        let mut stream = TcpStream::connect(&address)?;
        stream.write_all("start".as_bytes())?;
        let listner = TcpListener::bind(&address)?;
        return Ok(listner);
    }
}

#[cfg(test)]
mod client_test {
    #[test]
    fn server_test() {
        todo!()
    }
}
