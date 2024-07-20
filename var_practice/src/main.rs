fn main() {
    let x = 5;
    println!("The value of x is : {x}");
    let x = x + 5;
    println!("The value of x is : {}", x);
    let mut y = 1;
    println!("The value of y is : {y}");
    y = 7;
    println!("The value of y is : {y}");

    let z = 5;
    let z = z + 1;
    println!("z : {z}");
    
    {
        let z = 3;
        println!("z in () : {z}");
        //중괄호 안에 있기 때문에 외부 z의 영향을 받지 않음
    }
    let z = "Hello, World!";
    println!("z : {z}");

    //이 아래는 상수
    
    const THREE_HOURS_IN_SECONDS:u32 = 60*60*3;
    println!("{THREE_HOURS_IN_SECONDS}");
    //const THREE_HOURS_IN_SECONDS:u32 = 30*30;
    println!("{THREE_HOURS_IN_SECONDS}");
}