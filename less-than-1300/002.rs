use std::io;

fn main() {
    let mut sol: (i32, i32) = (0, 0);

    for i in 0..5 {
        let row = read_vector_int();
        for j in 0..5 {
            if row[j as usize] == 1 {
                sol = (i, j);
            }
        }
    }

    println!("{}", (2 - sol.0).abs() + (2 - sol.1).abs());
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
