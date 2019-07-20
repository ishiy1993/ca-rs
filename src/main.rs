extern crate getopts;
use getopts::Options;
use std::env;

fn step_rule30(l: bool,c: bool, r: bool) -> bool {
    match (l,c,r) {
        (false,false,false) => false,
        (false,false,true) => true,
        (false,true,false) => true,
        (false,true,true) => true,
        (true,false,false) => true,
        (true,false,true) => false,
        (true,true,false) => false,
        (true,true,true) => false,
    }
}

fn step_rule110(l: bool,c: bool, r: bool) -> bool {
    match (l,c,r) {
        (false,false,false) => false,
        (false,false,true) => true,
        (false,true,false) => true,
        (false,true,true) => true,
        (true,false,false) => false,
        (true,false,true) => true,
        (true,true,false) => true,
        (true,true,true) => false,
    }
}

fn step_rule90(l: bool,c: bool, r: bool) -> bool {
    match (l,c,r) {
        (false,false,false) => false,
        (false,false,true) => true,
        (false,true,false) => false,
        (false,true,true) => true,
        (true,false,false) => true,
        (true,false,true) => false,
        (true,true,false) => true,
        (true,true,true) => false,
    }
}

fn update<F>(f: F, xs: &Vec<bool>) -> Vec<bool>
    where F: Fn(bool,bool,bool) -> bool
{
    let mut ys = Vec::new();
    let n = xs.len();
    ys.push(f(false,xs[0],xs[1]));
    for i in 1..n-1 {
        ys.push(f(xs[i-1],xs[i],xs[i+1]));
    }
    ys.push(f(xs[n-2],xs[n-1],false));
    ys
}

fn init(n: usize) -> Vec<bool> {
    let mut xs = Vec::new();
    for i in 0..n {
        xs.push(i == n/2);
    }
    xs
}

fn format_state(xs: &Vec<bool>) -> String {
    let n = xs.len();
    let mut output = String::new();
    for i in 0..n {
        if xs[i] {
            output.push('#');
        } else {
            output.push(' ');
        }
    }
    output
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let mut opts = Options::new();
    opts.reqopt("r","","Set rule", "RULE");
    opts.reqopt("x","nx","Set the number of grid", "NX");
    opts.reqopt("t","nt","Set the number of update", "NT");
    let matches = match opts.parse(&args[1..]) {
        Ok(m) => { m },
        Err(f) => { panic!(f.to_string()) },
    };

    let step = match matches.opt_str("r") {
        Some(ref r) if r == "rule30" => step_rule30,
        Some(ref r) if r == "rule110" => step_rule110,
        Some(ref r) if r == "rule90" => step_rule90,
        r => panic!(format!("Not support: {:?}",r)),
    };
    let read: fn(String) -> usize = |s| s.parse().expect("Need a number");
    let nx = match matches.opt_str("x").map(read) {
        Some(x) if x > 0 => { x },
        _ => panic!("Need -x <int>"),
    };
    let mut nt = match matches.opt_str("t").map(read) {
        Some(t) if t > 0 => { t },
        _ => panic!("Need -t <int>"),
    };
    let mut xs = init(nx);

    while nt > 0 {
        println!("|{}|",format_state(&xs));
        xs = update(step,&xs);
        nt -= 1;
    }
}
