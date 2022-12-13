pub fn call() {
 
    println!("rust variable system basic usage");
    println!("with barrowing vaule");
    println!("with argument point");

    basic_usage_with_simple_variables();
}

struct VectorPoint {
    x: i32,
    y: i32,
    z: i32
}

impl VectorPoint {
    
    fn print(&self) {
        print!("x={} y={} z={}", self.x, self.y, self.z)
    }

}

fn print_type_of<T>(_: &T) {
    println!("type of = {}", std::any::type_name::<T>())
}

// https://doc.rust-lang.org/reference/types.html
fn basic_usage_with_simple_variables() {

    let simple_i32 = 32;
    println!("i32 value = {}", simple_i32);
    print_type_of(&simple_i32);

    let simple_str = "hello world!";
    println!("str value = {}", simple_str);
    print_type_of(&simple_str);

    let simple_string = String::from("Hello world!");
    println!("String value = {}", simple_string);
    print_type_of(&simple_string);

    let simple_float = 3.7_f32;
    println!("Float vaule = {}", simple_float);
    print_type_of(&simple_float);

    let simple_bool = true;
    println!("Boolean value = {}", simple_bool);
    print_type_of(&simple_bool);

    // byte
    let simple_byte = b'A';
    println!("byte value = {}", simple_byte);
    print_type_of(&simple_byte);

    //tuple
    let simple_tuple = (1, 2, false);
    println!("tuple value = {:#?}", simple_tuple);
    print_type_of(&simple_tuple);

    // array
    let simple_array = ["test", "test2", "test3", "test4", "test5"];
    println!("simple array vaule = {:#?}", simple_array);
    print_type_of(&simple_array);

    // slice
    let simple_slice = &simple_array[0..2];
    println!("simple slice value = {:#?}", simple_slice);
    print_type_of(&simple_slice);

    // Struct
    #[derive(Debug)]
    struct Position {
        x: i32,
        y: i32,
        z: i32,
        active: bool
    }
    let simple_struct = Position {x: 1, y: 1, z: 0, active: true};
    println!("simple struct value = {:#?}", simple_struct);
    print_type_of(&simple_struct);

    // enum
    #[derive(Debug)]
    enum GenderCategory {
       Male,Female
    }
    let simple_enum = GenderCategory::Male;
    println!("simple enum = {:#?}", simple_enum);
    print_type_of(&simple_enum);

    // union
    // references
    // raw pointer
    // func pointer

    // collections: https://doc.rust-lang.org/std/collections/index.html
    //1 Sequences: Vec, VecDeque, LinkedList
    //2 Maps: HashMap, BTreeMap
    //3 Sets: HashSet, BTreeSet
    //4 Misc: BinaryHeap

    // box: https://doc.rust-lang.org/std/boxed/index.html
    // cell; https://doc.rust-lang.org/std/cell/index.html
    // slice: https://doc.rust-lang.org/std/slice/index.html
    //trait object: https://doc.rust-lang.org/std/keyword.trait.html

}

fn basic_usage_string_struct() {

}

fn borrow_simple_variables() {

}

fn borrow_string_struct() {

}

fn basic_usage_enum() {

}

fn brrow_enum() {

}

fn ownership() {

}