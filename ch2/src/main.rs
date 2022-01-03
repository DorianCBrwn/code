fn main() {
  let ctx_lines = 2;
  let needle = "oo";
  let haystack = "\
Every face, every shop,
bedroom window, public-house, and
dark square is a picture
feverishly turned--in search of what?
It is the same with books.
What do we seek
through millions of pages?";

  let mut tags: Vec<usize> = vec![]; // <1> Initialize mutable variable tag that is a vector that stores a pointer-sized unsigned integer type. Hold the line numbers where matches occur
  let mut ctx: Vec<Vec<(usize, String)>> = vec![]; // <2> Creates a vector of vectors that stores a pointer-sized unsigned integer type and a String type. Binds variable ctx to an empty vector.

  // First pass that tags all the lines that contain a match
  for (i, line) in haystack.lines().enumerate() {
    // Iterates through string line by line
    // <3>
    if line.contains(needle) {
      // If the line contains the match
      tags.push(i); // Append the line number to the tags vector

      let v = Vec::with_capacity(2 * ctx_lines + 1); // <4> with_capacity reserves space for n items; In this case reserves space for the return value of 2 * ctx_lines + 1
      ctx.push(v); // Appends a vector containing
    }
  }

  if tags.is_empty() {
    // <5> Guard clause that exits the program if there are no matches
    return;
  }

  // Second pass that collects all the matches
  for (i, line) in haystack.lines().enumerate() {
    // <6> “ For each tag, at every line, checks to see if we are near a match. When we are, adds that line to the relevant Vec<T> within ctx.”
    for (j, tag) in tags.iter().enumerate() {
      let lower_bound = tag.saturating_sub(ctx_lines); // <7>“saturating_sub() is subtraction that returns 0 on integer underflow rather than crashing the program (CPUs don’t like attempting to send usize below zero).” - Crash protection so that the CPU does not send the usize below zero.

      let upper_bound = tag + ctx_lines;

      if (i >= lower_bound) && (i <= upper_bound) {
        let line_as_string = String::from(line); // <8> Creates a local variabel that stores a new String from line
        let local_ctx = (i, line_as_string); // Creates a tuple that stores the line and the string
        ctx[j].push(local_ctx); // Pushes the created tuple to vector of vectors ctx
      }
    }
  }

  for local_ctx in ctx.iter() {
    for &(i, ref line) in local_ctx.iter() {
      // <9>  ref Borrows from line variable
      let line_num = i + 1;
      println!("{}: {}", line_num, line);
    }
  }
}
