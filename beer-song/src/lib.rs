pub fn verse(n: i32) -> String {

  let bottle_str = |x| {
    match x {
      1 => String::from("1 bottle"),
      _ => format!("{} bottles", x)
    }
  };

  if n == 0 {
    return String::from("No more bottles of beer on the wall, no more bottles of beer.\nGo to the store and buy some more, 99 bottles of beer on the wall.\n");
  }

  if n == 1 {
    return String::from("1 bottle of beer on the wall, 1 bottle of beer.\nTake it down and pass it around, no more bottles of beer on the wall.\n");
  }

   format!("{bottles} of beer on the wall, {bottles} of beer.\nTake one down and pass it around, {bottles2} of beer on the wall.\n",
    bottles=bottle_str(n),
    bottles2=bottle_str(n-1))
}

pub fn sing(start: i32, end: i32) -> String {
    let mut output = String::from("");

    for number in (end..start + 1).rev() {
      output.push_str(verse(number).as_str());
      if number != end {
        output.push_str("\n");
      }
    }

    output
}
