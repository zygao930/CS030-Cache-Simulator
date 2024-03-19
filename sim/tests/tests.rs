use sim::cache_simulator::*;
use std::process::Command;

 // Positive Test Cases for all traces files with different inputs
#[test]
fn execute_with_valid_arguments_yi() {
    let b: usize = 4;
    let e: usize = 1;
    let s: usize = 4;
    let mut cache = Cache {
        cache: vec![Set {set: vec![Line {block: 0, validity: false, tag: 0, age: 0,}; e],}; 1 << s],
        hit: 0,
        miss: 0,
        eviction: 0,
    };

    operate_flags("./traces/yi.trace", &mut cache, b, s, e);
    assert_eq!(cache.hit, 4);
    assert_eq!(cache.miss, 5);
    assert_eq!(cache.eviction, 3);        
}

#[test]
fn execute_with_valid_arguments_yi2() {
    let s: usize = 12;   
    let e: usize = 2;
    let b: usize = 7;
    let mut cache = Cache {
        cache: vec![Set {set: vec![Line {block: 0, validity: false, tag: 0, age: 0,}; e],}; 1 << s],
        hit: 0,
        miss: 0,
        eviction: 0,
    };

    operate_flags("./traces/yi2.trace", &mut cache, b, s, e);
    assert_eq!(cache.hit, 16);
    assert_eq!(cache.miss, 1);
    assert_eq!(cache.eviction, 0);        
}

#[test]
fn execute_with_valid_arguments_ibm() {
    let s: usize = 3;   
    let e: usize = 2;
    let b: usize = 2;
    let mut cache = Cache {
        cache: vec![Set {set: vec![Line {block: 0, validity: false, tag: 0, age: 0,}; e],}; 1 << s],
        hit: 0,
        miss: 0,
        eviction: 0,
    };

    operate_flags("./traces/ibm.trace", &mut cache, b, s, e);
    assert_eq!(cache.hit, 0);
    assert_eq!(cache.miss, 5);
    assert_eq!(cache.eviction, 0);        
}

#[test]
fn execute_with_valid_arguments_trans() {
    let s: usize = 8;   
    let e: usize = 3;
    let b: usize = 6;
    let mut cache = Cache {
        cache: vec![Set {set: vec![Line {block: 0, validity: false, tag: 0, age: 0,}; e],}; 1 << s],
        hit: 0,
        miss: 0,
        eviction: 0,
    };

    operate_flags("./traces/trans.trace", &mut cache, b, s, e);
    assert_eq!(cache.hit, 233);
    assert_eq!(cache.miss, 5);
    assert_eq!(cache.eviction, 0);        
}

#[test]
fn execute_with_valid_arguments_long() {
    let s: usize = 8;   
    let e: usize = 1;
    let b: usize = 6;
    let mut cache = Cache {
        cache: vec![Set {set: vec![Line {block: 0, validity: false, tag: 0, age: 0,}; e],}; 1 << s],
        hit: 0,
        miss: 0,
        eviction: 0,
    };

    operate_flags("./traces/long.trace", &mut cache, b, s, e);
    assert_eq!(cache.hit, 281074);
    assert_eq!(cache.miss, 5890);
    assert_eq!(cache.eviction, 5634);        
}

// Boundary Value Test Cases
#[test]
fn execute_with_boundaray_long() {
    let s: usize = 1;   
    let e: usize = 1;
    let b: usize = 62;
    let mut cache = Cache {
        cache: vec![Set {set: vec![Line {block: 0, validity: false, tag: 0, age: 0,}; e],}; 1 << s],
        hit: 0,
        miss: 0,
        eviction: 0,
    };

    operate_flags("./traces/long.trace", &mut cache, b, s, e);
    assert_eq!(cache.hit, 286963);
    assert_eq!(cache.miss, 1);
    assert_eq!(cache.eviction, 0);        
}

// Error Handling Test Cases
#[test]
fn execute_value_equals_0() {
    let args = ["-s", "0" , "-E", "1", "-b", "4", "-t", "traces/yi.trace"];
    let output = Command::new("target/debug/sim")
        .args(&args)
        .output()
        .expect("Failed to execute command");
    let output_string = String::from_utf8_lossy(&output.stdout).to_string();
    println!("Command output: {}", output_string); 
    assert!(output_string.contains("Error: Missing or invalid value for option -s. It must be greater than 0."));
}

#[test]
fn execute_value_missing() {
    let args = ["-s", "4" , "-E", "-b", "4", "-t", "traces/yi.trace"];
    let output = Command::new("target/debug/sim")
        .args(&args)
        .output()
        .expect("Failed to execute command");
    let output_string = String::from_utf8_lossy(&output.stdout).to_string();
    println!("Command output: {}", output_string); 
    assert!(output_string.contains("Error: Missing or invalid value for option -E. It must be greater than 0."));
}

//Overflow Test Case
#[test]
fn execute_value_overflow() {
    let args = ["-s", "74" , "-E", "4", "-b", "80", "-t", "traces/yi.trace"];
    let _output = Command::new("target/debug/sim")
        .args(&args)
        .output()
        .expect("Failed to execute command");
}