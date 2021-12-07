use model::User;

pub fn get_login_info(name:String,pass:String)->User{
    User{
        username:name,
        password:pass
    }
}
