use std::collections::HashMap;
use bevy::prelude::*;

pub struct FactorioPlugin;

impl Plugin for FactorioPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(Storage::new())
            .add_system(storage_system);
    }
}

fn storage_system (storage : Res<Storage>) {
    println!("num in storage {}", storage.storage.len());
}

fn setup_storage(mut commands: Commands){
    commands.spawn()
}

#[derive(Debug, Resource)]
pub struct Storage {
    storage: HashMap<ItemType, i32>,
}

impl Storage {
    pub fn new() -> Storage {
        let mut storage = HashMap::new();
        storage.insert(ItemType::CopperOre, 20);
        storage.insert(ItemType::IronOre, 50);
        storage.insert(ItemType::Stone, 200);
        storage.insert(ItemType::Coal, 0);
        Storage {
            storage
        }
    }

    pub fn get_item(&mut self, item: ItemType, amount: i32) -> GetStorageResult {
        if self.storage.contains_key(&item) {
            let amount_available = self.storage.get(&item).expect("Could not find item in storage");
            if *amount_available >= amount {
                self.storage.insert(item, amount_available - amount);
                return GetStorageResult::Ok(amount);
            } else {
                return GetStorageResult::Empty;
            }
        } else {
            return GetStorageResult::Empty;
        }
    }
}

#[derive(Debug)]
pub enum GetStorageResult {
    Ok(i32),
    Empty,
}

#[derive(Debug)]
pub struct ItemDescription {
    pub kind: ItemType,
    pub name: String,
    pub description: String,
    pub burn_value: Option<f32>,
}

// pub struct Item {
//     kind: ItemType,
//
// }

#[derive(Debug,Eq, PartialEq, Hash)]
pub enum ItemType {
    Coal,
    Stone,
    IronOre,
    CopperOre
}