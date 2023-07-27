#[derive(Debug)]
struct User{
    username:String,
    email:String,
    sign_in_count:i32,
    active:bool
}

// Write functions in here which can be used to manipulate the struct
impl User {
    fn increment_signin_count(&mut self){
        self.sign_in_count+=1;
    }

    fn change_email(&mut self,new_email:&str){
        self.email = String::from(new_email);
    }

    fn get_user_email(&self)->String{
        String::from(&self.email)
    }
}

fn change_username(user:&mut User,new_username:&str){
  user.username = String::from(new_username);
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn tests_structs() {
       let mut user_one = User{
        username:String::from("Kambang"),
        email:String::from("kambang@email.com"),
        sign_in_count:1,
        active:true
       };

       dbg!(&user_one);
       
       change_username(&mut user_one, "Sinclaire");

       dbg!(&user_one);

       // Testing the Impl addition
       let mut user_2 = User{
        username:String::from("Sinclaire"),
        email:String::from("kambangSinclaire@email.com"),
        sign_in_count:2,
        active:false
       };

       user_2.change_email("Testing@gmail.com");

       dbg!(user_2.get_user_email());

    }
}
