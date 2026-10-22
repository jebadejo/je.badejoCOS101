use std::io::Write;

fn main() {

    // Three headers
    let commissioner: [&str; 5] = ["Aigbogun Alamba Daudu", "Murtala Afeez Bendu", "Okorocha Calistus Ogbona", "Adewale Jimoh Akanbi", "Osazuwa Faith Etieye"];
    let ministry: [&str; 5] = ["Internal Affairs", "Justice", "Defense", "Power & Steel", "Petroleum"];
    let zone: [&str; 5] = ["South West", "North East", "South South", "South West", "South East"];

    // Header vector
    let header = vec!["Commissioner", "Ministry", "Zone"];

    // row vectors
    let row_1 = vec![commissioner[0], ministry[0], zone[0]];
    let row_2 = vec![commissioner[1], ministry[1], zone[1]];
    let row_3 = vec![commissioner[2], ministry[2], zone[2]];
    let row_4 = vec![commissioner[3], ministry[3], zone[3]];
    let row_5 = vec![commissioner[4], ministry[4], zone[4]];

    // File creation
    let mut file = std::fs::File::create("The EFCC Merge.csv").expect("create failed");

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

println!("The EFCC Merge\n");    
}