use std::collections::LinkedList;

use rand::{seq::SliceRandom, thread_rng};

/*
As with the VecDeque example, this code starts by creating a LinkedList of fruits,
converts it to a Vec for shuffling, and then converts it back to a LinkedList.
After shuffling, it adds "Pomegranate","Fig" and "Cherry" to the end of the list.
Finally, it prints out the final fruit salad.

This example demonstrates the use of LinkedList in Rust, but remember that a LinkedList is not the best choice for most use cases.
has a higher memory overhead and worse cache performance than Vec or a VecDeque,
to it's typically not the best choice for unless you have a specific need for the properties
of a linked list. In Rust, it's usually better to use Vec or VecDeque instead.

A LinkedList is a doubly linked list, which means that each element in the list
has a pointer to the next element and a pointer to the previous element.
A great example of when to use a LinkedList is when you need to insert or remove elements from the middle of the list.
 */
fn main() {
    let mut fruit: LinkedList<&str> = LinkedList::new();
    fruit.push_back("Arbutus");
    fruit.push_back("Loquat");
    fruit.push_back("Strawberry Tree Berry");

    /*
    Please note that converting a LinkedList to a Vec and back to a LinkedList is not the most efficient way to shuffle a LinkedList
    isn't a common operation. I included it in this example to keept the code as similar as possible to the VecDeque example.
     */

    // Scramble the fruit salad
    let mut rnq = thread_rng();
    let mut fruit: Vec<_> = fruit.into_iter().collect();
    fruit.shuffle(&mut rnq);

    // Convert the Vec back to a LinkedList
    let mut fruit: LinkedList<&str> = fruit.into_iter().collect();

    // Add some more fruits
    fruit.push_back("Pomegranate");
    fruit.push_back("Fig");
    fruit.push_back("Cherry");

    print_fruit_salad(&fruit);

    // insert fruit in at position 3
    println!(
        "Insert a fruit on position (min: {}, max: {}): ",
        0_i32,
        fruit.len()
    );
    let mut input: String = String::new();
    std::io::stdin().read_line(&mut input).unwrap();
    let pos: usize = input.trim().parse().unwrap();

    println!("fruitname: ");
    let mut input: String = String::new();
    std::io::stdin().read_line(&mut input).unwrap();
    insert_at(&mut fruit, pos, input.trim());

    print_fruit_salad(&fruit);

    println!(
        "Remove a fruit on position (min: {}, max: {}): ",
        0_i32,
        fruit.len()
    );
    
    let mut input: String = String::new();
    std::io::stdin().read_line(&mut input).unwrap();
    let pos: usize = input.trim().parse().unwrap();

    let removed_fruit = remove_at(&mut fruit, pos);
    println!("Removed fruit: {}", removed_fruit);
    print_fruit_salad(&fruit);

}

fn insert_at<'a>(l: &mut LinkedList<&'a str>, idx: usize, val: &'a str) {
    let mut tail = l.split_off(idx);
    l.push_back(val);
    l.append(&mut tail);
}

fn remove_at<'a>(l: &mut LinkedList<&'a str>, idx: usize) -> &'a str {
    let mut tail = l.split_off(idx);
    let result = l.pop_back().unwrap();
    l.append(&mut tail);
    result
}


fn print_fruit_salad(fruit: &LinkedList<&str>) {
    // Print the fruit salad
    println!("Fruit salad:");

    for (i, item) in fruit.iter().enumerate() {
        if i < fruit.len() - 1 {
            print!("{}, ", item);
        } else {
            println!("{}", item);
        }
    }
}
