const INPUT: &'static str = "\"To be or not to be,\", quoth the Bard, \"that is the question\".";

fn main() {
    let (_,out) = INPUT.chars().fold((false,String::new()),|(mut left_quote, mut out), c|{
    	if c == '"' {
    		if !left_quote {
    			out.push_str("``");
    		} else {
    			out.push_str("’’");
    		}
    		left_quote = !left_quote;
    	} else {
    		out.push(c);
    	}
    	(left_quote, out)
    });

    println!("{}\n{}", INPUT, out);
}
