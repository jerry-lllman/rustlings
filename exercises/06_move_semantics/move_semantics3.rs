// move_semantics3.rs
//
// Make me compile without adding new lines -- just changing existing lines! (no
// lines with multiple semicolons necessary!)
//
// Execute `rustlings hint move_semantics3` or use the `hint` watch subcommand
// for a hint.

// I AM NOT DONE

#[test]
fn main() {
    let mut vec0 = vec![22, 44, 66];

    let vec1 = fill_vec(&mut vec0);

    assert_eq!(vec1, vec![22, 44, 66, 88]);
}

fn fill_vec(vec:&mut Vec<i32>) -> Vec<i32> {
    vec.push(88);

    // vec.clone()
    vec.to_vec()
}

 
// #[test]
// fn main() {
//     let vec0 = vec![22, 44, 66];

//     let mut vec1 = fill_vec(vec0);

//     assert_eq!(vec1, vec![22, 44, 66, 88]);
// }

// fn fill_vec(vec: Vec<i32>) -> Vec<i32> {
//     let mut vec = vec.clone();
//     vec.push(88);

//     vec
// }


// #[test]
// fn main() {
//     let vec0 = vec![22, 44, 66];

//     let mut vec1 = fill_vec(vec0);

//     assert_eq!(vec1, vec![22, 44, 66, 88]);
// }

// fn fill_vec(vec: Vec<i32>) -> Vec<i32> {
//     let mut vec = vec.into_iter().collect::<Vec<i32>>();
//     vec.push(88);

//     vec
// }

