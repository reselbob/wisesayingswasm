use wisesayings::get_wise_saying;

fn main() -> std::io::Result<()> {
    let mut str = get_wise_saying();
    println!("{}", str);
    Ok(())
}
