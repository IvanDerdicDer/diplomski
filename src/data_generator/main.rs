use diplomski_projekt::*;
use rust_decimal::Decimal;
use anyhow::{Result, Error};
use rand::Rng;
use chrono::{NaiveDate, NaiveDateTime, NaiveTime};
use thiserror::Error;


#[derive(Error, Debug)]
enum GeneratorErrors {
    #[error("Can not generate date.")]
    CanNotGenerateDate,
    #[error("Can not generate time.")]
    CanNotGenerateTime
}


fn id_generator() -> Result<String> {
    let mut rng = rand::thread_rng();
    Ok(rng.gen_range(1..1001).to_string())
}


fn price_generator() -> Result<String> {
    let mut rng = rand::thread_rng();
    Ok(
        rng.gen_range(0..1000).to_string()
            + "."
            + rng.gen_range(0..100).to_string().as_str()
    )
}


fn date_generator() -> Result<String> {
    let mut rng = rand::thread_rng();
    let year = rng.gen_range(2000..2050);
    let month = rng.gen_range(1_u32..13);
    let day = rng.gen_range(1_u32..29);

    NaiveDate::from_ymd_opt(year, month, day)
        .ok_or(Error::from(GeneratorErrors::CanNotGenerateDate))
        .map(|x| x.format("%Y-%m-%d").to_string())
}


fn datetime_generator() -> Result<String> {
    let mut rng = rand::thread_rng();
    let year = rng.gen_range(2000..2050);
    let month = rng.gen_range(1_u32..13);
    let day = rng.gen_range(1_u32..29);

    let hour = rng.gen_range(0_u32..24);
    let minute = rng.gen_range(0_u32..60);
    let second = rng.gen_range(0_u32..60);
    
    let date = NaiveDate::from_ymd_opt(year, month, day)
        .ok_or(Error::from(GeneratorErrors::CanNotGenerateDate))?;
    let time = NaiveTime::from_hms_opt(hour, minute, second)
        .ok_or(Error::from(GeneratorErrors::CanNotGenerateTime))?;
    
    Ok(NaiveDateTime::new(date, time).format("%Y-%m-%d %H:%M:%S").to_string())
}


fn main() {
    println!("Test")
}