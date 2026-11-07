use std::fmt;

// Enum for types of pet animals
enum PetType {
    Crocodile,
    Rat,
}

// Struct for pet
struct Pet {
    name: String,
    species: PetType,
}

// Setting the values the user gave to the pet
fn build_pet(name: String, species: PetType) -> Pet {
    Pet {
        name,
        species,
    }
}

// Display for PetType so the species can be printed
impl fmt::Display for PetType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let name = match self {
            PetType::Crocodile => "Crocodile",
            PetType::Rat => "Rat",
        };
        write!(f, "{}", name)
    }
}

// Display function for Pet so that it is printable
impl fmt::Display for Pet {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "({}, {})", self.name, self.species)
    }
}

// Function for reading user input
fn read_user_input (instructions: &str) -> String {
    use std::io::{stdin,stdout,Write};

    let mut s=String::new();

    print!("{}", instructions);
    let _=stdout().flush();
    stdin().read_line(&mut s).expect("Did not enter a correct string");

    if let Some('\n')=s.chars().next_back() {
        s.pop();
    }
    if let Some('\r')=s.chars().next_back() {
        s.pop();
    }
    return s;
}

// Prompt the user for pet information when the program is ran
fn prompt_user() -> (String, PetType) {
    
    let s = loop {
        let instructions_choose_species = "Choose which type of pet you want. Type C for Crocodile or R for Rat: ";
        let species_answer = read_user_input(instructions_choose_species);

        match species_answer.as_str() {
            "R" => break PetType::Rat,
            "C" => break PetType::Crocodile,
            _ => println!("Incorrect pet type!"),
        }
    };

    let instructions_set_name = "Set a name for the pet: ";
    let name = read_user_input(instructions_set_name);

    (name, s)

}

fn main() {
    // Prompting the user for the name and the species of the pet
    let (name, species) = prompt_user();

    // Building the pet
    let pet = build_pet(name, species);

    // Printing the pet
    println!("{}", pet);
}
