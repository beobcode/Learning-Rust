const CLEAR_TERMINAL_ESCAPE_CODE: &str = "\x1B[2J\x1B[H";

// Generated on https://patorjk.com/software/taag/#p=display&h=1&c=bash&f=Doom&t=Mansion%20Mystery
const GAME_TITLE: &str = r#"

#  ___  ___                    _                ___  ___             _                     
#  |  \/  |                   (_)               |  \/  |            | |                    
#  | .  . |  __ _  _ __   ___  _   ___   _ __   | .  . | _   _  ___ | |_  ___  _ __  _   _ 
#  | |\/| | / _` || '_ \ / __|| | / _ \ | '_ \  | |\/| || | | |/ __|| __|/ _ \| '__|| | | |
#  | |  | || (_| || | | |\__ \| || (_) || | | | | |  | || |_| |\__ \| |_|  __/| |   | |_| |
#  \_|  |_/ \__,_||_| |_||___/|_| \___/ |_| |_| \_|  |_/ \__, ||___/ \__|\___||_|    \__, |
#                                                         __/ |                       __/ |
#                                                        |___/                       |___/ 

"#;
const WELCOME_MSG: &str = "Hello and welcome to the Manson Mystery";
const GAME_MENU: &str = r#"
1. Start the game 
2. Read the instructions
3. Exit the game

"#;
const GO_HOME_MENUE_OPTION: &str = "Go Home ";

fn main() {
    println!("{}", WELCOME_MSG);

    show_game_instructions();
    println!("{}",GAME_MENU);
}

fn show_game_instructions(){
    const GAME_INSTRUCTIONS: &str = r#"
These are the game instruction yet to be completed as the game develops

    "#;

    println!("{}",CLEAR_TERMINAL_ESCAPE_CODE);
    println!("{}",GAME_INSTRUCTIONS);
    
}
