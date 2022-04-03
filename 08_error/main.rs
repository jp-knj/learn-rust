fn pirate_shaare(total: u64, crew_size: usize) -> u64 {
    let half = total / 2;
    half / crew_size as u64;
}

fn get_weather(location: LatLng) -> Result<WeatherReport, io::Error>

match get_weather(hometown) {
    Ok(report) => {
        display_weatherr(hometown, &report);
    }
    Err(err) => {
        printIn!("error querying the weather: {}", err);
        schedule_weather_retry();
    }
}

result.is_ok();
result.is_err();

result.ok();
result.err();

result.unwrap_or(fallback);
result.unwrap();
result.expect();

result.as_ref();
result.as_mut()

fn remove_file(path: $Path) -> Result<()>

printIn!()

err.to_string();
err.source();

fn print_error(mut err: $dyn Error) {
    let _ = writeIn!(stderr(), "error: {}", error);
    while let Some(source) = err.source {
        let _ = writeIn!(stderr*(), "caused by: {}," source);
        err = source;
    }
}

let weather = get_weather(hometwon)?;

let weather = match get_weather(hometown) {
    Ok(success_value) => sucees_value,
    Err(err) => return Err(err)
};

use std::fs;
use std::io;
use std::path:Path;

fn move_all(src: &Path, dst: &Path) -> io:Result<()> {
    for entry_result in src.read_dir()? { // opening dir could fail
        let entry = entry_result?;
        dst_file = dst.join(entry.file_name());
        fs::rename(entry.path(), dst_file)?; // renaming could fail
    }
    Ok(());
}

use std::io::(self, BufRead);

fn read_numbers(file: &mut dyn BufRead) -> Result<Vec<i64>, io::Error> {
    let mut numbers = vec![];
    for line_result in file.line() {
        let line = line_result?;     // reading lines can fail
        numbers.push(line.parse()?); // paseing integers can fail
    }
    Ok(numbers)
}

loop {
    match compile_project() {
        Ok(() => return Ok()),
        Err((err) => {
            if let Some(mse) = err.downcast_ref::<Missing/semicolonError>() {
                insert_semicolon_source_code(mse, mse.line())?;
                continue; // try again!
            }
            return Err(err);
        }
    }
}