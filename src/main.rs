use std::env;

fn primesieve(max: usize) -> Vec<bool> {
    println!("Searching for primes less than {max}");

    let g_factor = (max as f32).sqrt();

    println!("Greatest factor is {g_factor}");

    // Create boolean vector
    // Creating the full size vector is more efficient than creating a small one and incrementally adding to it
    let mut mask = vec![true; max];

    mask[0] = false;
    mask[1] = false;

    //println!("{:?}", mask);

    for n in 2..=(g_factor as usize) {
        // Check that the divisor is a prime
        if mask[n] {
            for (m, p) in mask.iter_mut().enumerate().skip(n + 1) {
                if !*p {
                    continue;
                }
                if m % n == 0 {
                    *p = false;
                }
            }
        }
    }

    mask
}

fn main() {
    let args: Vec<String> = env::args().collect();

    let search_end: usize = if args.len() > 1 {
        args[1].trim().parse().unwrap_or(1000)
    } else {
        1000
    };

    let out = primesieve(search_end);

    for (n, val) in out.iter().enumerate() {
        if *val {
            println!("{n}");
        }
    }

    println!(
        "\n{} primes in total...",
        out.iter().filter(|x| **x).count()
    );
}
