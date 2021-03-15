use std::env;
use std::fs::File;
use std::io::Read;

fn head(file:&str, lines:u32){
    let mut vec: Vec<u16> = Vec::new();
    let mut f = File::open(file).unwrap();
    let mut buf: [u8; 2] = [0, 0];
    let mut cnt = 0;
    while f.read(&mut buf).unwrap() > 0{
        let c = [((buf[1] as u16) << 8) | buf[0] as u16];
        vec.push(c[0]);
        if String::from_utf16(&c).unwrap() == "\n"{
            cnt += 1;
            print!("{}", &String::from_utf16(&vec).unwrap());
            vec.clear();
            if cnt == lines{break;}
        }
    }
    if f.read(&mut buf).unwrap() == 0{
        println!("{}", &String::from_utf16(&vec).unwrap());
    }
}

fn main() {
    let args:Vec<String> = env::args().collect();
    if args.len() % 2 != 0{
        eprintln!("Usage: ...");
    }
    else{
        let _n:u32 = if args.contains(&String::from("-n")) {args[(args.iter().position(|a| a == "-n").unwrap() + 1)].parse::<u32>().unwrap()} else {10};
        head(args.last().unwrap(), _n);
    }
}