// move_semantics2.rs
// Make me compile without changing line 13 or moving line 10!
// Execute `rustlings hint move_semantics2` or use the `hint` watch subcommand for a hint.

fn main() {
    let vec0 = Vec::new();

    // easy way, make clone of vec0 : let mut vec1 = fill_vec(vec0.clone());
    let mut vec1 = fill_vec_borrow(&vec0);

    // Do not change the following line!
    println!("{} has length {} content `{:?}`", "vec0", vec0.len(), vec0);

    vec1.push(88);

    println!("{} has length {} content `{:?}`", "vec1", vec1.len(), vec1);

    // mutable case
    /*
    let mut vec0 = Vec::new();

    fill_vec_mutably(&mut vec0);

    println!("{} has length {} content `{:?}`", "vec0", vec0.len(), vec0);

    vec0.push(88);

    println!("{} has length {} content `{:?}`", "vec1", vec0.len(), vec0);
    */
}

fn fill_vec(vec: Vec<i32>) -> Vec<i32> {
    let mut vec = vec;

    vec.push(22);
    vec.push(44);
    vec.push(66);

    vec
}

fn fill_vec_borrow(vec: &Vec<i32>) -> Vec<i32> {
    let mut vec_filled = vec.clone();

    vec_filled.push(22);
    vec_filled.push(44);
    vec_filled.push(66);

    vec_filled
}

fn fill_vec_mutably(vec: &mut Vec<i32>) {
    vec.push(22);
    vec.push(44);
    vec.push(66);
}
