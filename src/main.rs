use csv::ReaderBuilder;
use std::fs::File;
use std::io::copy;
use std::thread::sleep;
use std::time::Duration;
use sysinfo::{CpuExt, System, SystemExt};
use tokio::runtime::Runtime;

async fn download_file() {
    let url = "https://www.data.gouv.fr/fr/datasets/r/c13d05e5-9e55-4d03-bf7e-042a2ade7e49";
    let file_name = "tarif-kwh.csv";

    let response = reqwest::get(url).await.unwrap();

    let mut output = File::create(file_name).unwrap();
    copy(&mut response.text().await.unwrap().as_bytes(), &mut output).unwrap();

    println!("File downloaded to {}", file_name);
}

async fn get_rate() -> f32 {
    download_file().await;
    let file = File::open("tarif-cre.csv").unwrap();

    let mut rdr = ReaderBuilder::new().delimiter(b';').from_reader(file);

    let rates = rdr
        .records()
        .map(|result| {
            let record = result.unwrap();
            let record: Vec<&str> = record.iter().collect();

            if record[6].is_empty() {
                return 0.0;
            }
            let rate = record[6].replace(",", ".").parse::<f32>().unwrap();
            return rate;
        })
        .collect::<Vec<f32>>();

    for i in (0..rates.len()).rev() {
        if rates[i] != 0.0 {
            println!("Rate: {}€", rates[i]);
            return rates[i];
        }
    }

    return 0.0;
}

async fn main_async() {
    let mut system = System::new_all();

    if !System::IS_SUPPORTED {
        println!("Current power usage is not supported on this platform.");
        return;
    }

    let rate = get_rate().await;
    let mut total_cost = 0.0;

    loop {
        system.refresh_all();
        let cpus = system.cpus();
        let mut total_power_per_second = 0.0;
        for cpu in cpus {
            total_power_per_second += cpu.cpu_usage();
        }

        let cost_per_second = (total_power_per_second / 1000.0) * rate;
        total_cost = total_cost + cost_per_second;
        println!(
            "Total power usage by CPU {}W and cost: {}€",
            total_power_per_second, cost_per_second
        );
        println!("Total cost: {}€", total_cost);
        println!("----------------------------------------");

        sleep(Duration::from_secs(1));
    }
}

fn main() {
    let rt = Runtime::new().unwrap();
    rt.block_on(main_async());
}
