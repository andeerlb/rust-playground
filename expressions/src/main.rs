fn main() {
    let x = 5u32;

    let y = {
        let x_squared = x * x;
        let x_cube = x_squared * x;

        // This expression will be assigned to `y`
        x_cube + x_squared + x
    };

    // In rust, the semicolon suppresses the expression
    // and returns the unit type `()`
    // For exemple, here `z` will get the value `()` 
    // because of the semicolon after `2 * x`
    // if you remove the semicolon, `z` will get the value `10`
    let z = {
        // The semicolon suppresses this expression and `()` is assigned to `z`
        2 * x;
    };

    // bellow is the same as above but without the semicolon
    // This expression will be assigned to `a`
    let a = {
        // The semicolon suppresses this expression and `()` is assigned to `a`
        2 * x
    };

    println!("x is {:?}", x);
    println!("y is {:?}", y);
    println!("a is {:?}", a);
    println!("z is {:?}", z);
}