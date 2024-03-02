use {
    args::Args,
    colored::Colorize,
    std::{error::Error, io},
};

pub mod args;
pub mod board;
pub mod parse;
pub mod text;

pub fn run(_args: Args) {
    // println!("control");
    // println!("{}", "blue".blue());
    // println!("{}", "red on blue".red().on_blue());
    // println!("{}", "bold".bold());
    // println!(
    //     "{} {} {}",
    //     "cyan".cyan(),
    //     "italic yellow".italic().yellow(),
    //     "cyan".cyan()
    // );
    // println!("{}", "yellow blue red".yellow().blue().red());
    // println!(
    //     "{}",
    //     "bright blue on bright white"
    //         .bright_blue()
    //         .on_bright_white()
    // );

    println!("{}", text::intro);

    let mut input = String::new();

    // move:
    // move a piece
    // usage:
    // `move [piece-name] [position]`
    // `move [position] [position]`
    // if only one piece of that name can move there, move it
    // otherwise print error requiring `move [position] [position]` format

    // check:
    // see the effect of moving a piece (without actually moving it)
    // usage:
    // `move [piece-name] [position]`
    // `move [position] [position]`

    // hint:
    // get a (questionable) hint on where to move
    // usage:
    // `hint`
}

fn wait_for_enter(input: &mut String) -> Result<(), Box<dyn Error>> {
    println!("Press enter to continue . . .");
    get_input(input)?;
    Ok(())
}

fn get_input(buf: &mut String) -> Result<(), Box<dyn Error>> {
    // io::stdin().read_line(buf).unwrap_or_else(|err| {
    //     println!("Unexpected input error: {}", err);
    //     process::exit(1);
    // });
    io::stdin().read_line(buf)?;
    Ok(())
}
