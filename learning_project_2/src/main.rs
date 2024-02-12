fn main() {
    let name: &str = "Dominik";
    let age: u32 = 42;
    let birth_year: u16 = 1982;

    greet_by_name(name,age);
    
    let age_in_days: u16 = calculate_age_in_days(birth_year);
    println!("Your age in days is {}", age_in_days);
}

fn calculate_age_in_days(birth_year: u16) -> u16 {
    use chrono::{Local, NaiveDate, Datelike};
    let mut age_in_days: u16 = 0;
    let days_in_leap_year: u16 = 366;
    let days_in_normal_year: u16 = 365;
    let current_year: i32 = Local::now().year();
    let mut year_counter = birth_year;
    
    while u32::from(year_counter) as i32 <= current_year {
        let leap_year: bool = NaiveDate::from_ymd_opt(current_year,1,1).unwrap().leap_year();
        if leap_year {
            age_in_days = age_in_days + days_in_leap_year;
        } else {
            age_in_days = age_in_days + days_in_normal_year;
        }
        year_counter +=1; // Count up to the current year
    }
    return age_in_days;
}

fn greet_by_name(name: &str, age: u32) {
    println!("Hello {}! You are {} years old.", name, age);

}
