fn main() {
   let toshiba:f64 = 450000.00;
   let mac:f64 = 1500000.00;
   let hp:f64 = 750000.00;
   let dell:f64 = 2850000.00;
   let acer:f64 = 250000.00;

   //sum of sales
   let sum = toshiba + mac + hp + dell + acer;
   println!("Sum is {}", sum);

   //average of sales
   let average = sum / 5.0;
   println!("Average is {}", average );
}