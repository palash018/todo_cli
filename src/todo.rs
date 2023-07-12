mod file;
fn list(data: &Vec<String>) {
    for i in 0..data.len() {
        println!("{} {}", i, data[i]);
    }
}

fn main() {
    std::env::set_var("RUST_BACKTRACE", "1");
    let mut data: Vec<String> = Vec::new();
    loop {
        println!("Type input");
        let mut input = String::new();
        std::io::stdin().read_line(&mut input).expect("input");
        let args = input
            .trim()
            .split(' ')
            .flat_map(str::parse::<String>)
            .collect::<Vec<_>>();
       if(args.len()==1){
        match args[0].as_str(){
            "exit"=>{break;},
            &_=>{

            }
        }
       }
        if args.len() != 2 {
            println!("Expects 2 arguments");
            continue;
        }

        match args[0].as_str() {
            
            "add" => {
                data.push(args[1].clone());
            }
            "list" => match args[1].as_str() {
                "c" => {
                    println!("*************");
                    list(&data);
                    println!("*************");
                }
                "l" => {
                    if !file::search_file() {
                        println!("Local file doesn't exist");
                    }
                    else {
                    {
                        println!("*************");
                        file::show_data();
                        println!("*************");
                    }
                }
                }
                &_ => {
                    continue;
                }
            },
            "write" => match args[1].as_str() {
                "cl" => {
                    file::write_file(&data, false);
                }
                "cla" => {
                    file::write_file(&data, true);
                }
                "lc" => {
                    file::fetch_data(&mut data);
                }
                &_ => {
                    continue;
                }
            },
            "rm" => {
                if args[1].parse::<usize>().unwrap() > data.len()
                    || args[1].parse::<usize>().unwrap() < 0
                {
                    println!("Invalid line no");
                } else {
                    let mut temp: Vec<String> = Vec::new();
                    for i in 0..data.len() {
                        if i != args[1].parse::<usize>().unwrap() {
                            temp.push(data[i].clone());
                        }
                    }
                    data = temp;
                }
            }
            "clear" => match args[1].as_str() {
                "l" => {
                    file::create_file();
                }
                "c" => {
                    data.clear();
                }
                &_ => {
                    continue;
                }
            },
            &_ => {
                continue;
            }
        }
        
    }
}
