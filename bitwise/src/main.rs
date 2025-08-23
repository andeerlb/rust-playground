// One bit is reserved to indicate whether the number is positive or negative.
// The number is positive because the first bit is 0
// 00000000 00000000 00000000 00000001
// ˆ signal bit = 0, it means that the value is positive

// The number is negative because the first bit is 1
// 11111111 11111111 11111111 11111110
// ˆ signal bit = 1, it means that the value is negative

fn main() {
    let num1: i32 = 5;   // binary: 0101
    let num2: i32 = 9;   // binary: 1001

    let result = num1 & num2; // binary: 0001
    // result like a bit is = 00000000 00000000 00000000 00000001

    // Summarizing: the & operator goes through each bit and keeps 1 only when both numbers have 1 in the same position
    // In this case, only the last bit is 1 for both numbers.
    // 0|1|0|1 -> compare bit 1 with 1, bit 0 with 0, bit 1 with 0, bit 0 with 1
    // 1|0|0|1
    // -------
    // 0|0|0|1

    println!("num1: {:010b}", num1);
    // {:010b} → format as binary
    // :       → begins format specification
    // 0       → pad with zeros (on the left)
    // 10      → total width = 10 characters
    // b       → binary representation
    // Example (num1 = 5): 0000000101
    println!("num2: {:#010b}", num2);
    // {:#010b} → format as binary with prefix
    // #        → add "0b" prefix
    // 0        → pad with zeros
    // 10       → total width = 10 characters (prefix counts in width)
    // b        → binary representation
    // Example (num2 = 9): 0b00001001
    println!("The result of bitwise AND is: {}", result);
    println!("The result of bitwise NOT is: {}", !result); // \!result like a bit is = 11111111 11111111 11111111 11111110 (will invert each bit, if earlier 1 becomes 0 and vice versa)
    println!("The result of bitwise OR is: {}", num1 | num2);
    println!("The result of bitwise XOR is: {}", num1 ^ num2);
    println!("The result of bitwise left shift is: {}", num1 << 1);
    println!("The result of bitwise right shift is: {}", num1 >> 1);


    // rust formatting
    let num = 42;
    // Binary formatting
    println!("{:010b}", num); 
    // {:010b} → binary format
    // :       → starts format spec
    // 0       → pad with zeros
    // 10      → total width = 10 characters
    // b       → binary
    // Example (num = 42): 0000101010

    println!("{:#010b}", num); 
    // {:#010b} → binary with prefix
    // #       → add "0b" prefix
    // 0       → pad with zeros
    // 10      → total width (including prefix)
    // b       → binary
    // Example: 0b0000101010

    // Hexadecimal formatting
    println!("{:08x}", num); 
    // {:08x} → hex format (lowercase)
    // 0       → pad with zeros
    // 8       → total width = 8
    // x       → hexadecimal (lowercase)
    // Example: 0000002a

    println!("{:#08x}", num); 
    // {:#08x} → hex with prefix
    // #       → add "0x" prefix
    // 0       → pad with zeros
    // 8       → total width (including prefix)
    // x       → hex
    // Example: 0x00002a

    // Octal formatting
    println!("{:08o}", num); 
    // {:08o} → octal format
    // 0       → pad with zeros
    // 8       → total width = 8
    // o       → octal
    // Example: 00000052

    println!("{:#08o}", num); 
    // {:#08o} → octal with prefix
    // #       → add "0o" prefix
    // 0       → pad with zeros
    // 8       → total width (including prefix)
    // o       → octal
    // Example: 0o000052
}
