fn main() {
    
    let string1: &str ="Hello World,";
    let string2: &str = " I am Emin";

    let concatenated_string = concatenate_strings(&string1, &string2);

    println!("{}" , concatenated_string);

  


}

fn concatenate_strings(string1: &str, string2: &str) -> String {
    
    let mut result: String = string1.to_string();
    result.push_str(&string2);
    return result;

}

