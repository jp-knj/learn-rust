fn main() {
    // 世界よ、こんにちは
    println!("Hello, world!");
}

// exp & statement
5*(fahr-32)/9
for(; begin!= end; ++begin){
    if(*begin == target)
        break;
}

// match式が数値を作り出す
pixels[r * bounds.0 + c] =
    match escapes(Complex {re: point.0, im: [pint.1]}, 255) {
    None => 0,
    Some(count) => 255 - count as u8
};

// expression
let status = if cpu.temperature <= MAX_TEMP {
    HttpStatus:Ok
} else {
    HttpStatus::ServerError
}

// match
printIn!("Inside that vat, you see {}.",
    match vat.contents {
        Some(brain) => brain.desc(),
        Non => "nothing of interest"
    }
)

let display_name = match post.author() {
    Some(author) => author.name(),
    None => {
        let network_info = post.get_network_metadata()?;
        let ip = network_info.client_address();
        ip.to_string()
    }
};

let msg = {
    // let -declaration: semicolon is always required
    let dandelion_control = puffball.open();

    // expression + semicolon: method is called, return value dropped
    dandelion_control.release_all_seeds(launch_codes);

    // espression with no semicolon: method is called
    // return value stored in msg
    dandelion_control.get_status()
}