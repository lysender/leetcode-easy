fn main() {
    solution_add_to_array_form_of_integer();
}

fn solution_add_to_array_form_of_integer() {
    assert_eq!(add_to_array_form_of_integer(vec![1, 2, 0, 0], 34), vec![1, 2, 3, 4]);
    assert_eq!(add_to_array_form_of_integer(vec![2, 7, 4], 181), vec![4, 5, 5]);
    assert_eq!(add_to_array_form_of_integer(vec![2, 1, 5], 806), vec![1, 0, 2, 1]);
    assert_eq!(add_to_array_form_of_integer(vec![9, 9, 9, 9, 9, 9, 9, 9, 9, 9], 1), vec![1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0]);
    assert_eq!(
    add_to_array_form_of_integer(vec![3,5,8,6,9,7,8,3,8,5,4,1,6,7,4,1,0,1,7,7,1,5,3,2,9,3,4,1,0,5,8,6,9,9,4,8,7,0,2,8,2,4,7,0,4,4,3,7,2,2], 142),
    vec![3,5,8,6,9,7,8,3,8,5,4,1,6,7,4,1,0,1,7,7,1,5,3,2,9,3,4,1,0,5,8,6,9,9,4,8,7,0,2,8,2,4,7,0,4,4,3,8,6,4]
);
}

fn add_to_array_form_of_integer(num: Vec<i32>, k: i32) -> Vec<i32> {
    // Convert k into an array of digits too
    // Add num and k digit by digit from right to left
    // Carry over values to the next digit if >= 10
    // Add more digit if necessary

    // Reverse the numbers
    let mut numbers = num.clone();
    numbers.reverse();

    // Convert k to array and reverse it too
    let mut ks: Vec<i32> = Vec::new();
    for digit_char in k.to_string().chars() {
        let digit: i32 = digit_char.to_string().parse().unwrap();
        ks.push(digit);
    }
    ks.reverse();

    // Start adding digit by digit
    let mut carry_over: i32 = 0;
    let mut counter: usize = 0;
    let mut result: Vec<i32> = Vec::new();

    while counter < numbers.len() || counter < ks.len() {
        let a: i32 = match numbers.get(counter) {
            Some(val) => *val,
            None => 0,
        };
        let b: i32 = match ks.get(counter) {
            Some(val) => *val,
            None => 0,
        };
        let sum: i32 = a + b + carry_over;
        if sum >= 10 {
            let diff = sum - 10;
            carry_over = 1;
            result.push(diff);
        } else {
            carry_over = 0;
            result.push(sum);
        }
        counter += 1;
    }

    if carry_over > 0 {
        result.push(carry_over);
    }

    // Reverse it back
    result.reverse();
    return result;
}
