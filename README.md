## Main Functionalities
* Store a database of
	* Dishes
		* Ingredients (Vegetables, Fruits, Dairy, Meats, Condiment)
			* Price
			* Lifespan
* Database can be updated at **any time**
	* New Dishes and Ingredients can be added
	* An Ingredient can have different prices
		* Prices for an Ingredient can be added
		* Average price will be showed
* Tell you what Dishes in the database you can make with input ingredients
---
### Nice to Haves (Not implemented yet)
* can store data about what ingredients you currently have
* can display usage priority based on ingredients' lifespans
---
## Structure
- **Database (SQLite)**
	- Category Table
		* id
		* category name(vegetable, fruit, dairy, meat, condiment)
	- Recipe Table
		* id
		* dish id
		* ingredient id
		* quantity (g)
	- Ingredient Table
		* id
		* category id
		* name
		* lifespan
	- Dish Table
		* id
		* name
	- Price Table
		* id
		* ingredient id
		* price
- **CLI (Rust)**
	* add recipe
	* add ingredient
	* add dish
	* add price to ingredient
	* show dishes
	* show ingredients of specific dishes
	* show dishes of specific ingredients
	* show lifespan of each ingredients
	* show average price of each ingredients 
	* delete recipe
	* delete dish along with recipes
