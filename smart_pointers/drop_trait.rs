//smart pointers and implement drop trait and deref trait
//drop trait: will free up the memory the variable has occupied after its lifetime
//variables will be droped in the reverse order of their creation
//but the drop method will not be allowed to call manually. To free up memory, use drop() method. This is a method provided by the std library, and has nothing to do with drop trait

struct CustomSmartPointer {
    data: String,
}

//implementing drop trait manually
impl Drop for CustomSmartPointer {
    fn drop(&mut self) {
        println!("Dropping customSmartPointer with data: {}", self.data);
    }
}

fn main() {
    let c = CustomSmartPointer {
        data: String::from("Srivatsa"),
    };
    let d = CustomSmartPointer {
        data: String::from("anu"),
    };
    println!("Smart Pointers created");

}