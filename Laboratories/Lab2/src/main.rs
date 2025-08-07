fn main() {
    
    let frase: String = String::from("Hola mundo");
    let primer_palabra = first_word(&frase);
    let longitud: usize = calculate_length(&frase);

    println!("La frase elegida es: {frase}");
    println!("La primera palabra es {primer_palabra}, y la longitud de la frase es {longitud}");

    let mut x = 1;
    increment_length(&mut x);
    println!("Valor de x: {x}");
        increment_length(&mut x);
    println!("Valor de x: {x}");
}

//Punto 1
fn first_word(s:&String) -> &str{
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..i]
        }
    }

    &s[..]
}

//Punto 2
fn calculate_length(s: &String) -> usize {
    let length_chars: usize = s.chars().count();
    length_chars

    //o bien s.len()

    //o bien, puedo retornar con return y ';' al final
    //return length_chars;
}

//Punto 2
fn increment_length(x: &mut i32) {
    *x = *x + 1;
}