
struct Vehicle{
    brand: String,
    model: String,
    year: u32,
    mileage: f32
}

impl Vehicle{
    fn display_info(&self){
        println!("Marca: {0}", self.brand);
        println!("Modelo: {0}", self.model);
        println!("Año: {0}", self.year);
        println!("Mileage: {0}", self.mileage);
    }

    fn update_mileage(&mut self, new_mileage: f32){
        self.mileage = new_mileage;
    }
}


fn main() {
    println!("*****Iniciando programa*****\n");
    let mut vehicle = Vehicle{
        brand : String::from("Peugeot"),
        model : String::from("208"),
        year : 2023,
        mileage : 13955.3,
    };
    
    vehicle.display_info();
    
    // Tambien se puede mostrar asi
    //vehicle.brand = String::from("Peugeot");
    //vehicle.mileage = 1111.1;
    println!("\nKilometraje actualizado\n");

    vehicle.update_mileage(10000.1);
    vehicle.display_info();
}
