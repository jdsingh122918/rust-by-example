#[allow(unreachable_code)]
#[allow(unused)]
fn main() {
    println!("Hello, world to the flow control example!");

    let n = 5;

    if n < 0 {
        println!("{} is negative", n);
    } else if n > 0 {
        println!("{} is positive", n);
    } else {
        println!("{} is zero", n);
    }

    let big_n = if n < 10 && n > -10 {
        print!(", and is a smaller number, increase ten fold");
        n * 10
    } else {
        print!(", and is a big number, halve the number");
        n / 2
    };

    println!("{} -> {}", n, big_n);

    // loop is used for infinity loop

    let mut count = 0u32;

    println!("Let's count till infinity");

    loop {
        count += 1;

        if 3.eq(&count) {
            println!("We have reached 3!!!");
            continue;
        }

        if count == 5 {
            println!("That's enough");
            break;
        }
    }

    'outer: loop {
        println!("Entered the outer loop");

        'inner: loop {
            println!("Entered the inner loop");
            break 'outer;
        }

        println!("Unreachable Code here!!!")
    }

    let mut second_counter = 0;

    let result = loop {
        second_counter += 1;
        if second_counter == 10 {
            break second_counter * 2;
        }
    };
    assert_eq!(20, result);

    let mut n = 1;

    while n < 101 {
        if n % 15 == 0 {
            println!("fizzbuzz");
        } else if n % 3 == 0 {
            println!("fizz")
        } else if n % 5 == 0 {
            println!("buzz");
        } else {
            println!("{}", n);
        }
        n += 1;
    }

    let mut names = vec!["Frank", "Misha", "JD"];
    for name in names.iter_mut() {
        match *name {
            "Frank" => println!("Hello {}", name),
            _ => println!("Hello {}", name),
        }
    }

    println!("names: {:?}", names);

    let reference = &4;

    match *reference {
        val => println!("Value is {:?}", val),
    }

    println!("value is {}", reference);

    let ref _is_reference = 4;

    match *_is_reference {
        val => println!("{}", val),
    }

    let value = 6;

    match value {
        ref val => println!("{:?}", val),
    }

    let mut mut_value = 10;

    match mut_value {
        ref mut val => {
            println!("Got a mutable reference!! Needs to dereference before mutating");
            *val += 10;
            println!("We added 10. `mut_value`: {:?}", val);
        }
    }

    let pair = (2, -2);
    match pair {
        (x, y) if x == y => println!("We have twins!!"),
        _ => println!("Not twins"),
    }

    match age() {
        n @ 1..=12 => println!("Not a teenager!!"),
        n @ 13..=19 => println!("A Teenager"),
        _ => (),
    }

    let a = Foo::Bar;

    if let Foo::Bar = a.into() {
        println!("a is foo bar");
    }
}

fn age() -> u32 {
    110
}

enum Foo {
    Bar,
}
