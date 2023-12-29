const FILE_PATH: &str = "src/input_day1.txt";

fn main() {
    day1();
}

pub fn day1(){
    println!("Day1!");

    let input_res = std::fs::read_to_string(FILE_PATH).expect("Failed to read file");

    let mut sum = 0;

    for split_line in input_res.lines(){
        println!("{}", split_line);

        let mut val1_set = false; //Added this bool because couldn't use -1 as 'uninit val'
        let mut val1 = 0;
        let mut val2 = 0;

        for char_val in split_line.chars(){
            let val_from_char = char_val.to_digit(10); //string uses .parse(), char .to_digit()

            match val_from_char{
                Some(val) => {
                    if !val1_set{
                        val1 = val;
                        val1_set = true;
                    }

                    val2 = val;
                }
                None => continue,
            }
        }

        println!("Vals found {} {}", val1, val2);
        //Concat the vals and add them to sum
        let vals_string = format!("{}{}", val1, val2);
        let vals_string_int: i32 = vals_string.parse().unwrap();
        println!("{}", vals_string_int);

        sum += vals_string_int;
    }

    println!("Total sum: {}", sum);
}