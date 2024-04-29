struct User {
    active: bool,
    username: String,
    email: String,
    sign_in_count: u64,
}

struct Color(i32, i32, i32);
struct Point(i32, i32, i32);
struct AlwaysEqual;

fn build_user(email: String, username: String) -> User{
    User {
        active: true,
        username,
        email,
        sign_in_count: 1,
    }
}

fn main(){
    let mut user1 = User {
        active: true,
        username: String::from("NaimaHasan"),
        email: String::from("naimahasan@gmail.com"),
        sign_in_count: 1,
    };

    user1.email = String::from("naima@gmail.com");

    let user2 = User {
        active: user1.active,
        username: user1.username,
        email: String::from("another@example.com"),
        sign_in_count: user1.sign_in_count,
    };

    let user3 = User {
        email: String::from("another@example.com"),
        ..user2
    };

    let black = Color(0, 0, 0);
    let origin = Point(0, 0, 0);
    let subject = AlwaysEqual;
    
}


//Note that the struct update syntax uses = like an assignment; 
//this is because it moves the data,
//In this example, we can no longer use user1 as a whole after creating user2 
//because the String in the username field of user1 was moved into 


//If we had given user2 new String values for both email and username, 
//and thus only used the active and sign_in_count values from user1, 
//then user1 would still be valid after creating user2. 
//Both active and sign_in_count are types that implement the Copy trait