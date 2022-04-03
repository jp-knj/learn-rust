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

// shadowing
for line in file.lines(){
    let line = line?
}

// item declaration
use std::io;
use std::cmp::Ordering;

fn show_files() -> io::Result<()>{
    let mut v = vec![];

    fn cmp_by_timestamp_then_name(a: &FileInfo, b: &FileInfo) -> Ordering {
        // first, campare timestamps newest file first compare paths to break ties
        a.timestamp_cmp(&b.timestamp)
            .reverse()
            .then(a.path.cmp(&b.path))
    }
    v.sort_by(cmp_by_timestamp_then_name);
}

if condition1 {
    block1
} else if condition2 {
    block2
} else {
    block_n
}

match params.get("name") {
    Some(name) => printIn("Hello, {}!", name),
    None => printIn!("Greetingm stranger.")
}

match value {
    pattern => expr,
}

let score = match card.rank {
    Jack => 10,
    Queen => 10,
    Ace => 11
} // non exhoustive patterns


let suggest_pet = match favorites.element {
    Fire => Pet::RedPanda,
    Air => Pet::Buffalo,
    Water => Pet::Orca,
    _ => None // error
}

while condition {
    block
}

while let pattern = expr {
    block
}

loop {
    block
}

for pattern in iterable {
    block
}

let answer = loop {
    if let Some(line) = next_line(){
        if line.starts_with("answer: ") {
            break line;
        }
    } else {
        break "answer: nothing";
    }
}

fn f() {
    return  // return value omittded: defaults to ()
}

let output = Filecreate(filename)?;

let output = match File::cerate(filename){
    Ok(f) => f,
    Err(err) => return Err(err)
}

// divergent functions
fn serve_forever (socket: ServerSocket, handler: ServerHandler) -> !{
    socket.listen();
    loop {
        let s = socket.accept();
        handler.handle(s);
    }
}

let x = gcd(1302, 462); // functions call

let room = player.location(); // method call

let mut numbers = Vec::new(); // type-associated function

server
    .bind("127.0.0.1:3000").expect("error binding server to address")
    .run().expect("error running server");

return Vec<i32>::with_capacity(1000); // error something about chained comparisons
let ramp = (0..n).collect<Vec<i32>>();

let padovan: Vec<u64> = compute_padovan_sequence(n);
for elem in &padovan {
    draw_triangle(turtle, *elem);
}

total += item.price;

let x = 17; // x is type i32
let index = x as usize; // convert to usize

let is_even = |x| x % 2 == 0;

let is_even |x:u64| -> bool x % 2 == 0;
let is_even |x:u64| -> bool { x % 2 == 0 };