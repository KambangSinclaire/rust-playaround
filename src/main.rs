const _OUR_COURSE: &str = "Rust With AutoGPT";

mod m1_enums;
mod m2_structs;
mod m3_traits;
mod m4_polimorphism;
mod m5_lifetimes;
mod m6_pattern_matching;
mod m7_api_server;
mod m8_collections;
mod m9_declarative_macros; 

fn main() {
    // println!("Welcome to this course {}", OUR_COURSE);

    // // Stack
    // let x: i32;
    // x = 2;
    // println!("X is {}", x);
    // let y: i32 = 4;
    // println!("Y is {}", y);

    // // for loop
    // for i in 0..=y {
    //     if i != 4 {
    //         print!("{},", i);
    //     } else {
    //         println!("{},", i);
    //     }
    // }

    // // Mutation
    // let mut z = 35;
    // print!("z was {} ", z);
    // z = 10;
    // print!("But is now {}", z);

    // let freezing_temp: f64 = -2.4;
    // println!("freezing_temp is {}", freezing_temp);

    // let is_zero_remainder: bool = 10 % 4 != 0;
    // println!("is_zero_remainder is {}", is_zero_remainder);

    // let my_char: char = 'z';
    // println!("My Char is {} ", my_char);

    // let emoji_char = '😅';
    // println!("emoji_char is {} ", emoji_char);

    // // This creates an array stored on the stack
    // // fixed-size
    // let my_arr: [f32; 10] = [0.0; 10];
    // // println!("my_arr is {:#?}",my_arr) - For pretty print of an array
    // println!("my_arr is {:?}", my_arr);

    // // The line below maps through and array and executes a closure on each item
    // // of the array. The closure is like JS's arrow function
    // // The closure adds 2.0 on each item of the array.
    // let new_my_array: [f32; 10] = my_arr.map(|n| n + 2.0);
    // println!("8new_my_array is {:?}", new_my_array);
    // //

    // This is a static string and gets stored on the stack
    // let name: &str = "Kambang";
    // // This is a dynamic string and gets stored on the heap
    // let d_name: String = String::from("Kambang");

    // println!("Static string is {:?} ", name);
    // println!("Dynamic string is {:?} ", d_name);
    // let _d_name: String = "".to_string();

    // let str_slice: &str = &d_name[0..5];
    // println!("str_slice is {:?} ", str_slice);

    // // Vectors
    // let mut chars: Vec<char> = Vec::new();
    // chars.insert(0, 'f');
    // chars.insert(1, 'o');
    // chars.insert(2, 'o');
    // chars.push('B');
    // chars.push('a');
    // chars.push('r');

    // // If we directly pass chars to debug, it's ownership will be transfered to dbg
    // // So we need to instead pass by reference (borrow)
    // dbg!(&chars);

    // let rmoved_char = chars.pop().unwrap();
    // dbg!(rmoved_char);
    // dbg!(&chars);

    // // Iterators
    // chars.iter().for_each(|f: &char| print!("{}", f));

    // let chars_2 = vec!['f', 'o', 'o'];
    // dbg!(&chars_2);

    // let collected: String = chars_2.iter().collect();
    // dbg!(collected);

    // for c in chars_2 {
    //     print!("{}", c);
    //     if c == 'o' {
    //         println!(", Test");
    //     }
    // }

    // CLOSURES - Similar to arrow functions in JS
    // let num = 5;
    // let add_num = |x: i32| x + num;
    // let new_num = add_num(7);
    // dbg!(new_num);

    // let test_clo = |x: i32, y: i32| x * y;
    // dbg!(test_clo(2, 8));

    // NUMBER LITERALS AND RAW STRINGS - Revisit
    // println!("Big number is {}",98_9800); 

}
