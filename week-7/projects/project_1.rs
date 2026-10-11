use std::io;

fn get_pi()->f64 {
    let a:f64 = 22.0;
    let b:f64 = 7.0;
    let c:f64 = a/b;
    return c;
}

fn trapezium_area()->f64 {
    let mut input1 = String::new();
    println!("Enter height:");
    io::stdin().read_line(&mut input1).expect("Failed to read input");
    let height:f64 = input1.trim().parse().expect("Invalid input");

    let mut input2 = String::new();
    println!("Enter base1:");
    io::stdin().read_line(&mut input2).expect("Failed to read input");
    let base1:f64 = input2.trim().parse().expect("Invalid input");

    let mut input3 = String::new();
    println!("Enter base2:");
    io::stdin().read_line(&mut input3).expect("Failed to read input");
    let base2:f64 = input3.trim().parse().expect("Invalid input");

    let area:f64 = height / 2.0 * (base1 + base2);
    return area;
}

fn rhombus_area()->f64 {
    let mut input1 = String::new();
    println!("Enter diagonal1:");
    io::stdin().read_line(&mut input1).expect("Failed to read input");
    let diagonal1:f64 = input1.trim().parse().expect("Invalid input");

    let mut input2 = String::new();
    println!("Enter diagonal2:");
    io::stdin().read_line(&mut input2).expect("Failed to read input");
    let diagonal2:f64 = input2.trim().parse().expect("Invalid input");

    let area:f64 = 1.0 / 2.0 * diagonal1 * diagonal2;
    return area;
}

fn parallelogram_area()->f64 {
    let mut input1 = String::new();
    println!("Enter base:");
    io::stdin().read_line(&mut input1).expect("Failed to read input");
    let base:f64 = input1.trim().parse().expect("Invalid input");

    let mut input2 = String::new();
    println!("Enter altitude:");
    io::stdin().read_line(&mut input2).expect("Failed to read input");
    let altitude:f64 = input2.trim().parse().expect("Invalid input");

    let area:f64 = base * altitude;
    return area;
}

fn cube_surface_area()->f64 {
    let mut input1 = String::new();
    println!("Enter side:");
    io::stdin().read_line(&mut input1).expect("Failed to read input");
    let side:f64 = input1.trim().parse().expect("Invalid input");

    let area:f64 = 6.0 * side * side;
    return area;
}

fn cylinder_volume()->f64 {
    let mut input1 = String::new();
    println!("Enter radius:");
    io::stdin().read_line(&mut input1).expect("Failed to read input");
    let radius:f64 = input1.trim().parse().expect("Invalid input");

    let mut input2 = String::new();
    println!("Enter height:");
    io::stdin().read_line(&mut input2).expect("Failed to read input");
    let height:f64 = input2.trim().parse().expect("Invalid input");

    let volume:f64 = get_pi() * radius * radius * height;
    return volume;
}

fn main(){
    println!("Welcome to the Shape Calculator!");
    println!("1. Trapezium, area");
    println!("2. Rhombus, area");
    println!("3. Parallelogram, area");
    println!("4. Cube, surface area");
    println!("5. Cylinder, volume");
    println!("Enter your choice:");

    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read input");
    let choice:i32 = input.trim().parse().expect("Invalid input");

    if choice >= 1 && choice <= 1
    {
        let result:f64 = trapezium_area();
        println!("Area of trapezium is {}",result);
    }
    else
    {
        if choice >= 2 && choice <= 2
        {
            let result:f64 = rhombus_area();
            println!("Area of rhombus is {}",result);
        }
        else
        {
            if choice >= 3 && choice <= 3
            {
                let result:f64 = parallelogram_area();
                                println!("Area of parallelogram is {}",result);
            }
            else
            {
                if choice >= 4 && choice <= 4
                {
                    let result:f64 = cube_surface_area();
                    println!("Surface area of cube is {}",result);
                }
                else
                {
                    if choice >= 5 && choice <= 5
                    {
                        let result:f64 = cylinder_volume();
                        println!("Volume of cylinder is {}",result);
                    }
                    else
                    {
                        println!("Invalid choice");
                    }
                }
            }
        }
    }
}