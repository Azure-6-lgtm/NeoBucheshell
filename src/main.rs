/*

    _   __           ____             __              __         ____   
   / | / /__  ____  / __ )__  _______/ /_  ___  _____/ /_  ___  / / /   
  /  |/ / _ \/ __ \/ __  / / / / ___/ __ \/ _ \/ ___/ __ \/ _ \/ / /    
 / /|  /  __/ /_/ / /_/ / /_/ / /__/ / / /  __(__  ) / / /  __/ / /     
/_/ |_/\___/\____/_____/\__,_/\___/_/ /_/\___/____/_/ /_/\___/_/_/      
                                                                        
NeoBucheshell is the remaster of Bucheshell.
It aims to be faster, well-optimized and friendlier for new users.
NeoBucheshell regularly receives updates.

NeoBucheshell has been tested on ->
Android(termux)
Linux(Debian and arch)

If any issues occur or you find any bug.Open an issue on github using the issue template

Created by : Azure-6-lgtm 

For more information, visit NeoBucheshell's github repo.
"https://www.github.com/Azure-6-lgtm/NeoBucheshell"

*/

//Import necessary modules and crates
mod handler;
mod strap;
mod utils;
mod logger;
use logger::buchelog;
use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() >= 2 {
        handler::check::decipher_args(args);
    } else {
    }
    
    buchelog::init_logger(); // Starts the logger and generates examples
    buchelog::log_info("NeoBucheshell started succesfully");
    buchelog::log_warn("This is a warning example");
    buchelog::log_error("This is an error example");
    strap::init::init(); // Starts the actual shell
}
// REMINDER: If issues occur inside the shell , do not check this file , it just starts the shell
