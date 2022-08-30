fn main() {
    println!("{}", is_palindrome_second_solution(10));
}


fn is_palindrome_second_solution(x: i32) -> bool {
    let mut dividend: i32 = x;
    let mut reverse: i32 = 0;

    if x < 0 {
        return false;
    }
    
    while dividend > 0 {
        let rest: i32 = dividend % 10;
        reverse = reverse * 10 + rest;
        dividend = dividend/10;
    }
    return reverse == x;
}

// fn is_palindrome_first_solution(x: i32) -> bool {
//     let mut digits: Vec<i32>  = vec![];
//     let mut dividend: i32 = x;
    
//     if x < 0 {
//         return false;
//     }
//     while dividend > 0 {
//         let rest: i32 = dividend % 10;
//         digits.push(rest);
//         dividend = dividend/10;
//     }
//     for x in 0..digits.len()/2 {
//         if digits[x] != digits[digits.len()-1-x] {
//             return false;
//         }
//     }
//     return true;
// }
