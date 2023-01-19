fn main() {
    let my_string = "this cat this bat this rat";
    let mut word:Vec<&str> = my_string.split(" ").collect();
    for i in 0..=word.len(){
        for j in 0..=word.len(){
            if j > word.len()-1 || i > word.len()-1{
                break;
            }else if i != j+1 && j+1 < word.len() && word[i] == word[j+1]{
                word.remove(j+1);
            }
        }
    }
    println!("unique : {:?}",word);
    println!("count : {}",word.len());
}
#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        let my_string = "this cat this bat this rat";
        let mut word:Vec<&str> = my_string.split(" ").collect();
        for i in 0..=word.len(){
            for j in 0..=word.len(){
                if j > word.len()-1 || i > word.len()-1{
                    break;
                }else if i != j+1 && j+1 < word.len() && word[i] == word[j+1]{
                    word.remove(j+1);
                }
            }
        }
        let result = word.len();
        assert_eq!(result, 4);
    }
}  