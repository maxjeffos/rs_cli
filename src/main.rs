use rs_cli::{get_answer, might_return_error, print_stuff};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let answer = get_answer();
    println!("The answer is: {}", answer);

    let writer = &mut std::io::stdout(); // might return std::io:Error
    print_stuff(writer)?;

    might_return_error()?; // might return rs_cli::Error

    Ok(())
}
