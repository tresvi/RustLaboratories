#[derive(Debug)]
enum Bebida {
    Cafe(Tamano),
    Te(Tamano),
    Chocolate(Tamano),
}
 
#[derive(Debug)]
enum Tamano {
    Pequenio,
    Mediano,
    Grande,
}
 
// Función para obtener el precio de la bebida
fn obtener_precio(bebida: &Bebida) -> f64 {
    match bebida {
        Bebida::Cafe(tamano) => match tamano {
            Tamano::Pequenio => 2.0,
            Tamano::Mediano => 2.5,
            Tamano::Grande => 3.0,
        },
        Bebida::Te(tamano) => match tamano {
            Tamano::Pequenio => 1.5,
            Tamano::Mediano => 2.0,
            Tamano::Grande => 2.5,
        },
        Bebida::Chocolate(tamano) => match tamano {
            Tamano::Pequenio => 2.2,
            Tamano::Mediano => 2.7,
            Tamano::Grande => 3.2,
        },
    }
}
 
// Función para aplicar descuento si es café grande
fn aplicar_descuento(bebida: &Bebida, precio: f64) -> f64 {
    if let Bebida::Cafe(Tamano::Grande) = bebida {
        return precio * 0.9; // Aplica un 10% de descuento
    }
    precio
}
 
fn main() {
    let pedido1 = Bebida::Cafe(Tamano::Grande);
    let pedido2 = Bebida::Te(Tamano::Mediano);
    let pedido3 = Bebida::Chocolate(Tamano::Pequenio);
 
    let precio1 = obtener_precio(&pedido1);
    let precio2 = obtener_precio(&pedido2);
    let precio3 = obtener_precio(&pedido3);
 
    let precio_final1 = aplicar_descuento(&pedido1, precio1);
    let precio_final2 = aplicar_descuento(&pedido2, precio2);
    let precio_final3 = aplicar_descuento(&pedido3, precio3);
 
    println!("Pedido 1: {:?}, Precio Final: ${:.2}", pedido1, precio_final1);
    println!("Pedido 2: {:?}, Precio Final: ${:.2}", pedido2, precio_final2);
    println!("Pedido 3: {:?}, Precio Final: ${:.2}", pedido3, precio_final3);
}



/*
// Otra implementacion mas floja

#[derive(Debug)]        //Para impresion del enum
enum Bebida {
    Cafe,
    Te,
    Chocolate,
}

#[derive(Debug)]        //Para impresion enum
enum Tamano {
    Pequeno,
    Mediano,
    Grande,
}

fn main() {
    
    let bebida: Bebida = Bebida::Cafe;
    let tamano: Tamano = Tamano::Grande;

    let mut precio: f32 = obtener_precio(&bebida, &tamano);
    let mut precio: f32 = aplicar_descuentos(&bebida, &tamano, &precio);
  
    println!("Precio {0:.2}", precio);
}

fn obtener_precio (bebida: &Bebida, tamano: &Tamano) -> f32{
    match (bebida, tamano) {
        (Bebida::Cafe, Tamano::Pequeno) => { return 2.0 }
        (Bebida::Cafe, Tamano::Mediano) => { return 2.5 }
        (Bebida::Cafe, Tamano::Grande) => { return 3.0 }

        (Bebida::Te, Tamano::Pequeno) => { return 1.5 }
        (Bebida::Te, Tamano::Mediano) => { return 2.0 }
        (Bebida::Te, Tamano::Grande) => { return 2.5 }

        (Bebida::Te, Tamano::Pequeno) => { return 2.2 }
        (Bebida::Te, Tamano::Mediano) => { return 2.7 }
        (Bebida::Te, Tamano::Grande) => { return 3.2 }

        (_, _) => { return 0.0 }
    }
}

fn aplicar_descuentos(bebida: &Bebida, tamano: &Tamano, precio: &f32) -> f32{
    if let (Bebida::Cafe, Tamano::Grande) = (bebida, tamano){
        return (precio * 0.9) as f32;
    } else {
        return *precio;
    }
}
*/