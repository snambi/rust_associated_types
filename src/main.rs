
pub trait Animal<RHS=Self>{
    type Bird;

    fn chase( self , bird: Self::Bird ) -> Self::Bird;

    fn hello(){
        println!("hello bird");
    }
}

struct Dog{
    name: String,
}

impl Animal for Dog{
    type Bird = String;

    fn chase( self, bird:Self::Bird ) -> Self::Bird {
        println!("{} chasing {}", self.name, bird);
        bird
    }
}

fn main() {

    let dog = Dog{ name: String::from("Jimmy dog") };

    dog.chase( String::from("owl") );

    Dog::hello();
}
