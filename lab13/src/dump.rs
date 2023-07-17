use std::fs::File;
use std::io::Read;

#[derive(Copy, Clone, Debug)]
struct GPS {
    lat: f64,
    long: f64,
}

const COMMA:char = ',';
const NEW_LINE:char = '\n';

fn read_file()-> Vec<GPS>{
    let mut input = String::new();
	File::open("GPSA.csv").unwrap().read_to_string(&mut input).unwrap();
	
    let mut gps_vec : Vec<GPS> = vec!();
    let lines: Vec<&str> = input.split(NEW_LINE).collect();
    let n_lines = lines.len();
    for j in 0..n_lines {
        let t_set: Vec<&str> = lines[j].split(COMMA).collect();

        let x:f64 = t_set[0].trim().parse().unwrap();
        let y:f64 = t_set[1].trim().parse().unwrap();
        let gps = GPS{lat:x, long:y};
        gps_vec.push(gps.clone());
    }
    println!("{:?}", gps_vec);
    gps_vec
}

fn find_mean(gps_vec:&Vec<GPS>) -> (f64, f64){
    let mut sum_lat:f64 = 0.0;
    let mut sum_long:f64 = 0.0;

    for gps in gps_vec{
        sum_lat += gps.lat;
        sum_long += gps.long;
    }
    let mean_lat = sum_lat / gps_vec.len() as f64;
    let mean_long = sum_long / gps_vec.len() as f64;

    (mean_lat, mean_long)
}

const TEMP_MIN:f64 = 10000.0;
const TEMP_MAX:f64 = 0.0;

fn find_min(gps_vec:&Vec<GPS>)-> (f64, f64){

    let mut min_lat = TEMP_MIN;
    let mut min_long = TEMP_MIN;
    for gps in gps_vec{
        if gps.lat <= min_lat{
            min_lat = gps.lat;
        }

        if gps.long <= min_long{
            min_long = gps.long;
        }

    }

    (min_lat, min_long)
}

fn find_max(gps_vec:&Vec<GPS>)-> (f64, f64){

    let mut max_lat = TEMP_MAX;
    let mut max_long = TEMP_MAX;
    for gps in gps_vec{
        if gps.lat >= max_lat{
            max_lat = gps.lat;
        }

        if gps.long >= max_long{
            max_long = gps.long;
        }

    }

    (max_lat, max_long)
}

fn find_sd(gps_vec:&Vec<GPS>, mean:(f64, f64)) -> (f64, f64){
    let mut sum_lat:f64 = 0.0;
    let mut sum_long:f64 = 0.0;
    for gps in gps_vec{
        sum_lat += (gps.lat - mean.0).powf(2.0);
        sum_long += (gps.long - mean.1).powf(2.0);

    }
    let sd_lat: f64 = (sum_lat / gps_vec.len() as f64).sqrt();
    let sd_long: f64 = (sum_long / gps_vec.len() as f64).sqrt();

    (sd_lat, sd_long)
}

const LAT_MET: f64 = 111139.0;
const LONG_MET: f64 = 107963.0;

fn standard_error(gps_vec: &Vec<GPS>, sd:(f64, f64)) -> (f64, f64) {
    let lat_m = sd.0 * LAT_MET;
    let long_m = sd.1 * LONG_MET;

    let lat_se = lat_m ;
    let long_se = long_m ;

    (lat_se, long_se)
}

const NUMBER_OF_BINS:f64 = 10.0;


fn gen_unique_vec_lat(gps_vec:&Vec<GPS>) -> Vec<f64>{
    let mut vec_lat: Vec<f64> = vec!();

    let mut flag = 0;
    for i in 0..gps_vec.len(){
        for j in 0..vec_lat.len(){
            if gps_vec[i].lat != vec_lat[j] {
                flag = 0;
            } else {
                flag = 1;
                break;
            }
        }
        if (flag == 0) {
            vec_lat.push(gps_vec[i].lat);
        }
    }
    vec_lat.sort_by(|a, b| a.partial_cmp(b).unwrap());
    vec_lat
}

fn gen_unique_vec_long(gps_vec:&Vec<GPS>) -> Vec<f64>{
    let mut vec_long: Vec<f64> = vec!();

    let mut flag = 0;
    for i in 0..gps_vec.len(){
        for j in 0..vec_long.len(){
            if gps_vec[i].long != vec_long[j] {
                flag = 0;
            } else {
                flag = 1;
                break;
            }
        }
        if (flag == 0) {
            vec_long.push(gps_vec[i].long);
        }
    }
    vec_long.sort_by(|a, b| a.partial_cmp(b).unwrap());
    vec_long
}

fn gen_count_lat(gps_vec:&Vec<GPS>, lat_vec:&Vec<f64>) -> Vec<u64>{
    let mut count_lat: Vec<u64> = vec!();
    
    for i in 0..lat_vec.len(){
        let mut count = 0;
        for j in 0..gps_vec.len(){
            if lat_vec[i] == gps_vec[j].lat{
                count += 1;
            }
        }
        count_lat.push(count)
    }
    count_lat
}

fn gen_count_long(gps_vec:&Vec<GPS>, long_vec:&Vec<f64>) -> Vec<u64>{
    let mut count_long: Vec<u64> = vec!();
    
    for i in 0..long_vec.len(){
        let mut count = 0;
        for j in 0..gps_vec.len(){
            if long_vec[i] == gps_vec[j].long{
                count += 1;
            }
        }
        count_long.push(count)
    }
    count_long
}

// fn gen_histogram(main_vec:&Vec<f64>, count_vec:&Vec<u64>) {

//     for i in 0..main_vec.len(){
//         print!("{:?} ", main_vec[i]);
//         for i in 0..count_vec[i]{
//             print!("*");
//         }
//         println!();
//         // println!("{:?}", count_lat[i]);
//     }
//     println!();
// }

fn gen_bin(min_main:f64, bin_step_main: &f64) -> Vec<f64>{
    let mut step_main = min_main;
    let mut bin_main:Vec<f64> = vec!();
    for i in 0..10{
        bin_main.push(step_main);
        step_main += bin_step_main;
    }
    bin_main
}

fn gen_histogram(bin_main: &Vec<f64>, main_vec: &Vec<f64>, count_main: &Vec<u64>, bin_step_main: &f64){
    let mut count_bin_main: Vec<u64> = vec!();

    for i in 0..bin_main.len()-1{
        let mut count = 0;
        for j in 0..main_vec.len(){
            
            if bin_main[i] < main_vec[j] && main_vec[j] <= bin_main[i+1]{
                count += count_main[j];
            }
            else {
                continue
            }
        }
        count_bin_main.push(count);
    }

    let mut count = 0;
    for j in 0..main_vec.len(){
        if main_vec[j] > bin_main[bin_main.len()-1] {
            count += count_main[j];
        }
        else {
            continue
        }
    }
    count_bin_main.push(count);

    for i in 0..bin_main.len(){
        print!("{:.7} | ", bin_main[i]);
        for j in 0..count_bin_main[i]{
            print!("*");
        }
        println!();
    }
}

fn main(){
    let gps_vec = read_file();
    let mean = find_mean(&gps_vec);
    let min = find_min(&gps_vec);
    let max = find_max(&gps_vec);
    let sd = find_sd(&gps_vec, mean);
    let se = standard_error(&gps_vec, sd);

    let vec_lat = gen_unique_vec_lat(&gps_vec);
    let vec_long = gen_unique_vec_long(&gps_vec);
    let count_lat = gen_count_lat(&gps_vec, &vec_lat);
    let count_long = gen_count_long(&gps_vec, &vec_long);
    
    let range_lat = max.0 - min.0;
    let bin_step_lat = range_lat / NUMBER_OF_BINS;
    let range_long = max.1 - min.1;
    let bin_step_long = range_long / NUMBER_OF_BINS;

    let bin_lat = gen_bin(min.0, &bin_step_lat);
    let bin_long = gen_bin(min.1, &bin_step_long);

    println!("mean: {:?}", mean);
    println!("min: {:?}", min);
    println!("max: {:?}", max);
    println!("SD: {:?}", sd);
    println!("SE: {:?}", se);
    println!("{:?}\n", vec_lat);
    println!("{:?}\n", vec_long);
    println!("{:?}\n", count_lat);
    println!("{:?}", count_long);
    println!("range of latitude: {}", range_lat);
    println!("bin of latitide: {:?}", bin_lat);

    println!();
    println!("== LATITUDE ==");
    gen_histogram(&bin_lat, &vec_lat, &count_lat, &bin_step_lat);
    println!();
    println!("== LONGITUDE ==");
    gen_histogram(&bin_long, &vec_long, &count_long, &bin_step_long);

    // // generate histogram for latitude
    // gen_histogram(&vec_lat, &count_lat);
    
    // // generate histogram for longitude
    // gen_histogram(&vec_long, &count_long);
}