use std::env;
use std::io::{self, Write};
use std::net::{IpAddr, TcpStream};
use std::str::FromStr;
use std::process;
use std::sync::mpsc::{Sender, channel};
use std::thread;

//creates a datatype called arguments
struct Arguments{
    flag: String,
    ipaddr: IpAddr,
    threads: u16,
}

//creates a implementation of the arguments type
impl Arguments{
    fn new(args: &[String]) -> Result<Arguments, &'static str> {
        //if statement checks to see if the user has passed the correct amount of arguments
        if args.len()<2 {
            return Err("not enough arguments");
        }
        else if args.len()>4 {
            return Err("too many arguments");
        }
        //
        let f = args[1].clone();

        if let Ok(ipaddr) = IpAddr::from_str(&f){
            return Ok(Arguments {flag: String::from(""), ipaddr, threads:4});
        }
        else{
            //
            let flag = args[1].clone();
            //check whether the args type is valid or contains the help argument
            if flag.contains("-h") || flag.contains("-help")&& args.len==2{
                //displays help message
                println!("Usage: -j to select the number of threads you want to use
                \n\r -h or -help to show this help message");
                return Err("help");
            }
            //
            else if (flag.contains("-h") || flag.contains("-help")){
                return Err("too many arguments");
            }
            //
            else if flag.contains("-j") {
                let ipaddr = match IpAddr::from_str(&args[3]) {
                    Ok(s) => s,
                    Err(_) => return Err("Invalid IPADDR; must be IPv6 or IPv4")
                };
                let threads = match args[2].parse::<u16>() {
                    Ok(s) => s,
                    Er(_) => Err("failed to parse thread number")
                };
                return Ok(Arguments {threads, flag, ipaddr});
            }
            else {
                return Err("invalid syntax");
            }
        }
    }
}


fn main() {
    let args: Vec<String> = env::args().collect();
    let program = args[0].clone();
}
