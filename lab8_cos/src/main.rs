use std::f64::consts::PI;

const N: usize =  10; // numbers of lut
const M: usize = 101; // numbers of test_value

// convert degree to radian
fn cal_rad(angle: f64) -> f64 {
    return angle * PI / 180.0;
}

// find reference angle
fn cal_deg(mut angle: f64) -> f64 {
    if angle <= 90.0 {
        angle = angle;
    } else if angle > 90.0 && angle <= 180.0 {
        angle = 180.0 - angle;
    } else if angle > 180.0 && angle <= 270.0 {
        angle = (angle - 180.0) ;
    } else if angle > 270.0 && angle < 360.0 {
        angle = 360.0 - angle;
    }

    return angle;
}

fn lut_sin(lut: [f64; N],mut x: f64) -> f64 {
    let mut lower_s: f64 = 0.0;
    let mut upper_s: f64 = 0.0;
    let mut num_sin: f64 = 0.0;
    let mut sign: f64 = 1.0;

    let mut lut_rad: [f64; N] = [0.0; N];
    let mut lut_lib: [f64; N] = [0.0; N];

    for i in 0..N {
        lut_rad[i] = cal_rad(lut[i]);
        lut_lib[i] = lut_rad[i].sin();
    }

    // find sign (if 180 < x < 360 -> -sin(x))
    if x >= 360.0{
        x = x % 360.0 ;
    }
    if x > 180.0 && x < 360.0{
        sign = -1.0;
    }
  
    let mut x_rad = cal_deg(x).to_radians();

    let mut a = 0;
    for i in 0..N {
        if i > 0 {
            if x_rad <= lut_rad[i] {
                upper_s = lut_rad[i];
                a = i;
                lower_s = lut_rad[i - 1];
                break;
            }
        }
    }

    num_sin =
        lut_lib[a - 1] + (lut_lib[a] - lut_lib[a - 1]) * (x_rad - lower_s) / (upper_s - lower_s);
    num_sin * sign
}

fn lut_cos(lut: [f64; N],mut x: f64) -> f64 {
    let mut lower_s: f64 = 0.0;
    let mut upper_s: f64 = 0.0;
    let mut num_cos: f64 = 0.0;
    let mut sign: f64 = 1.0;

    let mut lut_lib: [f64; N] = [0.0; N];
    let mut lut_rad: [f64; N] = [0.0; N];

    for i in 0..N {
        lut_rad[i] = lut[i] * PI / 180.0;
        lut_lib[i] = (lut_rad[i]).cos();
    }

    // find sign (if 90 < x < 270 -> -cos(x))
    if x >= 360.0{
        x = x % 360.0 ;
    }
    if x > 90.0 && x < 270.0{
        sign = -1.0;
    }

    let mut x_rad = cal_deg(x).to_radians();

    let mut a = 0;
    for j in 0..N {
        if j > 0 {
            if x_rad <= lut_rad[j] {
                upper_s = lut_rad[j];
                a = j;
                lower_s = lut_rad[j - 1];
                break;
            }
        }
    }

    num_cos = lut_lib[a - 1] + (lut_lib[a] - lut_lib[a - 1]) * (x_rad - lower_s) / (upper_s - lower_s);
    num_cos * sign
}

// create lut
fn gen_lut() -> [f64; N]{
    let mut lut: [f64; N] = [0.0; N];
    let mut num: f64 = 0.0;
    for i in 0..N {
        lut[i] = num;
        num += 90.0 / (N - 1) as f64;
    }
    lut
}

// create test value
fn gen_test_value() -> [f64; M]{
    let mut test_value: [f64; M] = [0.0; M];
    let mut test: f64 = 0.0;
    for i in 0..M {
        test_value[i] = test;
        test += 3.75 as f64;
    }
    test_value
}

fn main() {
    let mut lut = gen_lut();
    let mut test_values = gen_test_value();
   
    // display table
    println!("");
    println!("angle\t\t\t| sin_from_library\t|\tsin_from_interpolation\t|\tcos_from_library\t|\tcos_from_interpolation");
    println!("");
    for i in 0..M {
        let angle = test_values[i];
        let sin_from_lib = cal_rad(test_values[i]).sin();
        let sin_from_lut = lut_sin(lut, test_values[i]);
        let cos_from_lib = cal_rad(test_values[i]).cos();
        let cos_from_lut = lut_cos(lut, test_values[i]);
        println!(
            "{:20.16}\t|{:20.16}\t|\t{:20.16}\t|\t{:20.16}\t|\t{:20.16}",
            angle, sin_from_lib, sin_from_lut, cos_from_lib, cos_from_lut
        );
    }
}