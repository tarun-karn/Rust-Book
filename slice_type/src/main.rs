fn main() {
    let s  = String::from("Hello World");

    let res = find_first_word(&s);

    println!("For string {s} the result is {}",res.len());

}

fn find_first_word(input : &String ) -> &str {
    let s =input.as_bytes();

    for (i,&item) in s.iter().enumerate(){
        if item == b' '{
            return &input[..i];
        }
    }
    &input[..]

}
