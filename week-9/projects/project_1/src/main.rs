
use std::io::Write;

fn main() {
    let header = "Lager, Stout, Non-Alcoholic\n";
    let row_1 = "33 Export, Legend, Maltina\n";
    let row_2 = "Desparados, Turbo King, Amstel Malta\n";
    let row_3 = "Goldberg, Williams, Malta Gold\n";
    let row_4 = "Gulder, , Fayrouz\n";
    let row_5 = "Heineken, , \n";
    let row_6 = "Star, , \n";

    let mut file = std::fs::File::create("Nigerian Brewerie.csv").expect("create failed");
    file.write_all("Nigerian Breweries\n"
        .as_bytes()).expect("write failed");
    file.write_all(header.as_bytes()).expect("write failed");
    file.write_all(row_1.as_bytes()).expect("write failed");
     file.write_all(row_2.as_bytes()).expect("write failed");
      file.write_all(row_3.as_bytes()).expect("write failed");
       file.write_all(row_4.as_bytes()).expect("write failed");
        file.write_all(row_5.as_bytes()).expect("write failed");
         file.write_all(row_6.as_bytes()).expect("write failed");
    println!("\nDrink Categories For Nigerian Breweries");
}