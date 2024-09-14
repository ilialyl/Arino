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
    FetchDatabase,
    SyncDatabase,
    BackupDatabase,
    Quit,
    Help,
    Unknown,
}

impl Command {
    pub fn to_str(&self) -> &'static str {
        match self {
            Command::NewIngredient => "new ingredient",
            Command::AddPrice => "add price",
            Command::NewDish => "new dish",
            Command::AddRecipe => "add recipe",
            Command::ListAllDishes => "list all dishes",
            Command::ListAllIngredients => "list all ingredients",
            Command::IHave => "i have",
            Command::RecipeOf => "recipe of",
            Command::DeleteIngredientFromRecipe => "delete ingredient from recipe",
            Command::DeleteDish => "delete dish",
            Command::FetchDatabase => "fetch database",
            Command::SyncDatabase => "sync database",
            Command::BackupDatabase => "backup database",
            Command::Help => "help",
            Command::Quit => "quit",
            Command::Unknown => "unknown",
        }
    }
}
