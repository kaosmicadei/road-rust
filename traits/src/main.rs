trait Shape {
    fn area(&self) -> f32;
}

struct Circle { radius: f32 }
struct Rectangle { width: f32, height: f32}

impl Shape for Circle {
    fn area(&self) -> f32 { 3.14 * self.radius * self.radius }
}

impl Shape for Rectangle {
    fn area(&self) -> f32 { self.width * self.height }
}

fn print_info_shape<T: Shape>(s: T) {
    println!{"The shape has area: {}", s.area()};
}

fn main() {
    let circle = Circle{radius: 3.0};
    let rectangle = Rectangle{ width: 2.3, height: 3.1};
    
    print_info_shape(circle);
    print_info_shape(rectangle);
}
