fn main(){
    let day =["first", "second", "third", "fourth", "fifth", "sixth", "seventh", "eighth", "ninth", "tenth", "eleventh", "twelfth"];
    let gift =["a partridge in a pear tree", "two turtle doves", "three French hens", "four calling birds", "five gold rings", "six geese a-laying", "seven swans a-swimming", "eight maids a-milking", "nine ladies dancing", "ten lords a-leaping", "eleven pipers piping", "twelve drummers drumming"];
    for i in 0..12{
        println!("On the {} day of Christmas, my true love sent to me:", day[i]);
        for j in (0..=i).rev(){
            if i == 0{
                println!("{}", gift[j]);
            }else if j == 0{
                println!("and {}", gift[j]);
            }else{
                println!("{}", gift[j]);
            }
        }
        println!();
    }
}