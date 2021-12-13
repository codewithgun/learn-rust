pub fn test_match() {
    let country_code = 101;

    let country = match country_code {
        44 => "US",
        46 => "Sweden",
        7 => "Russia",
        60 => "Malaysia",
        100..=1000 => "Alien !!",
        _ => "Unknown",
    };

    println!("Country code {} is {}", country_code, country);
}
