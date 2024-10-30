fn find_max_sum<const N: usize>(array: [i32; N]) -> i32 {
    let mut max_sum = 0;
    let mut curr_sum = 0;

    for i in array {
        curr_sum += i;
        max_sum = if curr_sum > max_sum {curr_sum} else {max_sum};
    }
    
    max_sum
}

fn find_max_sum_k_alg<const N: usize>(array: [i32; N]) -> i32 {
    let mut max_sum = array[0];
    let mut curr_sum = array[0];

    for &i in &array[1..] {
        curr_sum = i.max(curr_sum + i);

        max_sum = max_sum.max(curr_sum);
    }

    max_sum
}

fn main() {
    const N: usize = 4;
    let array: [i32; N] = [3, -2, 5, -1];

    println!("brute force: {}", find_max_sum(array));
    println!("Kadanes alg: {}", find_max_sum_k_alg(array));
}
