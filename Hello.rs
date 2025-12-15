fn main()
{
    //Primitive datatypes
    println!("Hi");

    let x: i32 = 42; //32 bits of data, can be negative
    let y: u64 = 100; //64 bits, cant be negative

    println!("Signed Integer: {}", x);
    println!("Signed Integer: {}", y);

    //Floats: f32, f64
    let pi: f64 = 3.14;
    println!("Value of pi: {}", pi);

    //Bool
    let is_snowing: bool = true;
    println!("Is it snowing? {}", is_snowing);

    //Char
    let letter: char = 'a';
    println!("First letter of the alphabet is: {}", letter);

    //Array
    let numbers: [i32; 5] = [1,2,3,4,5];
    println!("Number array: {:?}", numbers);
    //let mix = [1,2,"Apple", true]; // doesnt work due to mixed datatypes
    let fruits: [&str; 3] = ["Apple", "Banana", "Orange"];
    println!("Fruits array: {:?}", fruits);
    println!("Fruits array: {:?}", fruits[0]);

    //Tuple
    let homan = ("Alice", 30, false);
    let human: (String, i32, bool) =("Bob".to_string(), 30, false);
    println!("Homan tuple: {:?}", homan);
    println!("Human tuple: {:?}", human);

    let mixed_tuple =("Kratos", 23, true, [1,2,3,4,5]);
    println!("Mixed tuple: {:?}", mixed_tuple);

    //Slice
    let number_slices:&[i32] = &[1,2,3,4,5];
    println!("Slices of numbers: {:?}", number_slices);

    let animal_slices:&[&str] = &["Lion", "Monkey", "Giraph"];
    println!("Slices of animals: {:?}", animal_slices);

    let book_slices:&[&String] = &[&"IT".to_string(), &"Bible".to_string(), &"HP".to_string()];
    println!("Slices of books: {:?}", book_slices);

    //Strings vs strin glices (&str)
    //Strings [growable, mutable, owned string type]
    let mut stone_cold: String = String::from("Hell, ");
    println!("Stone cold says: {}", stone_cold);
    stone_cold.push_str("Yeah!");
    println!("Stone cold says: {}", stone_cold);

    //String slices
    let string: String = String::from("Hello, world");
    let slice: &str = &string;
    println!("Slice value: {}", slice);
}