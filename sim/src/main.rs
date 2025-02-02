extern crate getopts;
use getopts::Options;
use std::env;
use sim::cache_simulator::{Cache, Set, Line, operate_flags}; 

// Function to print usage information
fn print_usage(opts: &Options) {
    let brief = format!("Usage: -s <s> -E <E> -b <b> -t <tracefile>");
    println!("{}", opts.usage(&brief));
}

fn main() {
    // Parse command-line arguments
    let args: Vec<String> = env::args().collect();
    let mut opts = Options::new();
    opts.optopt("s", "", "Number of set index bits (S = 2s is the number of sets)", "");
    opts.optopt("E", "", "Associativity (number of lines per set)", "");
    opts.optopt("b", "", "Number of block bits (B = 2b is the block size)", "");
    opts.optopt("t", "", "Name of the trace to replay", "");

    let matches = match opts.parse(&args[1..]) {
        Ok(m) => m,
        Err(e) => {
            println!("Error parsing arguments: {}", e);
            print_usage(&opts);
            return;
        }
    };
    let s = match matches.opt_str("s").map(|s| s.parse::<usize>()) {
        Some(Ok(val)) if val > 0 => val,
        _ => {
            println!("Error: Missing or invalid value for option -s. It must be greater than 0.");
            print_usage(&opts);
            return;
        }
    };
    let e = match matches.opt_str("E").map(|e| e.parse::<usize>()) {
        Some(Ok(val)) if val > 0 => val,
        _ => {
            println!("Error: Missing or invalid value for option -E. It must be greater than 0.");
            print_usage(&opts);
            return;
        }
    };
    let b = match matches.opt_str("b").map(|b| b.parse::<usize>()) {
        Some(Ok(val)) if val > 0 => val,
        _ => {
            println!("Error: Missing or invalid value for option -b. It must be greater than 0.");
            print_usage( &opts);
            return;
        }
    };
    let tracefile = match matches.opt_str("t") {
        Some(file) => file,
        None => {
            println!("Error: Missing value for option -t");
            print_usage(&opts);
            return;
        }
    };

    // Initialize cache with specified parameters
    let mut cache = Cache {
        cache: vec![Set { set: vec![Line { block: 0, validity: false, tag: 0, age: 0 }; e], }; 1 << s],
        hit: 0,
        miss: 0,
        eviction: 0,
    };

    // Operate cache based on trace file and print cache statistics
    operate_flags(&tracefile, &mut cache, b, s, e);
    println!("hits:{}, misses:{}, evictions:{}", cache.hit, cache.miss, cache.eviction);
}