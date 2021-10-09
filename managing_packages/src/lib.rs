mod front_of_house {
    mod hosting{ //So, if you want to make an item like a function or struct private, you put it in a module.
        fn add_to_waitlist(){}

        fn seat_at_table(){}
    }    

    pub mod serving{ // exposing modules with "pub" word, by default they are private
        pub fn take_order(){} //exposing functions with "pub" word, by default they are private        
    }
}

// Starting Relative Paths with super

fn serve_order(){}

mod back_of_house{
    fn fix_incorrect_order(){
        cook_order();
        super::serve_order();
    }
    fn cook_order(){}

    // making structs and enums 
    pub struct Breakfast{
        pub toast: String,
        seasonal_fruits: String,
    }

    pub enum Appetizer {
        Soup,
        Salad,
    }

    impl Breakfast{
        pub fn summer (toast: &str) -> Breakfast {
            Breakfast {
                toast: String::from(toast),
                seasonal_fruits: String::from("peaches"),
            }
        }
    }
}



pub fn eat_at_restaurant(){
    //absolute path
    //crate::front_of_house::hosting::add_to_waitlist();

    //relative path
    //front_of_house::hosting::add_to_waitlist();

    crate::front_of_house::serving::take_order(); //now we can access the module and function
    front_of_house::serving::take_order();

    let mut meal = back_of_house::Breakfast::summer("Rye");
    meal.toast = String::from("Wheat");
    print!("I'd like {} toast please", meal.toast);

    let order1 = back_of_house::Appetizer::Soup;
    let order2 = back_of_house::Appetizer::Salad;
}
