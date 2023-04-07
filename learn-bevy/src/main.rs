use bevy::prelude::*;

//Add startup só roda uma vez e é no inicio do jogo, similar a ready
//add system roda em todos os frames da nossa aplicação, similar a physics process
fn main() {
    App::new()
    .add_plugins(DefaultPlugins)
    .add_plugin(PeoplePlugin)
    .run()
}

pub struct  PeoplePlugin;

impl Plugin for  PeoplePlugin{
    fn build(&self, app: &mut App) {
        app.add_startup_system(setup)
        .add_system(hello_world)
        .add_system(print_names)
        .add_system(people_with_jobs)
        .add_system(people_without_jobs)
        .add_system(person_does_job);
    }
}

//Pub fn é uma função básica de rust
pub fn hello_world() {
    println!("Hello World!")
}
//Commands é uma estrutura fornecida pela Bevy
// E é utlizado para criar, adicionar e remover componentes das entidades
pub fn setup(mut commands: Commands) {
    commands.spawn((
        Person {
            name: "Alex".to_string(),
        },
        Employed {
            job: Job::Doctor,
        },
    ));
    commands.spawn(Person {
        name: "Luis".to_string(),
    }); 
    commands.spawn(Person {
        name: "Danilo".to_string(),
    }); 
    commands.spawn(Person {
        name: "Anderson".to_string(),
    }); 
    commands.spawn(Person {
        name: "Evangelista".to_string(),
    }); 
    commands.spawn(Person {
        name: "Vanderlei".to_string(),
    }); 

}

pub fn print_names(person_query: Query<&Person>){
    for person in person_query.iter() {
        println!("Name {}", person.name);
    }
}

//Uso de With
pub fn people_with_jobs(
    person_query: Query<&Person, With<Employed>>

) {
    for person in person_query.iter() {
        println!("{} has a job.", person.name);
    }
}

//Uso de without
pub fn people_without_jobs(
    person_query: Query<&Person, Without<Employed>>

) {
    for person in person_query.iter() {
        println!("{} is ready to hire.", person.name);
    }
}


pub fn person_does_job(
    person_query: Query<(&Person,&Employed)>
) {
    for (person, employed) in person_query.iter() {
        let job_name = match employed.job {
            Job::Doctor => "Doctor",
            Job::FireFighter => "Fire Fighter",
            Job::Lawyer => "Lawyer",
        };

        println!("{0} is a {1}", person.name, job_name);
    }

    
}
//Esse derive macro diz que o struct que implementado é um componente bevy
#[derive(Component)]
pub struct Person{
    pub name: String,
}

#[derive(Component)]
pub struct Employed{
    pub job: Job,
}

//Macros se aplicam apenas a parte que esta logo em seguida dela
#[derive(Debug)]
pub enum Job {
    Doctor,
    FireFighter,
    Lawyer
}