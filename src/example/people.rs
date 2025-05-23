use bevy::prelude::{App, Commands, Component, Plugin, Query, Startup, Update, With, Without};

#[derive(Component)]
struct Person {
    pub(crate) name: String,
}

#[derive(Component)]
struct Employment {
    pub(crate) job: Job,
}

#[derive(Debug)]
enum Job {
    Doctor,
    FireFighter,
    Lawyer,
}

pub struct PeoplePlugin;

impl Plugin for PeoplePlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, spawn_person)
            .add_systems(Startup, spawn_person_with_job)
            .add_systems(Update, print_all_person)
            .add_systems(Update, print_person_with_job)
            .add_systems(Update, print_person_without_job)
            .add_systems(Update, print_person_with_their_job_name);
    }
}

pub fn spawn_person(mut commands: Commands) {
    commands.spawn(Person {
        name: "Alex".to_string(),
    });
    commands.spawn(Person {
        name: "Bob".to_string(),
    });
    commands.spawn(Person {
        name: "Charlie".to_string(),
    });
    commands.spawn(Person {
        name: "David".to_string(),
    });
    commands.spawn(Person {
        name: "Elen".to_string(),
    });
}

pub fn spawn_person_with_job(mut commands: Commands) {
    commands.spawn((
        Person {
            name: "Faye".to_string(),
        },
        Employment { job: Job::Doctor },
    ));
    commands.spawn((
        Person {
            name: "George".to_string(),
        },
        Employment {
            job: Job::FireFighter,
        },
    ));
    commands.spawn((
        Person {
            name: "Hayden".to_string(),
        },
        Employment { job: Job::Lawyer },
    ));
    commands.spawn((
        Person {
            name: "Isaac".to_string(),
        },
        Employment { job: Job::Doctor },
    ));
    commands.spawn((
        Person {
            name: "Jade".to_string(),
        },
        Employment {
            job: Job::FireFighter,
        },
    ));
}

fn print_all_person(person_query: Query<&Person>) {
    for person in person_query.iter() {
        println!("Person name: {}", person.name);
    }
}

fn print_person_with_job(person_query: Query<&Person, With<Employment>>) {
    for person in person_query.iter() {
        println!("Person name: {} has a job", person.name);
    }
}

fn print_person_without_job(person_query: Query<&Person, Without<Employment>>) {
    for person in person_query.iter() {
        println!("Person name: {} is ready for employment", person.name);
    }
}

fn print_person_with_their_job_name(person_employment_query: Query<(&Person, &Employment)>) {
    for (person, employment) in person_employment_query.iter() {
        println!(
            "Person name: {} has a job as {:?}",
            person.name, employment.job
        );
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use bevy::app::{App, Startup};

    #[test]
    fn did_spawn_5_people_without_job() {
        let mut app = App::new();
        app.add_systems(Startup, spawn_person);
        app.update();

        // 5 people should be spawned
        assert_eq!(
            app.world_mut().query::<&Person>().iter(app.world()).len(),
            5
        );

        app.update();

        // There should still be 5 people
        assert_eq!(
            app.world_mut().query::<&Person>().iter(app.world()).len(),
            5
        );
    }

    #[test]
    fn did_spawn_5_people_with_job() {
        let mut app = App::new();
        app.add_systems(Startup, spawn_person_with_job);
        app.update();

        // 5 people with job should be spawned
        assert_eq!(
            app.world_mut()
                .query::<(&Person, &Employment)>()
                .iter(app.world())
                .len(),
            5
        );

        app.update();

        // There should still be 5 people with job
        assert_eq!(
            app.world_mut()
                .query::<(&Person, &Employment)>()
                .iter(app.world())
                .len(),
            5
        );
    }
}
