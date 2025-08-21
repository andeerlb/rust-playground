// unsigned means that value cannot be negative
// signed means that value can be negative

// i64, i32, i16, i8 mean signed integers
// u64, u32, u16, u8 mean unsigned integers

// 64 bits = 8 bytes, 32 bits = 4 bytes, 16 bits = 2 bytes, 8 bits = 1 byte

// 8 bytes = −9,223,372,036,854,775,808 a 9,223,372,036,854,775,807
// 4 bytes = −2,147,483,648 a 2,147,483,647
// 2 bytes = −32,768 a 32,767
// 1 byte = −128 a 127

fn main() {
    // Integer addition
    println!("1 + 2 = {}", 1u32 + 2);

    // Integer subtraction
    println!("1 - 2 = {}", 1i32 - 2);
    // TODO ^ Try changing `1i32` to `1u32` to see why the type is important

    // Scientific notation
    println!("1e4 is {}, -2.5e-3 is {}", 1e4, -2.5e-3);

    // Short-circuiting boolean logic
    println!("true AND false is {}", true && false);
    println!("true OR false is {}", true || false);
    println!("NOT true is {}", !true);

    // Bitwise operations
    println!("0011 AND 0101 is {:04b}", 0b0011u32 & 0b0101);
    println!("0011 OR 0101 is {:04b}", 0b0011u32 | 0b0101);
    println!("0011 XOR 0101 is {:04b}", 0b0011u32 ^ 0b0101);
    println!("1 << 5 is {}", 1u32 << 5);
    println!("0x80 >> 2 is 0x{:x}", 0x80u32 >> 2);

    // Use underscores to improve readability!
    println!("One million is written as {}", 1_000_000u32);
}
