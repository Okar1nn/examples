mod sausage_factory {
    // fn get_secret_recipe() -> String {
    const fn get_secret_recipe() -> &str {
        // String::from("Ginger")
        "Ginger"
    }

    pub fn make_sausage() {
        get_secret_recipe();
        println!("sausage!");
    }
}

fn main() {
    sausage_factory::make_sausage();
}
