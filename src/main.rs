use clap::Parser;

fn primesieve(max: usize) -> Vec<bool> {
    println!("Searching for primes less than {max}");

    let g_factor = (max as f32).sqrt();

    println!("Greatest factor is {g_factor}");

    // Create boolean vector
    // Creating the full size vector is more efficient than creating a small one and incrementally adding to it
    let mut mask = vec![true; max];

    // Manually mark 0 and 1 as non-prime
    mask[0] = false;
    mask[1] = false;

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

/// Simple program to calculate prime numbers using a sieve
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// Number to stop search at
    #[arg(short, long, default_value_t = 1000)]
    search_end: usize,

    /// List all primes found
    #[arg(short, long)]
    list: bool,
}

fn main() {
    let args = Args::parse();

    let out = primesieve(args.search_end);

    if args.list {
        for (n, val) in out.iter().enumerate() {
            if *val {
                println!("{n}");
            }
        }
    }

    // Print a count of the number of primes found
    println!(
        "\n{} primes smaller than {}",
        out.iter().filter(|x| **x).count(),
        args.search_end
    );
}
