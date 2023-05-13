use rand::Rng;

fn main() {
    let mut user_input = String::new();
    let mut rng = rand::thread_rng();
    let answer = rng.gen_bool(0.5);

    loop {
        let args: Vec<String> = std::env::args().collect();

        if args.len() > 1 {
            user_input = args[1].clone();
            break;
        }

        println!("What do you want (or not) to play?");

        std::io::stdin()
            .read_line(&mut user_input)
            .expect("Failed to read line");

        if user_input.trim() == "" {
            user_input = String::new();
            continue;
        } else {
            break;
        }
    }

    match answer {
        true => println!("Good luck, you will play {}", user_input),
        false => println!("Go to sleep, you are NOT playing {}", user_input),
    }
}
