fn main() {
    // Paso 1: Declaración de variables básicas
    let product_name: String = String::from("Telefono Xiaomi");
    let mut price: f32 = 10000.50; // Precio en dólares
    let quantity: u32 = 10;
    let on_sale: bool = true;

    // Paso 2: Calcular el valor total del inventario
    let mut total_value: f32 = price * quantity as f32;

    if on_sale {
        total_value *= 0.9;
    }

    // Paso 3: Conversión y formateo de datos
    let price_as_f64: f64 = price as f64; // Convertir `f32` a `f64`
    let quantity_as_string: String = format!("Cantidad disponible: {} unidades", quantity);

    // Paso 4: Imprimir el resultado
    println!("\n**************************");
    println!("Información del producto:");
    println!("Nombre: {}", product_name);
    println!("Precio unitario: ${:.2}", price_as_f64);
    println!("{}", quantity_as_string);
    println!("Valor total del inventario: ${:.2}", total_value);
    println!("Está en oferta?: {}", if on_sale { "Sí" } else { "No" });
}

