//Encapsulation that Hides Implementation Details
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
