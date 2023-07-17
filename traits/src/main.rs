// use std::fmt;

// trait Money {
//     fn revenue(&self) -> f64;
//     fn profit(&self) -> f64;
// }

// #[derive(Debug,Copy,Clone)]
// struct Company {
//     price: f64,
//     quantity: f64,
//     cost: f64,
// }

// #[derive(Debug,Copy,Clone)]
// struct Firm {
//     revenue: f64,
//     cost: f64,
// }

// impl Money for Company{
//     fn revenue(&self) -> f64{
//         let mut revenue: f64 = (self.price * self.quantity);
//         return revenue;
//     }
//     fn profit(&self) -> f64{
//         let mut profit = (self.price * self.quantity) - self.cost;
//         return profit;
//     }
// }

// impl Money for Firm{
//     fn revenue(&self) -> f64{
//         return self.revenue;
//     }
//     fn profit(&self) -> f64{
//         let mut profit = (self.revenue() - self.cost);
//         return profit;
//     }
// }

// fn main() {
//     let c = Company{price:200.0, quantity:25.0, cost: 50.0};
//     let ans = c.profit();
//     println!("{} dollars", ans);
//     let f = Firm{revenue:5000.0, cost: 50.0};
//     let ans = f.profit();
//     println!("{} dollars", ans);
// }


//1
fn main() {
println!("Hello {:1$}!", "x", 5);
println!("Hello {1:0$}!", 5, "x"); 
println!("Hello {:width$}!", "x", width = 5); 
// println!("Hello {:width$}!", width = 5, "x"); 

let width = 5; 
println!("Hello {:width$}!", "x");

// println!("Hello {:width$}!", "x", width = 5);
}