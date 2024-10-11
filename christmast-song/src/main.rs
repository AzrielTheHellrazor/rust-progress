fn main() {
    let gifts = [
        "a Partridge in a Pear Tree.",
        "two Turtle Doves, and",
        "three French Hens,",
        "four Calling Birds,",
        "five Golden Rings,",
        "six Geese a-Laying,",
        "seven Swans a-Swimming,",
        "eight Maids a-Milking,",
        "nine Ladies Dancing,",
        "ten Lords a-Leaping,",
        "eleven Pipers Piping,",
        "twelve Drummers Drumming,",
    ];

    let days = [
        "first", "second", "third", "fourth", "fifth", "sixth", 
        "seventh", "eighth", "ninth", "tenth", "eleventh", "twelfth"
    ];

    for day in 0..12 {
        println!("On the {} day of Christmas, my true love gave to me:", days[day]);
        for gift in (0..=day).rev() {
            println!("{}", gifts[gift]);
        }
        println!(); 
    }
}
