extern crate linenoise;

use std::collections::BTreeMap;
use std::os::raw::c_void;
use std::ptr;

struct Dictionary
{
  map: BTreeMap< String, String >,
}

impl Dictionary
{
  fn new( ) -> Self
  {
    Self { map: BTreeMap::new( ) }
  }

  fn from_void( ptr: *const c_void ) -> Option< &'static Self >
  {
    if ptr == ptr::null( ) { ( ) }
    Some( unsafe { &*{ ptr as *const Self } } )
  }

  fn as_void( &self ) -> *const c_void
  {
    self as *const _ as *const c_void
  }

  fn insert( &mut self, key: &str, val: &str )
  {
    self.map.insert( key.to_string( ), val.to_string( ) );
  }

  fn matches( &self, input: &str ) -> Vec< String >
  {
    let mut result: Vec< String > = vec![ ];
    let range = self.map.range( input.to_string( ) .. );
    for item in range
    {
      if item.0.starts_with( input )
      {
        result.push( item.0.to_string( ) );
      }
    }
    result
  }
}

fn callback(input: &str, ptr: *const c_void) -> Vec<String> {
  match Dictionary::from_void( ptr )
  {
    Some( dict ) => dict.matches( input ),
    None => Vec::new( )
  }
}

fn main() {
  let mut dict = Dictionary::new( );

  dict.insert( "clear", "Clear screen" );
  dict.insert( "help", "This info" );
  dict.insert( "history", "Is wot happened before innit" );

  let ptr = dict.as_void( );
  linenoise::set_callback_with_arg(callback, ptr);
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
