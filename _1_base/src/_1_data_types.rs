

pub(crate) fn main1() {
    // todo print type of link on value
    fn print_type_of<T>(_: &T) {
        println!("type: {0}", std::any::type_name::<T>(), )
    }

    // todo FULL INITIALIZE
    let var_int1: i32 = -12;

    // todo Print value on console
    println!("value: {0}", var_int1); // value: -12

    // todo Print type of link on value on console
    print_type_of(&var_int1); // type: i32

    // todo SHORT INITIALIZE
    let var_int2 = 12;
    println!("value: {0}", var_int2); // value: 12

    // todo REINITIALIZE
    let var_int2 = 15;
    println!("value: {0}", var_int2); // value: 15

    // todo Mutable var
    let mut var_int3: i32 = 100;
    println!("value: {0}", var_int3); // value: 100

    // todo Change value
    var_int3 = 50;
    println!("value: {0}", var_int3); // value: 50

    // todo CONSTANT - values it's has no changes
    const CONST_VAR_INT: i32 = -10;
    println!("value: {0}", CONST_VAR_INT); // value: -10

    // todo INT - values without float point
    let var_int11: i8 = 10;
    let var_int12: i16 = -10;
    let var_int13: i32 = 10;
    let var_int14: i64 = -10;
    let var_int15: i128 = 10;

    // todo UNSIGNED INT - only positive integers
    let var_uint11: u8 = 10;
    let var_uint12: u16 = 10;
    let var_uint13: u32 = 10;
    let var_uint14: u64 = 10;
    let var_uint15: u128 = 10;

    // todo FLOAT - values with float point
    let var_float11: f32 = 10.01;
    let var_float12: f64 = -10.01;

    // todo BOOLEAN - true or false
    let var_bool: bool = true;
    let var_bool: bool = false;

    // todo STRING - array of chars
    let var_string: &str = "Rust";

    // todo CHAR - one character
    let var_char: char = 'R';

    // todo TUPLE - constant array
    //               0    1     2       0      1      2
    let var_tuple: (i32, &str, bool) = (12, "Hello", true);
    println!("value: {0}", var_tuple.1); // value: Hello
    println!("value: {:?}", var_tuple); // value: (12, "Hello", true)

    // todo unpacking tuple
    let (x, y, z) = var_tuple;
    println!("value: {0}", x); // value: 12

    // todo ARRAY - array vars of some types
    let var_array: [i32; 3] = [1, 2, 3];
    println!("value: {0}", var_array[1]); // value: 2
    println!("value: {:?}", var_array); // value: [1, 2, 3]

    let mut var_array = [1, 2, 3];
    var_array[1] = 100;
    println!("value: {:?}", var_array); // value: [1, 100, 3]

    // todo VECTOR - resizable array
    let mut var_vector = vec![1, 10];
    println!("value: {0}", var_vector[0]); // value: 1
    println!("value: {:?}", var_vector); // value: [1, 10]

    // todo ADD value to vector
    var_vector.push(666);
    println!("value: {:?}", var_vector); // value: [1, 10, 666]

    // todo ADD value to vector
    var_vector.pop();
    var_vector.pop();
    println!("value: {:?}", var_vector); // value: [1]

    var_vector.reverse();
    var_vector.remove(1);
}
