

const FILE_PATH: &'static str = "src/input_day1.txt";

fn main() {
    day1();
}

pub fn day1(){
    println!("Day1!");

    let input_res = std::fs::read_to_string(FILE_PATH);
    if input_res.is_err(){
        println!("Error unwrapping result");
        return;
    }

    let input = input_res.unwrap();
    let split_lines = input.split("\n");
    let mut sum = 0;

    for split_line in split_lines{
        println!("{}", split_line);
        let char_vec: Vec<char> = split_line.chars().collect();

        let mut val1_set = false; //Added this bool because couldn't use -1 as 'uninit val'
        let mut val1 = 0;
        let mut val2 = 0;

        for char_val in char_vec{
            let val_from_char = char_val.to_digit(10); //string uses .parse(), char .to_digit()

            if val_from_char.is_none(){
                continue;
            }

            if !val1_set{
                val1 = val_from_char.unwrap();
                val1_set = true;
            }

            val2 = val_from_char.unwrap();
        }
        println!("Vals found {} {}", val1, val2);
        //Concat the vals and add them to sum
        let mut vals_string =String::from("");
        vals_string.push(char::from_digit(val1,10).unwrap());
        vals_string.push(char::from_digit(val2,10).unwrap());
        let vals_string_int:i32 = vals_string.parse().unwrap();
        println!("{}", vals_string_int);
        sum = sum + vals_string_int;
    }

    println!("Total sum: {}", sum);
}

