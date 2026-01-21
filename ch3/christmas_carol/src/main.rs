use ToString;

fn main() {
    let gifts = [
        "partridge in a pear tree",
        "two turtle doves",
        "three french hens",
        "four calling birds",
        "five gold rings",
        "six geese a-laying",
        "seven swans a-swimming",
        "eight maids a-milking",
        "nine ladies dancing",
        "ten lords a-leaping",
        "eleven pipers piping",
        "twelve drummers drumming",
    ];

    let mut total_gifts = String::new();

    for n in 1..12 {
        let day = if n == 1 {
            "first"
        } else if n == 2 {
            "second"
        } else if n == 3 {
            "third"
        } else if n == 4 {
            "fourth"
        } else if n == 5 {
            "fifth"
        } else if n == 6 {
            "sixth"
        } else if n == 7 {
            "seventh"
        } else if n == 8 {
            "eighth"
        } else if n == 9 {
            "ninth"
        } else if n == 10 {
            "tenth"
        } else if n == 11 {
            "eleventh"
        } else {
            "twelfth"
        };

        if day == "first" {
            total_gifts += "A ";
        } else {
            total_gifts += "\nAnd ";
        }
        total_gifts += gifts[n - 1];

        println!("On the {day} day of Christmas my true love sent to me");
        println!("{total_gifts}\n");

        total_gifts += ",";
    }
}
