#[allow(unused)]
mod methods;

use std::io::{self};

//Basic structure for the user system.
struct User {
    name:String,

    pin:u32,
    age:u32,
}

fn get_asterisks(asterisks:Vec<String>, count:u32) {
    let mut i:usize = 0;
    loop {
        if i >= count as usize {
            break;
        }

        else {
            print!("{}", asterisks[i]);
            i += 1;
        }
    }
}

impl User {
    fn set_name(&mut self, name:String) {
        if name != "" {
            if self.name == "" {
                println!("Set users name to: {}", name);
                self.name = String::from(name);
            }

            else {
                println!("Set {}'s name to: {}", self.name, name);
                self.name = String::from(name);
            }
        }

        else {
            println!("New name is NULL.");
        }
    }

    fn set_pin(&mut self, pin:u32) {
        if pin != 0 {
            let mut num:u32 = pin;
            let mut count:u32 = 0;

            let mut asterisks:Vec<String> = vec![];

            //This loop gets the amount of digits in the new pin.
            loop {
                if num > 0 {
                    count += 1;
                    asterisks.push(String::from("*"));
                    num /= 10;
                }

                else {
                    break;
                }
            }
            
            self.pin = pin;

            print!("Set {}'s pin to: ", self.name);
            get_asterisks(asterisks, count);
            print!("\n");
        }

        else {
            println!("New pin is NULL.");
        }
    }

    fn get_pin(&mut self) {
        let mut num:u32 = self.pin;
        let mut _count:u32 = 0;

        let mut asterisks:Vec<String> = vec![];

        //This loop gets the amount of digits in the new pin.
        loop {
            if num > 0 {
                _count += 1;
                asterisks.push(String::from("*"));
                num /= 10;
            }

            else {
                break;
            }
        }

        get_asterisks(asterisks, _count);
        println!("");
    }

    fn set_age(&mut self, age:u32) {
        if age != 0 {
            println!("Set {}'s age to {}", self.name, age);
            self.age = age;
        }

        else {
            println!("New age is NULL.");
        }
    }

    fn build(&mut self, name:String, pin:u32, age:u32) {
        if name != "" && pin != 0 && age != 0 {

            println!("Building user: {}", name);

            self.set_name(name);
            self.set_pin(pin);
            self.set_age(age);
            
            // V Debugging purposes
            println!("Built user: {}", self.name);
        }

        else {
            println!("Name, pin, or age is NULL.");
        }
    }
}

fn main() {
    let mut users:Vec<User> = vec![];
    let mut running:bool = true;
    
    loop {
        if !running {
            /*This part checks if the running variable is false, if so: it stops
            the program.*/
            break;
        }

        else {
            methods::list_commands();

            let mut input:String = String::new();
            io::stdin().read_line(&mut input).unwrap();

            let input_n:u32 = input.trim().parse().unwrap();
            if input_n > 9 {
                running = false;
            }

            if methods::get_input(input_n) == "new" {
                let mut name:String = String::new();

                let mut pin_s:String = String::new();
                let mut age_s:String = String::new();

                io::stdin().read_line(&mut name).unwrap();
                io::stdin().read_line(&mut pin_s).unwrap();
                io::stdin().read_line(&mut age_s).unwrap();

                let pin:u32 = pin_s.trim().parse().unwrap();
                let age:u32 = age_s.trim().parse().unwrap();

                if name != "" && pin != 0 && age != 0 {
                    let mut u:User = User {
                        name: String::from(""),
                        pin:0,
                        age:0,
                    };

                    let n:String = String::from(name.trim());

                    u.build(n, pin, age);
                    users.push(u);

                    println!("Successfully built new user!");
                }
            }

            else if methods::get_input(input_n) == "setname" {
                let mut new_name:String = String::new();
                let mut name:String = String::new();

                io::stdin().read_line(&mut new_name).unwrap();
                io::stdin().read_line(&mut name).unwrap();

                name = String::from(name.trim());
                let mut exists:bool = false;

                for user in &mut users {

                    if name.eq(&user.name) {
                        exists = true;
                        let n = String::from(&new_name).to_owned();
                        user.name = String::from(n);

                        println!("Successfully changed users name to: {}", user.name);
                        break;
                    }
                }

                if !exists {
                    println!("Couldn't find user: {}", name);
                }

            }

            else if methods::get_input(input_n) == "setage" {
                let mut name_s:String = String::new();
                let mut new_age_s:String = String::new();

                io::stdin().read_line(&mut name_s).unwrap();
                io::stdin().read_line(&mut new_age_s).unwrap();

                name_s = String::from(name_s.trim());
                new_age_s = String::from(new_age_s.trim());

                let new_age:u32 = new_age_s.trim().parse().unwrap();

                let mut exists:bool = false;

                for user in &mut users {
                    if name_s.eq(&user.name) {
                        exists = true;
                        user.set_age(new_age);

                        println!("Successfully changed {}'s age to: {}", user.name, user.age);
                        break;
                    }
                }

                if !exists {
                    println!("Cannot find user: {}", name_s);
                }
            }

            else if methods::get_input(input_n) == "setpin" {
                let mut name:String = String::new();
                let mut new_pin_s:String = String::new();

                io::stdin().read_line(&mut name).unwrap();
                io::stdin().read_line(&mut new_pin_s).unwrap();

                name = String::from(name.trim());
                new_pin_s = String::from(new_pin_s.trim());

                let new_pin:u32 = new_pin_s.parse().unwrap();

                let mut exists:bool = false;

                for user in &mut users {
                    if name.eq(&user.name) {
                        exists = true;
                        user.set_pin(new_pin);

                        print!("Successfully changed {}'s pin to: ", user.name); user.get_pin();
                        break;
                    }
                }

                if !exists {
                    println!("Couldn't find user: {}", name);
                }
            }

            else if methods::get_input(input_n) == "getusers" {
                println!("getting all users");
                let mut i:u32 = 1;

                for user in &users {
                    println!("{}: {}", i, user.name);
                    i += 1;
                }
                
            }

            else if methods::get_input(input_n) == "getpin" {
                let mut name:String = String::new();
                io::stdin().read_line(&mut name).unwrap();

                name = String::from(name.trim());
                for user in &mut users {
                    if name.eq(&user.name) {
                        print!("{}'s pin is: ", user.name); user.get_pin();
                        break;
                    }
                }
            }

            else if methods::get_input(input_n) == "getage" {
                let mut name:String = String::new();
                io::stdin().read_line(&mut name).unwrap();

                name = String::from(name.trim());
                for user in &mut users {
                    if name.eq(&user.name) {
                        print!("{}'s age is: {}\n", user.name, user.age);
                        break;
                    }
                }
            }
        }
    }
}