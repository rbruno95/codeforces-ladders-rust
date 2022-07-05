use std::io;

fn main() {
    let n: i32 = read_int();
    let mut sum: Vec<i32> = vec![0, 0, 0];

    for _ in 0..n {
        let vector: Vec<i32> = read_vector_int();

        for i in 0..3 {
            sum[i] += vector[i];
        }
    }

    println!("{}", if sum == vec![0, 0, 0] { "YES" } else { "NO" });
}

fn read_int() -> i32 {
    read_line().trim().parse().unwrap()
}

fn read_vector_int() -> Vec<i32> {
    read_line()
        .split(' ')
        .map(|number| number.trim().parse::<i32>().unwrap())
        .collect()
}

fn read_line() -> String {
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();

    input
}
