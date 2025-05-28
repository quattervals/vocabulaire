use cucumber::{World, given, then, when};

// These `Cat` definitions would normally be inside your project's code,
// not test code, but we create them here for the show case.
#[derive(Debug, Default)]
struct Cat {
    pub hungry: bool,
}

impl Cat {
    fn feed(&mut self) {
        self.hungry = false;
    }
}

// `World` is your shared, likely mutable state.
// Cucumber constructs it via `Default::default()` for each scenario.
#[derive(Debug, Default, World)]
pub struct AnimalWorld {
    cat: Cat,
}

// #[given(regex = r"^a (hungry|satiated) cat$")]
#[given(expr = "a {word} cat")]
fn hungry_cat(world: &mut AnimalWorld, state: String) {
    match state.as_str() {
        "hungry" => world.cat.hungry = true,
        "satiated" => world.cat.hungry = false,
        _ => unreachable!(),
    }
}

#[when("I feed the cat")]
fn feed_cat(world: &mut AnimalWorld) {
    world.cat.feed();
}

#[then("the cat is not hungry")]
fn cat_is_fed(world: &mut AnimalWorld) {
    assert!(!world.cat.hungry);
}

#[cfg(test)]
use vocabulaire::test_utils::utils::shared;

use vocabulaire::driven::repository::mongo_repository;

#[derive(Default, World)]
pub struct DatabaseWorld {
    repo: Option<mongo_repository::VociMongoRepository>,
}
// impl Default for DatabaseWorld {
//     fn default() -> Self {
//         Self { repo: None }
//     }
// }

#[given("a clean database is available")]
async fn setup_database(world: &mut DatabaseWorld) {
    world.repo = Some(shared::setup_repo().await);
}

// This runs before everything else, so you can set up things here.
fn main() {
    // You may choose any executor you like (`tokio`, `async-std`, etc.).
    // You may even have an `async` main, it doesn't matter. The point is that
    // Cucumber is composable. :)
    futures::executor::block_on(AnimalWorld::run("tests/features/simple.feature"));
    futures::executor::block_on(AnimalWorld::run("tests/features/db.feature"));
}
