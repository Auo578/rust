use std::fmt::Display;

trait  Item<T = String> {
    type Output:Display;
    fn summarize(&self) -> Self::Output;
}

struct  Apple {
    name: String,
}

impl Item for Apple {
    type Output = String;
    fn summarize(&self) -> String {
        self.name.to_string()
    }
}
struct Weibo{
    author:String,
    content:String,
}

impl Item for Weibo {
    type Output = String;
    fn summarize(&self) -> String {
        format!("@{}:{}",self.author,self.content)
    }
}

pub struct Container{
    items:Vec<Box<dyn Item<Output = String>>>,
}

impl Container {
    pub fn iterator(&self){
        for item in &self.items {
            println!("{}",item.summarize());
        }
    }
}

fn main() {
    let apple = Apple {
        name:"Apple".to_string(),
    };

    let w = Weibo{
        author:"Weibo".to_string(),
        content:"Hello".to_string(),
    };

    let container = Container{
        items:vec![Box::new(apple),Box::new(w)],
    };

    container.iterator();
}