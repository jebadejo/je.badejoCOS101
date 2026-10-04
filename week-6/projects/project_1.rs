use std::io;

fn main() {
    // Display the menu
    println!("The Restaurant Menu");
    println!("P - Poundo Yam / Edinkaiko Soup   N3,200");
    println!("F - Fried Rice & Chicken          N3,000");
    println!("A - Amala & Ewedu Soup            N2,500");
    println!("E - Eba & Egusi Soup              N2,000");
    println!("W - White Rice & Stew             N2,500");
    println!();

    // Read the food type
    println!("Enter food type (P, F, A, E, W): ");
    let mut food = String::new();
    io::stdin().read_line(&mut food).expect("Failed to read");
    let food = food.trim();

    // Read the quantity
    println!("Enter quantity: ");
    let mut qty = String::new();
    io::stdin().read_line(&mut qty).expect("Failed to read");
    let quantity: u32 = qty.trim().parse().expect("Enter a number");

    // Decision on the letter
    let mut price = 0;

    if food == "P" {
        price = 3200;
    } else if food == "F"  {
        price = 3000;
    } else if food == "A" {
        price = 2500;
    } else if food == "E"  {
        price = 2000;
    } else if food == "W"  {
        price = 2500;
    } else {
        println!("Invalid food type");
    }

    // Arithmetic for the total
    let mut total = price * quantity;
    println!("Total charge: N{}", total);

    //  for the discount
    if total > 10000 {
        let discount = total * 5 / 100;
        total = total - discount;
        println!("Discount (5%): N{}", discount);
        println!("Total after discount: N{}", total);
    }
}