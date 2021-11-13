//Encapsulation that Hides Implementation Details

/*
pub struct AveragedCollection {
    list: Vec<i32>,
    average: f64,
}

impl AveragedCollection {
    pub fn add(&mut sef, value: i32)  {
        self.list.push(value);
        self.update_avarage();
    }

    pub fn remove(&mut self) -> Option<i32>{
        let result = self.pop();
        match result {
            Some(value) => {
                self.update_avarage();
                Some(value)
            }
            None => None,
        }
    }

    pub fn average(&self) -> f64{
        self.average
    }

    pub fn update_avarage(&mut self){
        let total: i32 = self.list.iter().sum();
        self.average = total as f64 / self.list.len() as f64;
    }
}

*/ 

/*
//Defining a Trait for Common Behavior
pub trait Draw{
    fn draw(&self);
}

pub struct Screen {
    pub components: Vec<Box<dyn Draw>>,
}

impl Screen {
    pub fn run(&self){
        for component in self.components.iter(){
            component.draw();
        }
    }
}

//Implementing the Trait
pub struct Button {
    pub width: u32,
    pub height: u32,
    pub label: String,
}

impl Draw for Button{
    fn draw(&self){
    // code to actually draw a button
    }
}
*/

pub struct  Post{
    content: String,    
}

pub struct DraftPost{
    content: String,
}

impl Post{
    pub fn new() -> DraftPost{
        DraftPost {
            content: String::new(),
        }
    }

    pub fn content(&self) -> &str{
        &self.content
    }

    //Ensuring the Content of a Draft Post Is Empty
    //pub fn content(&self) -> &str{
      //  self.state.as_ref().unwrap().content(self)
    //}
}

impl DraftPost{
    pub fn add_text(&mut self, text: &str){
        self.content.push_str(text);
    }
    pub fn request_review(self) -> PendingReviewPost{
        PendingReviewPost{
            content: self.content,
        }
    }

}

pub struct PendingReviewPost{
    content: String,
}

impl PendingReviewPost{
    pub fn approve(self) -> Post{
        Post{            
            content: self.content,
        }
    }
}

trait State{
    fn request_review(self: Box<Self>) -> Box<dyn State>;
    fn approve(self: Box<Self>) -> Box<dyn State>;

    //chapter 10 life time argument => <'a> 
    fn content<'a>(&self, post: &'a Post) -> &'a str{
        ""
    }

}

struct Draft{}

impl State for Draft{
    fn request_review(self: Box<Self>) -> Box<dyn State>{
        Box::new(PendingReview{})
    }

    fn approve(self: Box<Self>) -> Box<dyn State>{
        self
    }
}

struct PendingReview{}

impl State for PendingReview{
    fn request_review(self: Box<Self>) -> Box<dyn State>{
        self
    }

    fn approve(self: Box<Self>) -> Box<dyn State>{
        Box::new(Published {})
    }
}

struct  Published{}

//Adding the approve Method that Changes the Behavior of content
impl State for Published{
    fn request_review(self: Box<Self>) -> Box<dyn State>{
        self
    }

    fn approve(self: Box<Self>) -> Box<dyn State>{
        self
    }
}
