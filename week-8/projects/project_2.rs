fn main() {
    // vector of tuples (name, years)
    let mut city: Vec<(String, i32)> = Vec::new();

    let mut input1 = String::new();
    println!("How many candidates do you want to enter");
    std::io::stdin().read_line(&mut input1).expect("Failed to read input");
    let city_num: i32 = input1.trim().parse().expect("Invalid input");

    for count in 0..city_num {
        let mut input2 = String::new();
        println!("Enter name of candidate {}", count + 1);
        std::io::stdin().read_line(&mut input2).expect("Failed to read input");
        let new_city: String = input2.trim().parse().expect("Invalid input");

        let mut x = String::new();
        println!("Enter years of programming experience for {}", new_city);
        std::io::stdin().read_line(&mut x).expect("Failed to read input");
        let age: i32 = x.trim().parse().expect("Invalid input");

        city.push((new_city, age));
    }

    // count = position of the highest seen so far
    let mut count = 0;
    for index in 0..city.len() {
        if city[index].1 > city[count].1 {
            count = index;
        }
    }

    if city.len() > 0 {
        println!("Highest experience: {} with {} years", city[count].0, city[count].1);
    }
}