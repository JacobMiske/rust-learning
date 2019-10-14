fn main() {
    println!("Tuples!");
    let x: (i32, f64, u8) = (500, 6.4, 1);

    let _five_hundred = x.0;

    let _six_point_four = x.1;

    let _one = x.2;

    println!("Arrays!");
    let a = [1, 2, 3, 4, 5];

    let _first = a[0];
    let _second = a[1];

    println!("Functions!");
    second_function(5, 5);

    println!("Statements");
    let _x = 5;
    let _y = {
        let x =3;
        x + 1
    };
}

fn second_function(x: i32, y: i32) {
    println!("The value of x is {}", x);
    println!("The value of y is {}", y);
}