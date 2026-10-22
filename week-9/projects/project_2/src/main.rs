use std::io::Write;

fn main() {

    // Header vector
    let header = vec!["Student Name", "Matric Number", "Department", "Level"];

    // vectors for rows
    let row_1 = vec!["Oluchi Mardi", "ACC10211111", "Accounting", "300"];
    let row_2 = vec!["Adams Aliyu", "ECO10110101", "Economics", "100"];
    let row_3 = vec!["Shania Bolade", "CSC10328828", "Computer", "200"];
    let row_4 = vec!["Adekunle Gold", "EEE11020202", "Electrical", "200"];
    let row_5 = vec!["Bianca Edemah", "MEE10202001", "Mechanical", "100"];

    // File creation
     let mut file = std::fs::File::create("PAU-SMIS.csv").expect("create failed");
  file.write_all("PAU'S Student Management Information System\n".as_bytes()).expect("write failed");

for i in 0..header.len() {
    if i > 0 { file.write_all(",".as_bytes()).expect("write failed"); }
    file.write_all(header[i].as_bytes()).expect("write failed");
}
file.write_all("\n".as_bytes()).expect("write failed");

for i in 0..row_1.len() {
    if i > 0 { file.write_all(",".as_bytes()).expect("write failed"); }
    file.write_all(row_1[i].as_bytes()).expect("write failed");
}
file.write_all("\n".as_bytes()).expect("write failed");

for i in 0..row_2.len() {
    if i > 0 { file.write_all(",".as_bytes()).expect("write failed"); }
    file.write_all(row_2[i].as_bytes()).expect("write failed");
}
file.write_all("\n".as_bytes()).expect("write failed");

for i in 0..row_3.len() {
    if i > 0 { file.write_all(",".as_bytes()).expect("write failed"); }
    file.write_all(row_3[i].as_bytes()).expect("write failed");
}
file.write_all("\n".as_bytes()).expect("write failed");

for i in 0..row_4.len() {
    if i > 0 { file.write_all(",".as_bytes()).expect("write failed"); }
    file.write_all(row_4[i].as_bytes()).expect("write failed");
}
file.write_all("\n".as_bytes()).expect("write failed");

for i in 0..row_5.len() {
    if i > 0 { file.write_all(",".as_bytes()).expect("write failed"); }
    file.write_all(row_5[i].as_bytes()).expect("write failed");
}
file.write_all("\n".as_bytes()).expect("write failed");

println!("PAU'S Student Management Information System\n");
}