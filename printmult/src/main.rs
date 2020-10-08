fn main() {
    let mut row = String::from("");
    for x in 1..13{
        row = String::from("");
        let mut space = "";
        for y in 1..13{
            let mult = x * y;
            
            if mult >= 100{
                space = " ";
            }else if mult >= 10{
                space = "  ";
            }else{
                space = "   ";
            }
            row = format!("{}{}{}", row ,space, mult) ; 
        }
        println!("{}", row);
    }
}

