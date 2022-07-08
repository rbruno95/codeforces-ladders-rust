use std::io;

fn main() {
    let (n, t) = read_two_int();
    let mut queue = read_line().into_bytes();

    for _ in 0..t {
        let mut skip: bool = false;
        for i in 0..n as usize - 1 {
            if !skip && queue[i] == b'B' && queue[i + 1] == b'G' {
                queue.swap(i, i + 1);
                skip = true;
            } else {
                skip = false;
            }
        }
    }

    println!("{}", String::from_utf8(queue).unwrap());
}

fn read_two_int() -> (i32, i32) {
    let vector: Vec<i32> = read_line()
        .split(' ')
        .map(|x| x.trim().parse::<i32>().unwrap())
        .collect();

    (vector[0], vector[1])
}

fn read_line() -> String {
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();

    input
}
