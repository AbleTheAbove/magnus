use crate::entity::Behaviour;
use rhai::{Engine, ImmutableString, RegisterFn};
use std::fmt::{Debug, Display};

#[derive(Debug)]
struct Addon {
    addon_id: String,
    //authors: String,
    description: String,
}

pub fn run_addon_temp() {
    //    engine.register_fn("register_addon", register_addon::<ImmutableString>);

    let mut engine = Engine::new();

    engine.register_fn("print", addon_log::<ImmutableString>);
    engine.register_fn("register_addon", register_addon::<ImmutableString>);
    engine.register_fn("register_entity", register_entity::<ImmutableString>);
    engine.register_fn("register_melee", register_melee::<ImmutableString>);
    engine.register_fn("finish", exit_mod);
    engine.register_fn(
        "register_decoration",
        register_decoration::<ImmutableString>,
    );

    let result = engine.eval_file::<()>("addons/hello_world.mag".into());
    //println!("{:?}", result);
    match result {
        Err(e) => {
            println!("Error: {:?}", e);
        }
        Ok(()) => {}
    }
}

// Log to individual file!
fn addon_log<ImmutableString: Debug>(x: ImmutableString) {
    let _y = Behaviour::Freindly;
    println!("put up a good show: {:?}!", x)
}

fn register_addon<ImmutableString: Display + Debug>(
    addon_id: ImmutableString,
    description: ImmutableString,
) {
    let addon = Addon {
        addon_id: addon_id.to_string(),
        //    authors: ,
        description: description.to_string(),
    };
    println!("{:?}", addon);
}

// A decoration is a placeable item with no features
// Example: A flower
fn register_decoration<ImmutableString>(_id: String, _icon: char, _description: String) {
    println!("ok");
}

fn register_melee<ImmutableString: Debug>(id: ImmutableString) {
    println!("{:?}", id);
}

fn register_entity<ImmutableString>(_id: ImmutableString, _icon: char) {}

fn exit_mod() {}
