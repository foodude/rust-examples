use std::env;
use std::{error::Error, io, process};

fn main() {
    let arguments: Vec<String> = env::args().collect();
    if arguments.len() < 2 {
        eprintln!("ERROR: no argument. ARGS: < stdout | stdin >");
        process::exit(1);
    }

    if arguments[1] == "stdout" {
        if let Err(err) = wrtie_to_stdout() {
            eprintln!("ERROR: {:?}", err);
            process::exit(1)
        }
    } else if arguments[1] == "stdin" {
        if let Err(err) = read_from_stdin() {
            eprintln!("ERROR: {:?}", err);
            process::exit(1)
        }
    } else {
        eprintln!("ERROR: unknown argument {}", arguments[1]);
        process::exit(1)
    }
}

fn read_from_stdin() -> Result<(), Box<dyn Error>> {
    let mut reader = csv::Reader::from_reader(io::stdin());

    for result in reader.records() {
        let record = result?;
        println!("{:?}", record);
    }
    Ok(())
}

fn wrtie_to_stdout() -> Result<(), Box<dyn Error>> {
    let mut writer = csv::Writer::from_writer(io::stdout());

    writer.write_record(&["Name", "Verse", "Superpower"])?;
    writer.write_record(&["Spiderman", "Marvel", "Organic webbing"])?;
    writer.write_record(&["Wolverine", "Marvel", "Self healing"])?;
    writer.write_record(&["Flash", "DC", "Blazingly fast"])?;
    writer.write_record(&["Aquaman", "DC", "False"])?;
    writer.flush()?;
    Ok(())
}

