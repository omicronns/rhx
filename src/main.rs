extern crate getopts;
use getopts::Options;
use std::env;
use std::io::Read;

fn print_usage(program: &str, opts: &Options) {
    let brief = format!("Usage: {} [options]", program);
    print!("{}", opts.usage(&brief));
    println!("");
}

fn parse_num<T>(s: Option<String>) -> Option<T>
    where
    T: std::str::FromStr,
    T::Err: std::fmt::Display {
    match s {
        Some(n) => Some(match n.parse::<T>() {
            Ok(n) => n,
            Err(e) => panic!("{}", e)
        }),
        None => None
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let program = &args[0];

    let mut opts = Options::new();
    opts.optopt("c", "columns", "number of columns (default=1)", "COLS")
        .optopt("g", "group", "number of octets per column (default=inf)", "OCTETS")
        .optopt("o", "offset", "start after skipping OFFSET octets", "OFFSET")
        .optopt("l", "length", "stop after LENGTH octets", "LENGTH")
        .optopt("p", "prefix", "prefix to add before each octet", "PREFIX")
        .optopt("a", "postfix", "postfix to add after each octet", "POSTFIX")
        .optflag("h", "help", "print this help menu");

    let matches = match opts.parse(&args[1..]) {
        Ok(m) => m,
        Err(f) => panic!(f.to_string())
    };

    if matches.opt_present("h") {
        print_usage(program, &opts);
        return;
    }

    let columns = match parse_num::<usize>(matches.opt_str("c")) {
        Some(c) => c,
        None => 1
    };
    let group = parse_num::<usize>(matches.opt_str("g"));
    let offset = match parse_num::<usize>(matches.opt_str("o")) {
        Some(o) => o,
        None => 0
    };
    let length = parse_num::<usize>(matches.opt_str("l"));
    let prefix = match matches.opt_str("p") {
        Some(p) => p,
        None => String::new()
    };
    let postfix = match matches.opt_str("a") {
        Some(a) => a,
        None => String::new()
    };
    
    let mut input = Vec::new();

    match std::io::stdin().read_to_end(&mut input) {
        Ok(len) => {
            match group {
                Some(group) => {
                    let length = match length {
                        Some(l) => std::cmp::min(l, len),
                        None => len,
                    };
                    let mut iter: usize = offset;
                    loop {
                        let end = std::cmp::min(iter + group, length);
                        for i in iter..end {
                            print!("{}{:02x}{}", prefix, input[i], postfix);
                        }
                        if end == length {
                            println!("");
                            break;
                        }
                        if end % (group * columns) == 0 {
                            println!("");
                        } else {
                            print!(" ");
                        }
                        iter += group;
                    }
                },
                None => {
                    let length = match length {
                        Some(l) => std::cmp::min(l, len),
                        None => len,
                    };
                    let mut iter: usize = offset;
                    for i in iter..length {
                        print!("{}{:02x}{}", prefix, input[i], postfix);
                    }
                    println!("");
                }
            }
        },
        Err(e) => {
            panic!("{}", e);
        },
    }
}
