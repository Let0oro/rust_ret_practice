use lib::input::Input;

fn main() {

    let ciclo = Input::<String>::new("Enter ciclo").read();
    let name = Input::<String>::new("Enter name").read();
    let age = Input::<u32>::new("Enter age").read();
    let weight = Input::<f32>::new("Enter weight").read();

    println!("Nombre: {name}\nEdad: {age}\nPeso: {weight}\nCiclo: {ciclo}");
}
