fn main() {
    println!("Hello, world!");
    another_function(5);
    println!("{}", hello());
}

fn another_function(x: u32){
    println!("Another function {}",x);
}

fn hello() -> String{
    
    //セミコロンがないため式である、暗黙的に最後の行がReturnされる
    "Hello".to_string()
}