mod hello;
use view::get_login_info;
use model::{ User, Order };
 
fn main() {

    // mut is mutable
    let mut u = get_login_info(
        String::from("test"),
        String::from("123456")
    );
    u.password = String::from("abcde");
    println!("{:#?}", u);
 
    let o = Order {
        ordernum: String::from("20211207174624")
    };
    println!("{:?}",o);
 
    let u1 = User{
        username: String::from("admin"),
        password: String::from("**123**")
    };

    hello::print_some_str();
    println!("{:?}", u1);
}
