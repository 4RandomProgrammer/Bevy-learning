use bevy::prelude::*;

//Add startup só roda uma vez e é no inicio do jogo, similar a ready
//add system roda em todos os frames da nossa aplicação, similar a physics process
fn main() {
    App::new()
    .add_startup_system(setup)
    .add_system(hello_world)
    .add_system(print_names)
    .run()
}

//Pub fn é uma função básica de rust
pub fn hello_world() {
    println!("Hello World!")
}
//Commands é uma estrutura fornecida pela Bevy
// E é utlizado para criar, adicionar e remover componentes das entidades
pub fn setup(mut commands: Commands) {
    commands.spawn(Person {
        name: "Alex".to_string(),
    }); 
}

pub fn print_names(person_query: Query<&Person>){
    for person in person_query.iter() {
        println!("Name {}", person.name);
    }
}

//Esse derive macro diz que o struct que implementado é um componente bevy
#[derive(Component)]
pub struct Person{
    pub name: String,
}