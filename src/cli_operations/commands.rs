use bimap::BiMap;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Command {
    NewIngredient,
    AddPrice,
    NewDish,
    AddRecipe,
    ListAllDishes,
    ListAllIngredients,
    IHave,
    RecipeOf,
    DeleteIngredientFromRecipe,
    DeleteDish,
    DeleteIngredient,
    FetchDatabase,
    SyncDatabase,
    BackupDatabase,
    Quit,
    Help,
    Unknown,
    UpdateIngredient,
    UpdateDishName,
}

pub fn get_command_bimap() -> BiMap<Command, String> {
    let mut bimap = BiMap::new();
    bimap.insert(Command::NewIngredient, "new ingredient".to_string());
    bimap.insert(Command::AddPrice, "add price".to_string());
    bimap.insert(Command::NewDish, "new dish".to_string());
    bimap.insert(Command::AddRecipe, "add recipe".to_string());
    bimap.insert(Command::ListAllDishes, "list all dishes".to_string());
    bimap.insert(Command::ListAllIngredients, "list all ingredients".to_string());
    bimap.insert(Command::IHave, "i have".to_string());
    bimap.insert(Command::RecipeOf, "recipe of".to_string());
    bimap.insert(Command::DeleteIngredientFromRecipe, "delete ingredient from recipe".to_string());
    bimap.insert(Command::DeleteDish, "delete dish".to_string());
    bimap.insert(Command::DeleteIngredient, "delete ingredient".to_string());
    bimap.insert(Command::FetchDatabase, "fetch database".to_string());
    bimap.insert(Command::SyncDatabase, "sync database".to_string());
    bimap.insert(Command::BackupDatabase, "backup database".to_string());
    bimap.insert(Command::Help, "help".to_string());
    bimap.insert(Command::Quit, "quit".to_string());
    bimap.insert(Command::Unknown, "unknown".to_string());
    bimap.insert(Command::UpdateIngredient, "update ingredient".to_string());
    bimap.insert(Command::UpdateDishName, "update dish".to_string());

    bimap
}