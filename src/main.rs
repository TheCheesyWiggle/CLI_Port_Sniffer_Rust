use std::env;
use std::io::{self, Write};
use std::net::{IpAddr, TcpStream};
use std::str::FromStr;
use std::process;
use std::sync::mpsc::{Sender, channel};
use std::thread;
use std::fmt::rt::v1::Argument;

//creates constant og max amount of threads

const MAX: U16= 65535;

//creates a datatype called arguments
struct Arguments{
    flag: String,
    ipaddr: IpAddr,
    threads: u16,
}

//creates a implementation of the arguments type
impl Arguments{
    //takes arguments          returns argument struct or error message as string
    fn new(args: &[String]) -> Result<Arguments, &'static str> {
        //if statement checks to see if the user has passed the correct amount of arguments
        if args.len()<2 {
            return Err("not enough arguments");
        }
        else if args.len()>4 {
            return Err("too many arguments");
        }
        //clones ipaddress from args vector
        let f = args[1].clone();
        //turns string into ip address
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
            //Error handling when -h or -help has more than 2 arguments
            else if (flag.contains("-h") || flag.contains("-help")){
                return Err("too many arguments");
            }
            //checks if arguments contain -j
            else if flag.contains("-j") {
                //initializes ip address
                let ipaddr = match IpAddr::from_str(&args[3]) {
                    Ok(s) => s,
                    Err(_) => return Err("Invalid IPADDR; must be IPv6 or IPv4")
                };
                //initializes number of threads
                let threads = match args[2].parse::<u16>() {
                    Ok(s) => s,
                    Er(_) => Err("failed to parse thread number")
                };
                //returns struct
                return Ok(Arguments {threads, flag, ipaddr});
            }
            else {
                return Err("invalid syntax");
            }
        }
    }
}

fn scan (){

}

fn main() {
    //collects args passed through the program and stores them as a vector of strings
    let args: Vec<String> = env::args().collect();
    let program = args[0].clone();
    let arguments = Arguments::new(&args).unwrap_or_else(
        //error handling
      |err|{
          if err.contains("help"){
              process::exit(0);
          }
          else {
              eprintln!("{} problem parsing arguments: {}",program, err);
              process::exit(0);
          }
      }
    );
    //sets number of threads
    let num_threads = arguments.threads;
    //creates channel
    let(tx,rx)= channel();
    //iterates for the amount of threads specified
    for i in 0..num_threads{
        let tx = tx.clone();
        //creates a thread
        thread::spawn(move||{
            //calls scan function
            scan(tx, i, arguments.ipaddr, num_threads);
        });

    }
}
