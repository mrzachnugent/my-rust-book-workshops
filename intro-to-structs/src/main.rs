struct User {
    active: bool,
    username: String,
    email: String,
    sign_in_count: u64,
}

#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    fn area(&self) -> u32 {
        self.height * self.height
    }

    fn width(&self) -> bool {
        self.width > 0
    }

    fn can_hold(&self, rect: &Rectangle) -> bool {
        rect.height >= self.height && rect.width >= self.height
    }
}

impl Rectangle {
    fn square(size: u32) -> Rectangle {
        Rectangle {
            width: size,
            height: size,
        }
    }
}

fn main() {
    chapter5_1()
}

fn chapter5_1() {
    {
        let mut user = User {
            active: true,
            sign_in_count: 1,
            email: String::from("someone@email.com"),
            username: String::from("nunya"),
        };

        user.email = String::from("someone-else@email.com");

        println!("{}", user.email);
    }

    {
        let user = create_user(String::from("gotcha"), String::from("some@email.com"));

        println!(
            "User: {}, is active? {}, and has signed in {} time(s).",
            user.username, user.active, user.sign_in_count
        );

        let _user2 = create_user(user.email, user.username);
    }

    {
        let user = User {
            active: true,
            sign_in_count: 1,
            email: String::from("someone@email.com"),
            username: String::from("nunya"),
        };

        let mut user2 = User {
            email: String::from("mrzachnugent@github.com"),
            ..user
        };

        user2.active = false;

        // println!("'{}' is different from '{}'", user.username, user2.username) // username (String) was moved from user to user2
    }

    {
        struct Color(i32, i32, i32);
        struct Point(i32, i32, i32);

        let black = Color(0, 0, 0);
        let origin = Point(0, 0, 0);

        let x = black.0;
        let o = origin.1;
        println!("color0: {}, origin1: {}", x, o)
    }

    {
        struct AlwaysEqual;
        let _subject = AlwaysEqual;
    }

    {
        let height = 30;
        let width = 50;

        println!("{}", area(height, width))
    }

    {
        let rect = (50, 50);

        println!("area_from_tuple: {}", area_from_tuple(rect))
    }

    {
        let rect = Rectangle {
            width: 45,
            height: 20,
        };

        println!("area_from_struct: {}", area_from_struct(&rect));
        println!("{:#?}", rect);

        println!("area from Retangle method: {}", rect.area());
        println!("Is width from rect not zero? {}", rect.width());

        let rect2 = Rectangle {
            width: 45,
            height: 21,
        };

        println!("Can rect 2 hold rect 1? {}", rect2.can_hold(&rect));
        println!("Can rect 1 hold rect 2? {}", rect.can_hold(&rect2));
    }

    {
        println!(
            "square with size of 25 has an area of: {}px",
            Rectangle::square(50).area()
        )
    }
}

fn create_user(username: String, email: String) -> User {
    User {
        username,
        email,
        active: true,
        sign_in_count: 1,
    }
}

fn area(width: u32, height: u32) -> u32 {
    width * height
}

fn area_from_tuple(dimensions: (u32, u32)) -> u32 {
    dimensions.0 * dimensions.1
}

fn area_from_struct(rect: &Rectangle) -> u32 {
    rect.height * rect.width
}
