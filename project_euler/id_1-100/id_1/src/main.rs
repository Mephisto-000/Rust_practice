// Problem 1 :
// If we list all the natural numbers below 10
// that are multiples of 3 or 5, we get 3, 5, 6 and 9.
// The sum of these multiples is 23.
// Find the sum of all the multiples of 3 or 5 below 1000.

fn main() {
    let mut sum_of_multiples: u32 = 0;

    for n_index in (1..1000).rev() {
        if multiples_check(n_index) == true {
            sum_of_multiples += n_index;
        }
    }

    println!("Result : {}", sum_of_multiples);
}

fn multiples_check(x: u32) -> bool {
    if x % 3 == 0 {
        return true;
    } else if x % 5 == 0 {
        return true;
    } else {
        false
    }
}

// Nice 1:
// fn main() {
//     let answer: i32 = (3..1_000)
//         .filter(|number| number % 3 == 0 || number % 5 == 0)
//         .sum();
//     println!("Answer: {}", answer);
// }

// Nice 2:
// fn main() {
//     let mut sum: u32 = 0;

//     for i in 1..1000 {
//         if i % 3 == 0 || i % 5 == 0 {
//             sum += i;
//         }
//     }

//     println!("sum: {}", sum);
// }




