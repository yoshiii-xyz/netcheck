use anyhow::Result;
use clap::Parser;
use std::net::{SocketAddr, TcpStream, ToSocketAddrs};
use std::time::Duration;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// Host to check
    host: String,
    /// Port to check
    port: u16,
    /// Timeout in seconds
    #[arg(short, long, default_value_t = 3)]
    timeout: u64,
}

fn main() -> Result<()> {
    let args = Args::parse();
    let addr: SocketAddr = format!("{}:{}", args.host, args.port)
        .to_socket_addrs()?
        .next()
        .ok_or_else(|| anyhow::anyhow!("Could not resolve host"))?;

    let timeout = Duration::from_secs(args.timeout);
    let start = std::time::Instant::now();

    match TcpStream::connect_timeout(&addr, timeout) {
        Ok(_) => {
            let elapsed = start.elapsed();
            println!("OK {}:{}. Connected in {:?}", args.host, args.port, elapsed);
            Ok(())
        }
        Err(e) => Err(anyhow::anyhow!("Failed to connect: {}", e)),
    }
}
