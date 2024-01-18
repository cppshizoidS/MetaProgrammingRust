// main.rs
mod tuple;
// Import traits and macros from the module
use tuple::{Tuple, AsTupleOfRefs, TupleCons, NonEmptyTuple};

fn main() {
    // Example 1: Creating a tuple and converting it to a tuple list
    let tuple: (i32, f64, char) = (42, 3.14, 'A');
    let tuple_list = tuple.into_tuple_list();
    println!("{:?}", tuple_list);

    // Example 2: Creating a tuple list using the macro
    let tuple_list2 = tuple_list!(1, "hello", 3.14);
    println!("{:?}", tuple_list2);
    // Example 3 : As Tuple of refs
    let tuple_refs: (&i32, &f64, &char) = tuple.as_tuple_of_refs();
    println!("Tuple of References: {:?}", tuple_refs);

    // Example 4: Prepending a value to a tuple using TupleCons
    let new_tuple = <(i32, f64, char) as TupleCons<&str>>::cons("world", tuple);
    println!("{:?}", new_tuple);

    // Example 5: Unconsing a non-empty tuple
    let (head, tail) = new_tuple.uncons();
    println!("Head: {:?}, Tail: {:?}", head, tail);

    // Example 5: Utilizing NonEmptyTuple trait methods
    let non_empty_tuple: (i32, f64, char) = (42, 3.14, 'A');
    let head_value = non_empty_tuple.head();
    let tail_tuple = non_empty_tuple.tail();
    println!("Head Value: {:?}, Tail Tuple: {:?}", head_value, tail_tuple);
}
