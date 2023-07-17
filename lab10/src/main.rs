
use std::time::{Duration, Instant};
use std::f64::consts::PI;

const MAX_POWER: u64 = 12;
const PI2: f64 = 2.0*PI;
const BASE: u64 = 4;

// fn fact(n: i32) -> i32{
//     if (n == 1 || n == 0){
//         1
//     }
//     else{
//         n * fact(n-1)
//     }
// }

fn time_diff_secs( t0:Instant ) -> f64 {
	let duration = t0.elapsed();
        let d_secs = (duration.as_secs() as f64)
           + (duration.subsec_nanos() as f64) * 1e-9;
	d_secs
}

fn empty_loop( n: u64 ) -> f64 {
  let mut sum: f64 = 0.0;
  for _j in 0..n {
    let x:f64 = (rand::random::<f64>())*PI2;
    sum += x;
    }
  let mean = sum / (n as f64);
  mean
}

fn real_loop( n: u64 ) -> f64 {
  let mut sum: f64 = 0.0;
  for _j in 0..n {
    let x:f64 = (rand::random::<f64>())*PI2;
    sum += x.sin();
    }
  let mean = sum / (n as f64);
  mean
}

// Report times in a consistent way
fn print_time( t_secs: f64, n: u64 ) {
  print!("T {:8.3} s t {:6.1} ns ", t_secs, t_secs*1.0e9/(n as f64) );
}

// fn sin_series( x: f64, n: u64 ) -> f64 {
//   let x2: f64 = x*x;
//     let mut sum = 0.0;
//     let mut numer = 1.0;
//     let mut deno = 1;
//     // let mut expo = 1.0;
//     let mut xn = x;
//     let mut f = 2;
//     let mut sign = 1.0;
//     let mut t = x;
//     for i in 0..n{
//         // numer = x.powf(expo);
//         // //println!("numer = {}", numer);
//         // sum += sum;
//         // expo += 2.0;
//         // for j in 1..=expo as i32{
//         //     mul *= j as f64;
//         // }
//         // deno = mul;
//         // sum = numer / deno;
//         // // deno = (expo as u32).factorial();
//         // sign = -sign;
//         // println!("fn {}, {}, {}",numer, deno, sum);
//         sum += sum;
//         xn = xn * x2;
//         deno = deno * f * (f+1);
//         f += 2;
//         sign = -sign;


//     }
//     sum as f64

// }

fn sin_series(x: f64, n:u64) -> f64{
    let x2: f64 = x * x;
    let mut divisor: u64 = 1;
    let mut sum: f64 = 0.0;
    let mut xn = x;
    let mut f: u64 = 2;
    // let mut sign: bool = true;
    let mut sign = 1;
    let mut term = x;
    for _j in 0..n{
    //     if sign{
    //         sum += term;
    //     } else {
    //         sum -= term;
        // }
        sum += term;
        xn = xn * x2;
        divisor = divisor * f * (f+1);
        f += 2;
        term = xn/(divisor as f64);
        // sign = !sign;
        sign = - sign;
    }
    sum
}

fn main() {
    println!("Accurate timing");
    let mut n:u64 = 1;
    for j in 1..MAX_POWER {
        let mut t0 = Instant::now();
        let mut mean = empty_loop( n );
        let loop_time = time_diff_secs( t0 );
        print!("{:3} {:9} ", j, n );
        print_time( loop_time, n );
        print!("m {:8.4} ", mean );
      
// Now time the loop with some real code in it
    println!("");
        t0 = Instant::now();
        let mut mean = real_loop( n );
        let real_time = time_diff_secs( t0 );
        print!("{:3} {:9} ", j, n );
        print_time( real_time, n );
        let difftime = real_time - loop_time;
        print!("DIFF = ");
        print_time(difftime, n);
        print!("m {:8.4} ", mean );


    
// // .....................

      n = n * BASE;
      
      println!("");
      println!("");

      }
  

// Test your sin function for various numbers of terms
   for n_angles in 0..8 {
     let xd: f64 = 15.0 * (n_angles as f64);
     let x =  PI * (xd / 180.0 );
     print!("{:5.1} {:8.4} {:10.6} ", xd, x,  x.sin() );
     for k in 1..10{
        let sx = sin_series(x,k);
        print!("sx {:10.6}", sx)
     }
// Now test your sin_series function for various numbers of terms
// for n_terms ..........
// ... print each result as you go ...
     println!("");
   }
}

// fn main(){
//     let mut x = 1.0;
//     let mut n = 1;
//     for i in 0..2{
//         x = sin_series(PI / 6.0, n);
//         print!("{}", x);
//         n *= 4
//     }
// }