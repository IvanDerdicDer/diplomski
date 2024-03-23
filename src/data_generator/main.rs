use diplomski_projekt::*;
use rust_decimal::Decimal;
use anyhow::{Result, Error};
use rand::Rng;
use rand::seq::SliceRandom;
use chrono::{NaiveDate, NaiveDateTime, NaiveTime};
use thiserror::Error;


#[derive(Error, Debug)]
enum GeneratorErrors {
    #[error("Can not generate date.")]
    CanNotGenerateDate,
    #[error("Can not generate time.")]
    CanNotGenerateTime,
    #[error("Can not generate name.")]
    CanNotGenerateName,
}


fn id_generator() -> Result<String> {
    let mut rng = rand::thread_rng();
    Ok(rng.gen_range(1..=1_000).to_string())
}


fn price_generator() -> Result<String> {
    let mut rng = rand::thread_rng();
    Ok(
        rng.gen_range(0..1_000).to_string()
            + "."
            + rng.gen_range(0..100).to_string().as_str()
    )
}


fn net_worth_generator() -> Result<String> {
    let mut rng = rand::thread_rng();
    Ok(
        rng.gen_range(0..1_000_000_000).to_string()
            + "."
            + rng.gen_range(0..1_000_000).to_string().as_str()
    )
}


fn datetime_generator() -> Result<String> {
    let mut rng = rand::thread_rng();
    let year = rng.gen_range(2000..2050);
    let month = rng.gen_range(1_u32..=12);
    let day = rng.gen_range(1_u32..=28);

    let hour = rng.gen_range(0_u32..=23);
    let minute = rng.gen_range(0_u32..=59);
    let second = rng.gen_range(0_u32..=59);

    let date = NaiveDate::from_ymd_opt(year, month, day)
        .ok_or(Error::from(GeneratorErrors::CanNotGenerateDate))?;
    let time = NaiveTime::from_hms_opt(hour, minute, second)
        .ok_or(Error::from(GeneratorErrors::CanNotGenerateTime))?;

    Ok(NaiveDateTime::new(date, time).format("%Y-%m-%d %H:%M:%S").to_string())
}


fn generate_first_name() -> Result<String> {
    let first_names = vec![
        "Emma", "Liam", "Olivia", "Noah", "Ava", "Oliver", "Isabella", "William", "Sophia", "Elijah",
        "Charlotte", "James", "Amelia", "Benjamin", "Mia", "Lucas", "Harper", "Henry", "Evelyn", "Alexander",
        "Abigail", "Michael", "Emily", "Daniel", "Ella", "Matthew", "Avery", "Jackson", "Scarlett", "David",
        "Madison", "Joseph", "Luna", "Jackson", "Grace", "Luke", "Chloe", "Samuel", "Penelope", "John",
        "Lily", "Gabriel", "Zoe", "Carter", "Riley", "Anthony", "Nora", "Isaac", "Aria", "Wyatt",
        "Elizabeth", "Andrew", "Hannah", "Joshua", "Sofia", "Christopher", "Addison", "Grayson", "Aurora",
        "Jayden", "Eleanor", "Mateo", "Natalie", "Leo", "Victoria", "Jack", "Hazel", "David", "Nova",
        "Sebastian", "Camila", "Owen", "Alice", "Ryan", "Stella", "Nicholas", "Layla", "Julian", "Zoey",
        "Evan", "Bella", "Levi", "Aubrey", "Nathan", "Aaliyah", "Caleb", "Aurora", "Hunter", "Skylar",
        "Christian", "Claire", "Isaiah", "Lucy", "Thomas", "Sarah", "Lincoln", "Anna", "Jonathan", "Samantha",
    ];

    first_names.choose(&mut rand::thread_rng())
        .ok_or(Error::from(GeneratorErrors::CanNotGenerateName))
        .map(|x| x.to_string())
}


fn generate_last_name() -> Result<String> {
    let last_names = vec![
        "Smith", "Johnson", "Williams", "Brown", "Jones", "Garcia", "Miller", "Davis", "Rodriguez", "Martinez",
        "Hernandez", "Lopez", "Gonzalez", "Wilson", "Anderson", "Thomas", "Taylor", "Moore", "Jackson", "Martin",
        "Lee", "Perez", "Thompson", "White", "Harris", "Sanchez", "Clark", "Ramirez", "Lewis", "Robinson",
        "Walker", "Young", "Hall", "Allen", "King", "Wright", "Scott", "Torres", "Nguyen", "Hill", "Flores",
        "Green", "Adams", "Nelson", "Baker", "Hall", "Rivera", "Campbell", "Mitchell", "Carter", "Roberts",
        "Gomez", "Phillips", "Evans", "Turner", "Diaz", "Parker", "Cruz", "Edwards", "Collins", "Reyes",
        "Stewart", "Morris", "Morales", "Murphy", "Cook", "Rogers", "Gutierrez", "Ortiz", "Morgan", "Cooper",
        "Peterson", "Bailey", "Reed", "Kelly", "Howard", "Ramos", "Kim", "Cox", "Ward", "Richardson", "Watson",
        "Brooks", "Chavez", "Wood", "James", "Bennett", "Gray", "Mendoza", "Ruiz", "Hughes", "Price", "Alvarez",
        "Castillo", "Sanders", "Patel", "Myers", "Long",
    ];

    last_names.choose(&mut rand::thread_rng())
        .ok_or(Error::from(GeneratorErrors::CanNotGenerateName))
        .map(|x| x.to_string())
}

fn generate_city_name() -> Result<String> {
    let city_names = vec![
        "New York", "Los Angeles", "Chicago", "Houston", "Phoenix", "Philadelphia", "San Antonio", "San Diego", "Dallas", "San Jose",
        "Austin", "Jacksonville", "San Francisco", "Indianapolis", "Columbus", "Fort Worth", "Charlotte", "Seattle", "Denver", "El Paso",
        "Detroit", "Washington", "Boston", "Memphis", "Nashville", "Portland", "Oklahoma City", "Las Vegas", "Baltimore", "Louisville",
        "Milwaukee", "Albuquerque", "Tucson", "Fresno", "Sacramento", "Kansas City", "Long Beach", "Mesa", "Atlanta", "Colorado Springs",
        "Virginia Beach", "Raleigh", "Omaha", "Miami", "Oakland", "Minneapolis", "Tulsa", "Wichita", "New Orleans", "Arlington",
        "Cleveland", "Bakersfield", "Tampa", "Aurora", "Honolulu", "Anaheim", "Santa Ana", "Corpus Christi", "Riverside", "Lexington",
        "St. Louis", "Stockton", "Pittsburgh", "Anchorage", "Cincinnati", "Saint Paul", "Greensboro", "Toledo", "Newark", "Plano",
        "Henderson", "Lincoln", "Orlando", "Jersey City", "Chula Vista", "Buffalo", "Fort Wayne", "Chandler", "St. Petersburg", "Laredo",
        "Durham", "Irvine", "Madison", "Norfolk", "Lubbock", "Gilbert", "Winston-Salem", "Glendale", "Hialeah", "Reno", "Garland",
        "Scottsdale", "Irving", "Chesapeake", "North Las Vegas", "Fremont", "Baton Rouge", "Richmond", "Boise", "San Bernardino", "Spokane",
    ];

    city_names.choose(&mut rand::thread_rng())
        .ok_or(Error::from(GeneratorErrors::CanNotGenerateName))
        .map(|x| x.to_string())
}


fn generate_producer_name() -> Result<String>{
    let producer_names = vec![
        "Alpha Products", "Beta Creations", "Gamma Enterprises", "Delta Industries", "Omega Innovations", "Sigma Solutions",
        "Theta Manufacturing", "Zeta Ventures", "Epsilon Exports", "Nu Technologies", "Rho Goods", "Kappa Enterprises",
        "Lambda Creations", "Mu Industries", "Xi Innovations", "Pi Solutions", "Tau Manufacturing", "Upsilon Ventures",
        "Phi Exports", "Chi Technologies", "Psi Goods", "Omega Enterprises", "Alpha Innovations", "Beta Solutions",
        "Gamma Manufacturing", "Delta Ventures", "Sigma Exports", "Theta Technologies", "Zeta Goods", "Epsilon Enterprises",
        "Nu Innovations", "Rho Solutions", "Kappa Manufacturing", "Lambda Ventures", "Mu Exports", "Xi Technologies",
        "Pi Goods", "Tau Enterprises", "Upsilon Innovations", "Phi Solutions", "Chi Manufacturing", "Psi Ventures",
        "Omega Exports", "Alpha Technologies", "Beta Goods", "Gamma Enterprises", "Delta Innovations", "Sigma Solutions",
        "Theta Manufacturing", "Zeta Ventures", "Epsilon Exports", "Nu Technologies", "Rho Goods", "Kappa Enterprises",
        "Lambda Creations", "Mu Industries", "Xi Innovations", "Pi Solutions", "Tau Manufacturing", "Upsilon Ventures",
        "Phi Exports", "Chi Technologies", "Psi Goods", "Omega Enterprises", "Alpha Innovations", "Beta Solutions",
        "Gamma Manufacturing", "Delta Ventures", "Sigma Exports", "Theta Technologies", "Zeta Goods", "Epsilon Enterprises",
        "Nu Innovations", "Rho Solutions", "Kappa Manufacturing", "Lambda Ventures", "Mu Exports", "Xi Technologies",
        "Pi Goods", "Tau Enterprises", "Upsilon Innovations", "Phi Solutions", "Chi Manufacturing", "Psi Ventures",
        "Omega Exports",
    ];

    producer_names.choose(&mut rand::thread_rng())
        .ok_or(Error::from(GeneratorErrors::CanNotGenerateName))
        .map(|x| x.to_string())
}


fn generate_item_name() -> Result<String>{
    let item_names = vec![
        "Widget", "Gadget", "Thingamajig", "Doohickey", "Contraption", "Device", "Apparatus", "Tool", "Implement", "Equipment",
        "Appliance", "Utensil", "Instrument", "Machine", "Contrivance", "Artifact", "Accessory", "Component", "Fixture", "Article",
        "Unit", "Product", "Gizmo", "Furnishing", "Material", "System", "Kit", "Element", "Part", "Component", "Mechanism",
        "Object", "Item", "Piece", "Gear", "Hardware", "Software", "App", "Solution", "Resource", "Material", "Supply",
        "Stationery", "Stationary", "Material", "Apparel", "Clothing", "Garment", "Wearable", "Accessory", "Footwear", "Fashion",
        "Textile", "Fabric", "Cloth", "Thread", "Yarn", "Fiber", "Wool", "Cotton", "Silk", "Denim", "Leather",
        "Synthetic", "Plastic", "Metal", "Wood", "Glass", "Ceramic", "Porcelain", "Pottery", "Stone", "Brick", "Concrete",
        "Tile", "Laminate", "Vinyl", "Paper", "Cardboard", "Packaging", "Box", "Bag", "Container", "Envelope", "Packet",
        "Wrapper", "Label", "Tag", "Sticker", "Tape", "Ribbon", "String", "Twine", "Cord", "Chain", "Wire",
    ];

    item_names.choose(&mut rand::thread_rng())
        .ok_or(Error::from(GeneratorErrors::CanNotGenerateName))
        .map(|x| x.to_string())
}


fn main() {
    println!("Test")
}