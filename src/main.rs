mod abstract_syntax_tree;

fn main() {
    let vec = vec![1, 5, 13, 40];
    let mut iter = vec.iter().peekable();

    // Peeking at the first element without advancing
    println!("Peek: {:?}", iter.peek()); // Outputs: Some(&1)

    // Using next() to advance the iterator
    println!("Next: {:?}", iter.next()); // Outputs: Some(1)

    // Peeking at the second element now
    println!("Peek: {:?}", iter.peek()); // Outputs: Some(&2)
    println!("Next: {:?}", iter.next()); // Outputs: Some(2)

    // Continue with the rest of the iteration
    while let Some(val
    ) = iter.next() {
        println!("Value: {}", val);
    }
}
