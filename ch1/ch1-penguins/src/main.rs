fn main() {
  // <1> <2>
  let penguin_data = "\
  common name,length (cm)
  Little penguin,33
  Yellow-eyed penguin,65
  Fiordland penguin,60
  Invalid,data
  ";

  let records = penguin_data.lines();

  // Common Control Flow Mechanisms
  for (i, record) in records.enumerate() {
    //Skips header row and lines with only whitespace
    if i == 0 || record.trim().len() == 0 {
      // <3>
      continue;
    }

    //Method syntax
    //Starts with a line of text
    let fields: Vec<_> = record // <4>
      // Splits records in fields
      .split(',') // <5>
      //Trims whitespace of each field
      .map(|field| field.trim()) // <6>
      //Builds a collection of fields
      .collect(); // <7>

    //checks configuration at compile time
    if cfg!(debug_assertions) {
      // <8>
      //prints to standard error
      eprintln!("debug: {:?} -> {:?}", record, fields); // <9>
    }

    let name = fields[0];
    //Attempts to parse fields as a floating point number
    if let Ok(length) = fields[1].parse::<f32>() {
      // <10>
      //Prints to standard out
      println!("{}, {}cm", name, length); // <11>
    }
  }
}
