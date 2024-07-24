use std::io;

const BLOCKED: i32 = -1;
const NORTH : usize = 0;
const EAST : usize = 1;
const SOUTH : usize = 2;
const WEST : usize = 3;

struct Item {
    name: &'static str,
    description: &'static str,
}

struct Player {
    name: String,
    location: usize,
    strength: i32,
    dexterity: i32,
    intelligence: i32,
    wisdom: i32,
    charisma: i32,
    constitution: i32,
    inventory: Vec<Item>,
}

struct Location {
    description: &'static str,
    exits: [i32; 4],
    items: Vec<Item>,
}

fn load_level() -> Vec<Location> {
    vec![
        Location{
            description : "You are in the Start Room", 
            exits : [1, 2, 3, 4], 
            items : vec![]
        },
        Location{
            description : "You are in a dark room.", 
            exits : [BLOCKED, BLOCKED, 0, BLOCKED],
            items : vec![ Item{name: "key", description: "A shiny key"}]
        },
        Location{
            description : "You are in a bright room.", 
            exits : [BLOCKED, BLOCKED, BLOCKED, 0],
            items : vec![]
        },
        Location{
            description : "You are in a room with a door.", 
            exits : [0, BLOCKED, BLOCKED, BLOCKED],
            items : vec![]
        },
        Location{
            description : "You are in the kitchen", 
            exits : [BLOCKED, 0, BLOCKED, BLOCKED],
            items : vec![]},
    ]
}

fn print_items(items: &Vec<Item>) {
    for (i, item) in items.iter().enumerate() {
        println!("{}) {}", i+1, item.name);
    }
}

fn pickup_item(player: &mut Player, location: &mut Location, item_index: usize) {
    let item = location.items.remove(item_index);
    player.inventory.push(item);
    println!("\nYou picked up {}\n", player.inventory.last().unwrap().name);
}

fn drop_item(player: &mut Player, location: &mut Location, item_index: usize) {
    let item = player.inventory.remove(item_index);
    location.items.push(item);
    println!("\nYou dropped {}\n", location.items.last().unwrap().name);
}

fn print_character(player: &Player) {
    println!("Name: {}", player.name);
    println!("Strength: {}", player.strength);
    println!("Dexterity: {}", player.dexterity);
    println!("Intelligence: {}", player.intelligence);
    println!("Wisdom: {}", player.wisdom);
    println!("Charisma: {}", player.charisma);
    println!("Constitution: {}", player.constitution);
}

fn move_player(player: &mut Player, location: &Location, direction: usize) -> usize {
    if location.exits[direction] != BLOCKED {
        player.location = location.exits[direction] as usize;
    } else {
        println!("You can't go that way!");
    }
    player.location
}

fn main() {
    println!("Welcome to Single User Dungeon!");

    // structure of each location is a tuple with a description string and an array of 4 i32s indicating
    // the index of the location to the north, east, south, and west of the current location or
    // BLOCKED if there is no location in that direction
    let mut locations = load_level();
    let mut player = Player{
        name: "Malaidas".to_string(), 
        location: 0, 
        strength: 10,
        dexterity: 10,
        intelligence: 10,
        wisdom: 10,
        charisma: 10,
        constitution: 10,
        inventory: 
        vec![]};
    // usize is ok here because player.location can never be set to BLOCKED

    loop {
        
        println!("{}", locations[player.location].description);
        if locations[player.location].items.len() > 0 {
            println!("\nin this room there {}: " , if locations[player.location].items.len() > 1 {"are"} else {"is"});
            print_items(&locations[player.location].items);
        }
        println!("What would you like to do? : ");

        let mut input = String::new();
        io::stdin()
            .read_line(&mut input)
            .expect("Failed to read line");

        let command = input.trim().to_lowercase();
        let command: Vec<&str> = command.split_whitespace().collect();
        let current_location = player.location;
        match &command[0] as &str {
            "n" => {
                move_player(&mut player, &locations[current_location], NORTH);
            }
            "e" => {
                move_player(&mut player, &locations[current_location], EAST);
            }
            "s" => {
                move_player(&mut player, &locations[current_location], SOUTH);
            }
            "w" => {
                move_player(&mut player, &locations[current_location], WEST);
            }
            "pickup" =>{
                // accepts a string name for an item and then checks the current location for the item
                // then adds the item to the player's inventory
                if command.len() < 2 {
                    println!("Please specify an item to pickup!");
                } else {
                    let item_name = command[1];
                    let mut found = false;
                    for (i, item) in locations[current_location].items.iter().enumerate() {
                        if item.name == item_name {
                            pickup_item(&mut player, &mut locations[current_location], i);
                            found = true;
                            break;
                        }
                    }
                    if !found {
                        println!("Item not found!");
                    }
                }
            }
            "drop" =>{
                // accepts a string name for an item and then checks the player's inventory for the item
                // then adds the item to the current location
                if command.len() < 2 {
                    println!("Please specify an item to drop!");
                } else {
                    let item_name = command[1];
                    let mut found = false;
                    for (i, item) in player.inventory.iter().enumerate() {
                        if item.name == item_name {
                            drop_item(&mut player, &mut locations[current_location], i);
                            found = true;
                            break;
                        }
                    }
                    if !found {
                        println!("Item not found!");
                    }
                }
            }
            "examine" | "x" => {
                if command.len() < 2 {
                    println!("Please specify an item to examine!");
                } else {
                  // accepts the name of an item and then checks first inventory and then the current location for the item
                  // then prints the description of the item
                    let item_name = command[1];
                    let mut found = false;
                    for item in &player.inventory {
                        if item.name == item_name {
                            println!("{}", item.description);
                            found = true;
                            break;
                        }
                    }
                    if !found {
                        for item in &locations[current_location].items {
                            if item.name == item_name {
                                println!("{}", item.description);
                                break;
                            }
                        }
                    }
                }
            }
            "i" => {
                // print the player's inventory
                if player.inventory.len() == 0 {
                    println!("You have no items!");
                } else {
                    println!("You have the following items: ");
                    print_items(&player.inventory);
                }
            }
            "c" =>{
                // print the characters details
                print_character(&player);
            }
            "quit" | "q" => {
                println!("Goodbye!");
                break;
            }
            _ => {
                println!("Invalid input!");
            }
        }
    }
}
