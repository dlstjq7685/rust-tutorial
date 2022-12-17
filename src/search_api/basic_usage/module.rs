pub fn call() {
 
    println!("rust variable system basic usage");
    println!("with barrowing vaule");
    println!("with argument point");

    basic_usage_with_simple_variables();
    basic_ownership();
    basic_browwing();
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


//Here are some of the types that implement Copy:(!!!Dont Happen OWNER MOVE!!!IT JUST VALUE COPY.)
//All the integer types, such as u32.
//The Boolean type, bool, with values true and false.
//All the floating point types, such as f64.
//The character type, char.
//Tuples, if they only contain types that also implement Copy. For example, (i32, i32) implements Copy, but (i32, String) does not.
fn basic_ownership() {
    let v = vec![1, 2, 3];

    let v2 = v;

    // ownership moved.
    //println!("{}", v);

    println!("v2[0] is: {}", v2[0]);
    
    let s1 = String::from("hello");
    let s2 = s1;

    //println!("s1 value = {}", s1);
    println!("s2 value = {}", s2);
    
}

fn basic_browwing() {

    // Dont have owner-ship
    fn calculate_length(s: &String) -> usize {
        s.len()
    }

    // Dont have owner-ship
    fn change(some_string: &mut String) {
        some_string.push_str(", world");
    }

    // Dont have owner-ship
    fn add_val(target: &mut i32) {
        // reference value.
        *target = *target + 100
    }

    
    let s1 = String::from("hello");

    let len = calculate_length(&s1);

    println!("The length of '{}' is {}.", s1, len);

    let mut s = String::from("hello");

    change(&mut s);

    println!("string valuei = {}", s);

    let mut i = 1;
    add_val(&mut i);

    println!("add 100 from i = {}", i)


}

fn basic_pointer() {



}
