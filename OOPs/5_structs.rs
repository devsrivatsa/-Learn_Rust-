//regular struct
struct User {
    username:String,
    email:String,
    sign_in_count: u64,
    active:bool
}

//tuple struct
struct Point2d(u32, u32,u32);
struct ColorRGB(i32,i32,i32);

//struct with implementation
//implementations are functions associated with that struct

#[derive(Debug)] //this is a trait. added here to make rectangle printable in println!
struct Rectangle {
    width: u32,
    height: u32
}
impl Rectangle {
    fn perimeter(&self) -> u32
    //the first argument is always a reference to self
    // we can take a mutable reference as well
    // in rare cases, we can take the ownership as well
    {
        return 2 * (self.height + self.width)
    }

    fn area(&self) -> u32 {
        return self.width * self.height;
    }

    fn can_hold(&self, other:&Rectangle) -> bool { 
        return if self.area() > other.area() {true} else {false};
    }
}

// associated function
impl Rectangle {
    //a struct can have as many implementations as needed
    fn create_square(size:u32) -> Rectangle { //note that there is no argument that takes &self as input
        return Rectangle { width: size, height: size}
    }
}




fn main() {

    let mut user1 = User {
        email: String::from("monkeydlufy@pirates.com"),
        username: String::from("Monkey9"),
        sign_in_count: 24,
        active: true
    };

    let name = user1.username;
    user1.username = String::from("Monkey_D_Lufy");

    //building a user with a function
    fn build_user(name:String, email:String) -> User {
        return User {
            username:name,
            email:email,
            sign_in_count:24,
            active:true
        }
    }


    let user2 = build_user(String::from("kenshin"), String::from("rkenshin@animax.com"));

    //partially fill a struct and get the rest from other fields
    let user3 = User {
        username: String::from("Sri"), 
        email: String::from("fake@fakeemail.com"), 
        ..user2 //get rest of the fields from user2
    };

    fn find_area(rect:&Rectangle) -> u32 {
        return rect.height * rect.width;
    }
    let myrectangle = Rectangle{width:24, height:32};
    let otherrect1 = Rectangle{width: 5, height:10};
    let otherrect2 = Rectangle{width: 50, height:40};


    println!("The area of the rectangle is {} sq units", find_area(&myrectangle));
    println!("The perimeter of the rectangle is {}", myrectangle.perimeter());

    println!("Can {:?} hold {:?}: {}", myrectangle, otherrect1, myrectangle.can_hold(&otherrect1));
    println!("Can {:?} hold {:?}: {}", myrectangle, otherrect2, myrectangle.can_hold(&otherrect2));


    //using an associated function
    let square = Rectangle::create_square(14);
    println!("Area of square - {:?} is: {}", square, square.area());


}