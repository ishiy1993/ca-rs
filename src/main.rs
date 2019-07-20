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
    let nx = 100;
    let mut nt = 200;
    let mut xs = init(nx);

    while nt > 0 {
        println!("|{}|",format_state(&xs));
        xs = update(step_rule90,&xs);
        nt -= 1;
    }
}
