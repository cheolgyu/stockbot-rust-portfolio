use std::{env, process::Command};

fn run() {
    println!("Hello==> world!");
    let args: Vec<String> = env::args().collect();
    dbg!(&args);

    let mut i = 0;
    for arg in args.clone() {
        println!("{}=>{}", i, arg);
        i = i + 1;
    }

    t1();
    t2();
}

fn t1() {
    Command::new("sh")
        .arg("-c")
        .arg("echo hello23")
        .spawn()
        .expect("ls command failed to start");
}

fn t2() {
    //cargo run -- C:/Users/example/Documents/GitHub/stockbot/hello/script/hello.shC:/Users/example/Documents/GitHub/stockbot/hello/script/hello.sh
    let args: Vec<String> = env::args().collect();

    let query = &args[2];
    println!("Searching for {}", query);
    println!("Searching for {}", query);

    Command::new("sh")
        .arg("-c")
        .arg(query)
        .spawn()
        .expect("ls command failed to start");
}
