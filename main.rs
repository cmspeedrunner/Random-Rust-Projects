use std::io; //importing the standard input output crate

fn main(){
loop{
    //Reading data
    println!("Welcome to the string detector, enter your first string below.");
    let mut str1 = String::new();
    io::stdin().read_line(&mut str1).expect("Error reading LN data");
    println!("Now enter the second string below!");
    let mut str2 = String::new();
    io::stdin().read_line(&mut str2).expect("Error reading LN data");

    //some if statements to detect if they r same or not
    if str1 == str2{
        println!("The two strings are the same!");
        println!("String 1:{}",str1);
        println!("String 2:{}",str2);
    }
    if str1 != str2{
        println!("The two strings are different!");
        println!("String 1:{}",str1);
        println!("String 2:{}",str2);
    }
    if str1 == "exit"{
        break;
    }
    if str2 == "exit"{
        break;
    }


 }}