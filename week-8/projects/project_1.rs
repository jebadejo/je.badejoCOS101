fn main() {
    // v = APS levels
    let v = vec!["APS 1-2", "APS 3-5", "APS 5-8", "EL1 8-10", "EL2 10-13", "SES"];

    // x = Office Admin
    let x = vec!["Intern", "Administrator", "Senior Administrator", "Office Manager", "Director", "CEO"];
    // city = Academic
    let city = vec!["-", "Research Assistant", "PhD Candidate", "Post-Doc Researcher", "Senior Lecturer", "Dean"];
    // numbers = Lawyer
    let numbers = vec!["Paralegal", "Junior Associate", "Associate", "Senior Associate 1-2", "Senior Associate 3-4", "Partner"];
    // colors = Teacher
    let colors = vec!["Placement", "Classroom Teacher", "Senior Teacher", "Leading Teacher", "Deputy Principal", "Principal"];

    let mut input1 = String::new();
    println!("APS Level Checker");
    println!("Enter your position (e.g. Associate)");
    std::io::stdin().read_line(&mut input1).expect("Failed to read input");
    let new_city: String = input1.trim().parse().expect("Invalid input");

    let mut count = 0;
    for index in 0..6 {
        if x[index] == new_city {
            println!("{} (Office Admin) holds position {}", x[index], v[index]);
            count += 1;
        }
        if city[index] == new_city {
            println!("{} (Academic) holds position {}", city[index], v[index]);
            count += 1;
        }
        if numbers[index] == new_city {
            println!("{} (Lawyer) holds position {}", numbers[index], v[index]);
            count += 1;
        }
        if colors[index] == new_city {
            println!("{} (Teacher) holds position {}", colors[index], v[index]);
            count += 1;
        }
    }

    if count == 0 {
        println!("Position not found");
    }
}