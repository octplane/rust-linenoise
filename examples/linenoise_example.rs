extern crate linenoise;

fn callback(input: &str) -> Vec<String> {
  let ret : Vec<&str>;
  if input.starts_with("s") {
    ret = vec!["suggestion", "suggestion2", "suggestion-three"];
  } else {
    ret = vec!["wot"];
  }

  return ret.iter().map(|s| s.to_string()).collect();
}

fn main() {
  linenoise::set_callback(callback);
  linenoise::set_multiline(0);

  loop {
    let val = linenoise::input("Hello Dave -> ");
    match val {
      None => { break }
      Some(input) => {
        println!("> {}", input);
        linenoise::history_add(input.as_ref());
        let is: &str = input.as_ref();
        if is == "clear" {
          linenoise::clear_screen();
        } else if is == "history" {
          let mut index = 0;
          loop {
            match linenoise::history_line(index) {
              None => { break; },
              Some(line) => {
                println!("{}: {}", index, line);
              }
            };
            index = index + 1;
          }
        }
      }
    }
  }
  linenoise::history_free( );
}
