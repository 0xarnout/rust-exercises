fn main() {
    const ORDINALS: [&str; 12] = [
        "first", "second", "third", "fourth", "fifth", "sixth", "seventh", "eighth", "ninth",
        "tenth", "eleventh", "twelfth",
    ];

    const GIFTS: [&str; 12] = [
        "And a partridge in a pear tree.",
        "Two turtle doves,",
        "Three French hens,",
        "Four calling birds,",
        "Five gold rings,",
        "Six geese a-laying,",
        "Seven swans a-swimming,",
        "Eight maids a-milking,",
        "Nine ladies dancing,",
        "Ten lords a-leaping,",
        "Eleven pipers piping,",
        "Twelve drummers drumming,",
    ];


    println!("On the first day of Christmas my true love sent to me");
    println!("A partridge in a pear tree.");
    for day in 1..12 {
        println!("\nOn the {} day of Christmas my true love sent to me", ORDINALS[day]);
        for gift in (0..=day).rev() {
            println!("{}", GIFTS[gift]);
        }
    }
}
