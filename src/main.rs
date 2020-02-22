extern crate primal;

// Inspired from: https://www.quantamagazine.org/mathematicians-discover-prime-conspiracy-20160313/

fn main() {
    let counter: Vec<Vec<i32>> = vec![vec![0; 10]; 10];

    let (last, counter) =
        primal::Primes::all()
            .take(100_000_000)
            .fold((1, counter), |(last, mut counter), n| {
                counter[last % 10][n % 10] += 1;
                (n, counter)
            });

    for (i, from) in counter.iter().enumerate() {
        let mut sum = 0;
        for to in from.iter() {
            if to > &1 {
                sum += to;
            }
        }

        for (j, to) in from.iter().enumerate() {
            if to > &1 {
                println!(
                    "from {:?} to {:?}: {:?} ({:.5})",
                    i,
                    j,
                    to,
                    *to as f64 / sum as f64
                );
            }
        }

        if sum > 0 {
            println!("from {:?} to *: {:?}", i, sum);
            println!();
        }
    }

    println!("last: {:?}", last);
}
