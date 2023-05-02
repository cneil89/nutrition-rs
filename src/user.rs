
#[derive(Debug)]
pub enum Gender {
    Male,
    Female,
}

#[derive(Debug)]
pub struct User {
    last_name: String,
    first_name: String,
    email: String,
    // Height inches
    height: u8,
    // Weight in lbs
    weight: u16,
    age: u8,
    gender: Gender,
    basal_metabolic_rate: f32,
}

impl User {
    pub fn new(last_name: String, first_name: String, age: u8, gender: Gender) -> User {
        let mut user = User { 
            last_name, 
            first_name, 
            age,
            email: String::from("Niloc@colin.com"),
            height: 72,
            weight: 230,
            gender,
            basal_metabolic_rate: 0.00,
        };    
        user.set_bmr();
        user
    }

    pub fn print_name(&self) {
        println!("{}, {}  =>  {}", self.last_name, self.first_name, self.basal_metabolic_rate);
    }

    pub fn get_email(&self) -> &String {
        &self.email
    }
    
    fn set_bmr(&mut self) {
       match self.gender {
           Gender::Male => self.basal_metabolic_rate = (10.0 * (self.weight as f32 * 0.4535924)) + 
               (6.25 * (self.height as f32 * 2.54)) - (5.0 * self.age as f32) + 5.0,
           Gender::Female => self.basal_metabolic_rate = 555.10,
       } 
    }
}
