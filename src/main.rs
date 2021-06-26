use std::env;
use std::io::{self, Write};
use std::net::{IpAddr, TcpStream};
use std::process;
use std::str::FromStr;
use std::sync::mpsc::{channel, Sender};
use std::thread;

const MAX_PORT: u16 = 65535;

struct Arguments {
    #[allow(dead_code)]
    flag: String,
    ipaddr: IpAddr,
    threads: u16,
}

impl Arguments {
    pub fn new(args: &Vec<String>) -> Result<Arguments, &'static str> {
        if args.len() < 2 {
            return Err("Not Enough Arguments");
        } else if args.len() > 4 {
            return Err("Too Many Arguments");
        }

        let f = args[1].clone();
        if let Ok(ipaddr) = IpAddr::from_str(&f) {
            return Ok(Arguments {
                flag: String::from(""),
                ipaddr,
                threads: 4,
            });
        } else {
            let flag = args[1].clone();

            if flag.contains("-h") || flag.contains("--help") && args.len() == 2 {
                println!(
                    "Usage:\n\t-j\t Select the amount of threads to use\n\t-h\tShow help message"
                );
                return Err("help");
            } else if flag.contains("-h") || flag.contains("--help") {
                return Err("Too Many Arguments");
            } else if flag.contains("-j") {
                let ipaddr = match IpAddr::from_str(&args[3]) {
                    Ok(s) => s,
                    Err(_) => return Err("Invalid IP Address; Use IPV4 or IPV6"),
                };

                let threads = match args[2].parse::<u16>() {
                    Ok(s) => s,
                    Err(_) => return Err("Invalid Thread Count number"),
                };

                return Ok(Arguments {
                    ipaddr,
                    threads,
                    flag,
                });
            } else {
                return Err("Invalid Syntax");
            }
        }
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();

    let program = args[0].clone();

    let arguments = Arguments::new(&args).unwrap_or_else(|err| {
        if err.contains("help") {
            process::exit(0);
        } else {
            eprintln!("{} problem parsing arguments {}", program, err);
            process::exit(0);
        }
    });

    let num_threads = arguments.threads;
    let addr = arguments.ipaddr;
    let (tx, rx) = channel();

    for i in 0..num_threads {
        let tx = tx.clone();

        thread::spawn(move || {
            scan(tx, i, addr, num_threads);
        });
    }

    let mut out = vec![];
    drop(tx);

    for p in rx {
        out.push(p);
    }

    println!("");

    out.sort();

    for v in out {
        println!("{} is open", v);
    }
}

fn scan(tx: Sender<u16>, start_port: u16, addr: IpAddr, num_threads: u16) {
    let mut port = start_port + 1;
    loop {
        match TcpStream::connect((addr, port)) {
            Ok(_) => {
                print!(".");
                io::stdout().flush().unwrap();
                tx.send(port).unwrap();
            }

            Err(_) => {}
        };

        if (MAX_PORT - port) >= num_threads {
            break;
        }

        port += num_threads;
    }
}
