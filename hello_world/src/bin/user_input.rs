use std::io;

enum PowerState {
    Off,
    Sleep,
    Reboot,
    Shutdown,
    Hibernate,
}

impl PowerState {
    fn new(state: &str) -> Result<PowerState, String> {
        let lowercase_state = state.trim().to_lowercase();
        match lowercase_state.as_str() {
            "off" => Ok(PowerState::Off),
            "sleep" => Ok(PowerState::Sleep),
            "reboot" => Ok(PowerState::Reboot),
            "shutdown" => Ok(PowerState::Shutdown),
            "hibernate" => Ok(PowerState::Hibernate),
            _ => Err("Invalid state input".to_owned()),
        }
    }

    fn println(&self) {
        match self {
            PowerState::Off => println!("turning off"),
            PowerState::Sleep => println!("sleeping"),
            PowerState::Reboot => println!("rebooting"),
            PowerState::Shutdown => println!("shutting down"),
            PowerState::Hibernate => println!("hibernating"),
        }
    }
}

fn main() {
    println!("Please enter your state");
    loop {
        let mut buffer = String::new();
        // let _ = io::stdin().read_line(&mut buffer).or_else(|err| {
        //     println!("Error when reading input");
        //     Err(err)
        // });

        // let _ = PowerState::new(&buffer)
        //     .and_then(|state| {
        //         state.println();
        //         Ok(())
        //     })
        //     .or_else(|err| {
        //         println!("{err}");
        //         Err(err)
        //     });
        if io::stdin().read_line(&mut buffer).is_err() {
            println!("Error when reading input");
            break;
        }

        let input = buffer.trim();
        match PowerState::new(input) {
            Ok(state) => {
                state.println();
                break;
            }
            Err(err) => {
                println!("{err}");
                println!("Please enter again:")
            }
        }
    }
}
