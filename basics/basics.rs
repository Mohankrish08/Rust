fn main()
{
    variable_const();
    println!("----------");
    dtypes();
    println!("----------");
    type_casting();
}

fn variable_const()
{
    // let is a keyword is used to store the value
    let x = 5;
    println!("The value of x is {}",x);

    let age = 19;
    println!("The age is {}", age);

    // for changing the value

    let mut y = 10;
    println!("Before changing  y = {}", y);
    y = 12;
    println!("After changing y = {}",y);

    // const keyword is used to  declare constant variables.

    const PI: f32 = 3.12;
    println!("PI is {}", PI);
}

fn dtypes()
{
    // integer 
    let number: i32 = 10;
    // float 
    let float_n: f32 = 20.5;
    //char
    let chars: char = 'A';
    // boolean
    let boolen: bool = true;
    // special chars
    let special_chars: char = '$';

    println!("{4}, {3}, {2}, {1}, {0}", special_chars, boolen, chars, float_n, number);
}

fn type_casting()
{
    // converting float to int
    let int = 10;
    let float = int as f32;
    println!("{}",float);

    // converting char to int
    let chars = 'A';
    let int_n = chars as u8;
    println!("{}", int_n);

    // bool to int
    let bool1 = true;
    let output = bool1 as i32;
    println!("{}", output);

}