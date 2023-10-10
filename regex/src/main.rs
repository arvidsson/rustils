fn main() {
    println!("abc…	\tLetters");
    println!("123…\t\tDigits");
    println!("\\d\t\tAny Digit");
    println!("\\D\t\tAny Non-digit character");
    println!(".\t\tAny Character");
    println!("\\.\t\tPeriod");
    println!("[abc]\t\tOnly a, b, or c");
    println!("[^abc]\t\tNot a, b, nor c");
    println!("[a-z]\t\tCharacters a to z");
    println!("[0-9]\t\tNumbers 0 to 9");
    println!("\\w\t\tAny Alphanumeric character");
    println!("\\W\t\tAny Non-alphanumeric character");
    println!("{{m}}\t\tm Repetitions");
    println!("{{m,n}}\t\tm to n Repetitions");
    println!("*\t\tZero or more repetitions");
    println!("+\t\tOne or more repetitions");
    println!("?\t\tOptional character");
    println!("\\s\t\tAny Whitespace");
    println!("\\S\t\tAny Non-whitespace character");
    println!("^…$\t\tStarts and ends");
    println!("(…)\t\tCapture Group");
    println!("(a(bc))\t\tCapture Sub-group");
    println!("(.*)\t\tCapture all");
    println!("(abc|def)\tMatches abc or def");
}
