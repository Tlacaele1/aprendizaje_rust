fn main(){
 
 //Definir los rectángulos
 let rect1 = Rectangulo {altura: 20, ancho: 40};
 let rect2 = Rectangulo {altura: 22, ancho: 39};

 println!("¿Cabe el rectángulo 1 en el 2? {}", rect2.cabe(&rect1));
}

// definir la estructura
struct Rectangulo {
    altura: i32,
    ancho: i32,
}

//implementar los métodos de la estructura
impl Rectangulo {
    fn area(&self)->i32{
        self.altura*self.ancho
    }

    //Pasamos los dos rectángulos como parámetros y dentro llamamos al método area.
    fn cabe(self, otro: &Rectangulo) -> bool {
        self.area() > otro.area()
    }

}