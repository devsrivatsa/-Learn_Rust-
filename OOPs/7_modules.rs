use building::temple;

mod building {
    // pub stands for public 
    pub mod school {
        pub fn something() {
            println!("inside school module which is inside building module");
        }
    }




    pub mod hospital {
        pub fn something() {
            println!("inside hospital module which is inside building module");
        }
        pub fn get_medical_staff() {
            println!("Medical staff has been made available");
        }
    }




    pub mod church {
        pub fn something() {
            println!("inside church module which is inside building module")
        }
    }




    // by default a child module and everything inside of it is private from the prespective of the parent
    // hence we use the pub keyword
    pub mod temple {

        pub enum Deities {
            SpaceDeity,
            AirDeity,
            FireDeity,
            WaterDeity,
            EarthDeity
        }


        //this is a private function as this does not include the pub keyword
        fn clean_temple() {
            println!("Temple is now cleaned");
        }


        //super::
        //ready temple calls functions from a different module
        //it calls a function from the same module
        //It also calls function from the parent module
        pub fn ready_temple() {
            super::renovate_building();
            clean_temple();
            super::hospital::get_medical_staff();
            print!("Temple is now ready for the carnival")
        }

        
        pub struct HolyFood {
            pub starter:String,
            pub main_course:String,
            pub drink:String
        }
        impl HolyFood {
            pub fn get_default_drink(&self) -> String {
                String::from("water")
            }
            
            pub fn give_some_food(&self) -> HolyFood {
                HolyFood {
                    starter:String::from("peanuts"),
                    main_course:String::from("rice"),
                    drink:String::from("poridge")
                }
            }
        }
        pub fn make_holy_food(s:String, m:String, d:String) -> HolyFood {
            HolyFood {
                starter:s,
                main_course:m,
                drink:d
            }
        }

    }

    fn renovate_building() {
        print!("Building is now renovated..");
    }

}

pub fn go() {
    crate::building::school::something(); //absolute path
    building::church::something(); //relative path
   
}





fn main() {
    
    
    //using short path with use
    use crate::building::temple; //absolute path
    
    
    go();

    let mut f = temple::HolyFood{
        starter:String::from(""),
        main_course:String::from(""),
        drink:String::from("")
    };
    f.give_some_food();
}