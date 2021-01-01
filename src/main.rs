use clap::{value_t_or_exit, App, Arg};
use rand::{thread_rng, Rng};

fn main() {
    let matches = App::new("password generator")
        .author("Jaehyeon Park <skystar@skystar.dev>")
        .arg(
            Arg::with_name("length")
                .index(1)
                .required(true)
                .help("length of password")
        )
        .get_matches_safe()
        .unwrap_or_else(|e| e.exit());

    let length = value_t_or_exit!(matches, "length", usize);
    let mut rng = thread_rng();
    let res: String = (&mut rng)
        .sample_iter(rand::distributions::Alphanumeric)
        .take(length)
        .map(char::from)
        .collect();
    println!("{}", res)
}
