pub struct NewsArticle {
    pub author: String,
    pub headline: String,
    pub content: String
}


pub struct Tweet {
    username: String,
    content: String,
    reply: bool,
    retweet: bool
}
// traits allow us to define a set of methods that are shared between types
pub trait Summary {
    // this is a default implementation. Hence, the concrete structs do not need implementations
    fn summarize_auhor(&self) -> String {
        return String::from("not implemented");
    }
    // there is no default implementation. Hence we need to implement this in the inheriting struct
    fn summarize(&self) -> String;
}

impl Summary for NewsArticle {
    fn summarize(&self) -> String {
        return format!("{}, by {}", self.headline, self.author);
    }
}

impl Summary for Tweet {
    fn summarize(&self) -> String {
        return format!("{}, by {}", self.username, self.content);
    }
}


//a function that takes in a trait as a parameter
fn notify(item: &impl Summary) //item could be anything that implements summary
{
    println!("Breaking News: {}", item.summarize())
}

//the above is just syntactic sugar for the below 
fn notify_real<T: Summary>(item: T) {
    println!("Breaking News: {}", item.summarize())
}

//slightly complicated example where impl syntax is not the best thing
fn slightly_complicated<T:Summary>(item1: T, item2: T) {
    //some code
} 

pub trait Display {
    fn display(&self) {
        println!("this is the display trait!!!")
    }
}

// implementing multiple traits
fn multi(item: &(impl Summary + Display), item2:&(impl Summary + Display)) {
    //code
}
// same thing with generic syntax
fn multi_generic_syntax<T:Summary+Display>(item: &T, item2: &T) {
    //code
}

fn main() {

    let na = NewsArticle {
        author: "Vivekananda".to_owned(),
        headline: "Vedanta".to_owned(),
        content: "The philosophy of the vedas".to_owned()
    };

    let t = Tweet {
        username: String::from("myook"),
        content: String::from("pyodi"),
        reply: false,
        retweet: true
    };

    println!("{}",t.summarize());
    println!("{}",t.summarize_auhor());

    println!("{}",na.summarize());
    println!("{}",na.summarize_auhor());

}