pub mod cache_simulator{
    use std::io::{BufReader, BufRead};
    use std::fs::File;
    // Structure representing the line,set and cache
    #[derive(Clone)] 
    pub struct Line {       
        pub block: u64,     // Block number
        pub validity: bool, // Valid bit
        pub tag: u64,       // Tag bits
        pub age: i32,       // Age for LRU eviction
    }

    #[derive(Clone)] 
    pub struct Set {
        pub set: Vec<Line>, // Array of lines in the set
    }

    pub struct Cache {
        pub cache: Vec<Set>, // Array of sets in the cache
        pub hit: u32,        // Number of cache hits
        pub miss: u32,       // Number of cache misses
        pub eviction: u32,   // Number of cache evictions
    }

    // Enum representing the result of a cache operation (Hit, Miss, Full)
    pub enum Cacheresult {
        Hit,
        Miss,
        Full,
    }

    // Function to convert hexadecimal address to binary string
    pub fn hex_to_binary(hex_address: &str) -> Result<String, String> {
        let decimal: usize = match usize::from_str_radix(hex_address, 16) {
            Ok(x) => x,
            Err(err) => panic!("Error parsing hex address {} ({})", hex_address, err),
        };
        let binary = format!("{:0>64b}", decimal); 
        Ok(binary)
    }

    // Function to check for cache hits, misses, and evictions
    pub fn check_cache(cache: &mut Cache, set: usize, tag: u64) -> Result<Cacheresult, String> {
        let mut empty_line = 0;
        for line in &cache.cache[set].set {
            if !line.validity {
                empty_line += 1;
            }
        }      

        // Check if the tag matches any valid line in the set
        for line in &cache.cache[set].set {        
            if line.validity && line.tag == tag {
                return Ok(Cacheresult::Hit);           
            } 
        }     

        // If there are empty lines, return Miss, else return Full
        if empty_line > 0 {   
            return Ok(Cacheresult::Miss);
        } else {
            return Ok(Cacheresult::Full);
        }        
    }

    // Function to evict a cache line using LRU policy
    pub fn evict(cache: &mut Cache, block: u64, set: usize, tag: u64, e: usize)  {
        let mut min_index: usize = 0;
        // Find the line with the minimum age (LRU line)
        for i in 1..e {  
            let age = cache.cache[set].set[i].age;
            let min_age = cache.cache[set].set[min_index].age;
            if age < min_age {
                min_index = i;
            } cache.cache[set].set[i].age -= 1;
        }
        // Evict the LRU line and replace it with the new line
        let evict_line = &mut cache.cache[set].set[min_index];
        evict_line.age = e as i32 - 1;
        evict_line.block = block;
        evict_line.tag = tag;
        evict_line.validity = true;
    }

    // Function to perform cache operations (Load, Store, Modify)
    pub fn operate_cache(address: &str, cache: &mut Cache, b: usize, s: usize, e: usize, operation: &str) -> Result<(), String> {
        let binary_address = match hex_to_binary(address) {
            Ok(binary_address) => binary_address,
            Err(err) => return Err(format!("Error converting hex to binary: {}", err)),
        };

        // Extract block, set index, and tag from the binary address
        let block = match u64::from_str_radix(&binary_address[(64 - b)..64], 2)  {
            Ok(block) => block,
            Err(err) => return Err(format!("Error parsing set index: {}", err)),
        };
        let set_index_binary = &binary_address[(64 - b - s)..(64 - b)];
        let set_index_decimal = match u64::from_str_radix(set_index_binary, 2) {
            Ok(decimal) => decimal,
            Err(err) => return Err(format!("Error parsing set index: {}", err)),
        };
        let set = set_index_decimal as usize;
        let tag = match u64::from_str_radix(&binary_address[0..(64 - b - s)], 2) {
            Ok(tag) => tag,          
            Err(err) => return Err(format!("Error parsing tag: {}", err)),
        };

        // Perform cache operation based on the operation type
        match operation {
            "I" => {
                for line in &mut cache.cache[set].set {
                    line.age -= 1;
                }
            }
            "L" | "S" => {
                match check_cache(cache, set, tag) {
                    Ok(Cacheresult::Hit) => {    // Handle cache hit
                        cache.hit += 1;
                        for line in &mut cache.cache[set].set {
                            line.age -= 1;
                        } //println!("{} {},{} hit", operation, address, size);
                    }
                    Ok(Cacheresult::Miss) => {   // Handle cache miss
                        cache.miss += 1;
                        for line in &mut cache.cache[set].set {
                            line.age -= 1;
                            if !line.validity {   // If an empty line is found, insert the new line
                                line.age = e as i32 - 1;
                                line.validity = true;
                                line.block = block;
                                line.tag = tag;
                                break;
                            }
                        } //println!("{} {},{} miss", operation, address, size);
                    }
                    Ok(Cacheresult::Full) => {   // Handle cache full (miss with eviction)
                        cache.miss += 1;
                        cache.eviction += 1;
                        evict(cache, block, set, tag, e);
                        //println!("{} {},{} miss eviction", operation, address, size);
                    } Err(_) => todo!(),
                }
            }             
            "M" => {
                match check_cache(cache, set, tag) {
                    Ok(Cacheresult::Hit) => {         // Handle cache hit for modify operation
                        cache.hit += 2;
                        for line in &mut cache.cache[set].set {
                            if line.validity && line.tag == tag {
                                line.age = e as i32 - 1;
                                line.block = block;
                                line.tag = tag;
                                } 
                            } //println!("{} {},{} hit hit", operation, address, size);                     
                        }
                   Ok(Cacheresult::Miss) => {       // Handle cache miss for modify operation
                        cache.miss += 1;
                        for line in &mut cache.cache[set].set {
                            line.age -= 1;
                            if !line.validity {
                                line.age = e as i32 - 1;
                                line.validity = true;
                                line.block = block;
                                line.tag = tag;
                                cache.hit += 1;
                                break;
                            } 
                        } //println!("{} {},{} miss hit", operation, address, size);
                    }
                    Ok(Cacheresult::Full) => {      // Handle cache full for modify operation (miss with eviction)
                        cache.miss += 1;
                        cache.eviction += 1;
                        evict(cache, block, set, tag, e);
                        cache.hit += 1;
                        //println!("{} {},{} miss eviction hit", operation, address, size);
                    } Err(_) => todo!(),
                }
            }  _ => {} 
        } Ok(())
    }

    // Function to read trace file and operate cache accordingly
    pub fn operate_flags(trace_file: &str, cache: &mut Cache, b: usize, s: usize, e: usize) {
        match File::open(trace_file) {
            Ok(file) => {
                let read = BufReader::new(file);
                for line in read.lines() {
                    match line {
                        Ok(line) => {
                            let statements: Vec<&str> = line.trim().split(',').collect();
                            if statements.len() != 2 {
                                continue;
                            }
                            let oper = statements[0];
                            let statement: Vec<&str> = oper.trim().split_whitespace().collect();
                            let operation = statement[0];
                            let hex_address = statement[1];
                            match operate_cache(hex_address, cache, b, s, e, operation) {
                                Ok(_) => {}, 
                                Err(err) => {
                                    println!("Error processing cache operation: {}", err);
                                    break; 
                                }
                            }
                        } Err(e) => {
                            println!("Error reading line from trace file: {}", e);
                        }
                    }
                }
            } Err(err) => {
                println!("Error opening trace file: {}", err);
            }
        }
    }
}