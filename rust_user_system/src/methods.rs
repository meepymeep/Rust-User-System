pub fn list_commands() {
    println!("*User System Commands*");
        println!("\t1, \"new\" (creates a new user)");
    println!("\t2, \"setname\" (set name of user by entering in the new name, then\nthe old name.)");
        println!("\t3, \"setage\" (set age of user)");
    println!("\t4, \"setpin\" (set pin of user)");
        println!("\t5, \"getusers\" (get all users)");
    println!("\t6, \"getage\" (returns age)");
        println!("\t7, \"getpin\" (returns pin)");
}

pub fn get_input(input:u32) -> String {
    match input {
        1=> {
            return String::from("new");
        }

        2=> {
            return String::from("setname");
        }

        3=> {
            return String::from("setage");
        }

        4=> {
            return String::from("setpin");
        }

        5=> {
            return String::from("getusers");
        }

        6=> {
            return String::from("getage");
        }

        7=> {
            return String::from("getpin");
        }

        _=> {
            return String::from("null");
        }

        0_u32 | 2_u32..=u32::MAX => todo!()
    }
}