use std::io::{self, Write};
use std::sync::mpsc::{Sender, channel};
use std::thread;
use std::{env, net::IpAddr, net::TcpStream, process, str::FromStr};

const MAX_PORT: u16 = 65535;

struct Params {
    ip: IpAddr,
    threads: u16,
}

impl Params {
    fn build(args: &[String]) -> Result<Params, &'static str> {
        if args.len() <= 1 || args.len() > 4 {
            return Err("Invalid arguments");
        }

        let flag = args[1].clone();

        if let Ok(ipaddr) = IpAddr::from_str(&flag) {
            return Ok(Params {
                ip: ipaddr,
                threads: 4,
            });
        }

        if flag.contains("-h") || flag.contains("-help") {
            println!("Help text");
            return Err("");
        }

        if flag.contains("-j") {
            if args.len() < 4 {
                return Err("Invalid arguments");
            }

            let ipaddr = match IpAddr::from_str(&args[3]) {
                Ok(s) => s,
                Err(_) => return Err("Invalid ip format"),
            };

            let threads: u16 = args[2].clone().parse().expect("Error in threads count");

            return Ok(Params {
                ip: ipaddr,
                threads,
            });
        }

        Err("Invalid syntax")
    }
}

fn main() {
    let args: Vec<_> = env::args().collect();
    let params = Params::build(&args).unwrap_or_else(|err| {
        println!("{err}");
        process::exit(0);
    });

    let num_threads = params.threads;
    let addr = params.ip;
    let (tx, rx) = channel();

    for i in 0..num_threads {
        let tx = tx.clone();

        thread::spawn(move || scan(tx, i, addr, num_threads));
    }

    let mut out: Vec<u16> = vec![];
    drop(tx);

    for p in rx {
        out.push(p);
    }

    out.sort();

    if out.len() < 1 {
        println!("No open ports found");
    } else {
        for prt in out {
            println!("{} port is open", prt);
        }
    }
}

fn scan(tx: Sender<u16>, start_port: u16, addr: IpAddr, num_threads: u16) {
    let mut port: u16 = start_port + 1;
    loop {
        match TcpStream::connect((addr, port)) {
            Ok(_) => {
                //println!("{port} is open");
                print!(".");
                io::stdout().flush().unwrap();
                tx.send(port).unwrap();
            }
            Err(_) => {}
        }

        if (MAX_PORT - port) <= num_threads {
            break;
        }
        port += num_threads;
    }
}
