#![allow(dead_code, warnings)]

use {
    rand::Rng,
    std::env,
};

fn gen_rand_range(floor: i64, ciel: i64) -> i64 {
    let mut rng = rand::thread_rng();

    return rng.gen_range(floor..ciel);
}

fn guess(num: i64, x: i64) -> i8 {
    return if num == x { 0 } else if num < x  {-1} else {1};
}

fn algo(min: i64, max: i64) {
    let rand = gen_rand_range(min, max);
    let mut low = min;
    let mut high = max;
    let mut guesses = 0;
    
    while low <= high {
        guesses += 1;

        let g = (low + high) / 2;
        
        match guess(rand, g) {
            0 => {
                println!("Number guessed in {} guesses and it's {}", guesses, g);
                return;
            },
            -1 => high = g-1,
            1 => low = g+1,
            _ => unreachable!(),
        }
    }


}

fn main() {
    algo(1, 1000);
}
