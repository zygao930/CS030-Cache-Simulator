#![allow(unused)]
extern crate getopts;
use getopts::Options;
use std::env;
use std::fs;
use std::io::{BufReader, BufRead};
use std::fs::File;

// Define the Line struct
#[derive(Clone)] 
struct Line {
    // Line stores its block, validity, tag, and recency/age information.
    block: u64,
    validity: bool,
    tag: u64,
    age: u32,
}

// Define the Set struct
#[derive(Clone)] 
struct Set {
    // Set represents a set in the cache and stores an array of Line structures
    // and the set's current rate and placement rate.?????
    set: Vec<Line>,
    //current_rate??placement rate??
}

// Define the Cache struct
struct Cache {
    // Cache represents the entire cache and stores an array of Set structures, 
    // various cache parameters, performance statistics, and a flag indicating the eviction policy.
    cache: Vec<Set>,
    hit_rate: u32,
    miss_rate: u32,
    eviction_policy: String,
}

enum Cacheresult {
    Hit,      
    Miss,
    Full           
 }

 fn hex_to_binary(hex_address: &str) -> Result<String, String> {
    let decimal: usize = match usize::from_str_radix(hex_address, 16) {
        Ok(x) => x,
        Err(err) => panic!("Error parsing hex address {} ({})", hex_address, err),
    };
    let binary = format!("{:0>64b}", decimal); 
    Ok(binary)
}

// Function to check for cache hits, misses, and evictions
fn check_cache(s: usize, cache: &mut Cache, set: usize, tag: u64) -> Result<Cacheresult, String> {
    // Implement the logic for cache operations
    // Update cache statistics and data structures
    // HIT/MISS/EVIC MISS  
    let mut empty_line = 0;
    for line in &cache.cache[set].set {
        if line.validity == false {
            empty_line += 1;
        }
    }

    for line in &cache.cache[set].set {        
        if line.validity == true && line.tag == tag {
            return Ok(Cacheresult::Hit);           
        } 
    }           

    if empty_line < s {   
        return Ok(Cacheresult::Miss);
    } else {
        return Ok(Cacheresult::Full);
    } 
    Err(String::from("Cache operation failed"))              
}

fn evict(s: usize, cache: &mut Cache, block: u64, set: usize, tag: u64)  {
    // Use Least-Recently-Used as the replacement policy to Implement the eviction logic
    // Update cache statistics and data structures 
    let mut min_index: usize = 0;
    for i in 0..s{
        let age = cache.cache[set].set[i].age;
        let min_age = cache.cache[set].set[min_index].age;
        if age < min_age{
            min_index = i;
        }
        cache.cache[set].set[i].age -= 1;
    }
    let evict_line = &mut cache.cache[set].set[min_index];
    evict_line.age = s as u32 - 1;
    evict_line.block = block;
    evict_line.tag = tag;
    evict_line.validity = true;
}

fn operate_cache(address: &str, cache: &mut Cache, b: usize, s: usize, operation: &str)  -> Result<(), String>{
    // implement cache operation
    let mut hit: u32 = 0;
    let mut miss: u32 = 0;
    let mut evict: u32 = 0;

    let binary_address = match hex_to_binary(address) {
        Ok(binary_address) => binary_address,
        Err(err) => return Err(format!("Error converting hex to binary: {}", err)),
    };

    let block = match binary_address[(64 - b )..64].parse::<u64>() {
        Ok(block) => block,
        Err(err) => return Err(format!("Error parsing set index: {}", err)),
    };

    let set = match binary_address[(64 - b - s)..(64 - b)].parse::<usize>() {
        Ok(set) => set,
        Err(err) => return Err(format!("Error parsing set index: {}", err)),
    };

    let tag = match binary_address[0..(64 - b - s)].parse::<u64>() {
        Ok(tag) => tag,
        Err(err) => return Err(format!("Error parsing tag: {}", err)),
    };

    match operation {
        "I" => {
            for line in &cache.cache[set].set{
                line.age -= 1;
            }               
         }
        "L" | "S" => {
            match check_cache(s, cache, set, tag){
                Ok(Cacheresult::Hit) => {
                    for line in &cache.cache[set].set{
                        line.age -= 1;
                        hit += 1;
                    }
                }
                Ok(Cacheresult::Miss) => {
                    let mut empty_line = 0;
                    for line in &cache.cache[set].set {
                        if line.validity == false {
                            empty_line += 1;
                        }
                    }
                    for line in &cache.cache[set].set{
                        if line.validity == false{
                            line.age == 0;
                            line.validity == true;
                            line.block = block;
                            line.tag = tag;
                        }
                    }
                }
                Ok(Cacheresult::Full) => {
                    for line in &cache.cache[set].set{
                        line.age -= 1;
                        hit += 1;
                    }
                }
                Err(_) => todo!(),
            }
            } 


            //check_cache(s, cache, set, tag);

        
            
            
        
    } Ok(())
}

fn operate_flags(trace_file: &str, cache: &mut Cache, b: usize, s: usize) {
    // Implement the logic to read the trace file and call check_cache function
    // Update cache statistics
    match File::open(trace_file) {
        Ok(file) => {   
            let read = BufReader::new(file);
            for line in read.lines() {
                match line {
                    Ok(line) => {               
                        let statements: Vec<&str> = line.trim().split(',').collect();
                        if statements.len() != 2 {
                            eprintln!("Invalid line format: {}", line);
                            continue;
                        }
                        let oper = statements[0];
                        let size = statements[1];

                        let statement: Vec<&str> = oper.trim().split_whitespace().collect();
                        let operation = statement[0];
                        let hex_address = statement[1];

                        operate_cache(hex_address, cache, b, s, operation);
                        println!("{}", operation);
                    }
                    Err(e) => {
                        eprintln!("Error reading line from trace file: {}", e);
                    }
                }
            }
        }
        Err(err) => {
            eprintln!("Error opening trace file: {}", err);
        }
    }
}

// Function to print the final result of the simulation
fn print_simulation_result(cache: &Cache) {
    // Implement the logic to print simulation result
}

// Main function
fn main() {
    // Parse command-line arguments
    let args: Vec<String> = env::args().collect();
    let mut opts = Options::new();

    opts.optflag("h", "help", "that prints usage info");
    opts.optflag("v", "verbose", "that displays trace info");
    opts.optopt("s", "", "Number of set index bits (S = 2s is the number of sets)", "s");
    opts.optopt("E", "", "Associativity (number of lines per set)", "E");
    opts.optopt("b", "", "Number of block bits (B = 2b is the block size)", "b"); 
    opts.optopt("t", "", "Name of the trace to replay", "tracefile");
    
    let matches = match opts.parse(&args[1..]) {
        Ok(m) => m,
        Err(e) => {  
            eprintln!("Error: {}", e);
            return;
        }
    };
    if matches.opt_present("h") {
        println!("{}", opts.usage("Usage: ./sim-ref [-hv] -s <s> -E <E> -b <b> -t <tracefile>"));
        println!("Options:");
        println!("  -h        Print this help message.");
        println!("  -v        Optional verbose flag.");
        println!("  -s <num>  Number of set index bits.");
        println!("  -E <num>  Number of lines per set.");
        println!("  -b <num>  Number of block offset bits.");
        println!("  -t <file> Trace file.\n");
        println!("Examples:");
        println!("  linux>  ./sim-ref -s 4 -E 1 -b 4 -t traces/yi.trace");
        println!("  linux>  ./sim-ref -v -s 8 -E 2 -b 4 -t traces/yi.trace");
        return;
    }

    let e = match matches.opt_str("E"){
        Some(e) => match e.parse::<usize>() {
            Ok(num) => num,
            Err(_) => {
                eprintln!("Invalid input for set index bits.");
                return;
            }
        },
        None => {
            eprintln!("Missing input for set index bits.");
            return;
        }
    };

    let s = match matches.opt_str("s"){
        Some(s) => match s.parse::<usize>() {
            Ok(num) => num,
            Err(_) => {
                eprintln!("Invalid input for set index bits.");
                return;
            }
        },
        None => {
            eprintln!("Missing input for set index bits.");
            return;
        }
    };

    // Initialize cache with specified parameters
    let cache = Cache {
        cache: vec![Set { set: vec![Line { block: 0, validity: false, tag: 0, age: 0 }; e] }; s],
        hit_rate: 0,
        miss_rate: 0,
        eviction_policy: String::from("LRU"),
    };

    // Call operate_flags function to simulate cache behavior
    operate_flags(&trace_file, &mut cache);

    // Print the final result of the simulation
    print_simulation_result(&cache);
}
