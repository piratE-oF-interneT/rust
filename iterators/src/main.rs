pub mod cli_tool;
fn main() {
    println!("Hello, world!");

    // let mut counter = Counter::new();
    // // println!("{:?}", counter.next());
    // // println!("{:?}", counter.next());
    // // println!("{:?}", counter.next());
    // // println!("{:?}", counter.next());
    // // println!("{:?}", counter.next());
    // // println!("{:?}", counter.next());
    // // println!("{:?}", counter.next());

    // for cnt in counter.into_iter() {
    //     println!("{:?}", cnt);
    // }

    let nums = vec![1, 2, 3, 4, 5, 6, 7, 8];

    let odd_iter = nums.iter().map(|x| x * 1);

    let even_iter = nums.into_iter().map(|x| x * 2);

    for i in even_iter {
        println!("{}", i);
    }
}

// struct Counter {
//     count: u32,
// }

// impl Counter {
//     fn new() -> Self {
//         Counter { count: 0 }
//     }
// }
// impl Iterator for Counter {
//     type Item = u32;

//     fn next(&mut self) -> Option<Self::Item> {
//         if self.count < 5 {
//             self.count += 1;
//             Some(self.count)
//         } else {
//             None
//         }
//     }
// }
