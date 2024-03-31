// fn main() {
//     println!("Hello, world!");
//     let count = fizz_buzz();
//     println!("Number of times fizz buzz occured: {}", count);
// }

// fn fizz_buzz() -> u32 {
//     let mut count = 0;
//     for i in 0..=301 {
//         if i % 3 == 0 && i % 5 == 0 {
//             println!("fizz buzz");
//             count += 1;
//         } else if i % 3 == 0 {
//             println!("fizz");
//         } else if i % 5 == 0 {
//             println!("buzz");
//         }
//     }
//     count
// }

fn main() {
    println!("Hello, world!");
    
    let count = (0..=301).filter_map(|i| {
        if i % 3 == 0 && i % 5 == 0 {
            println!("fizz buzz");
            Some(())
        } else if i % 3 == 0 {
            println!("fizz");
            None
        } else if i % 5 == 0 {
            println!("buzz");
            None
        } else {
            None
        }
    }).count();
    
    println!("Number of fizz buzz occurrences: {}", count);
}