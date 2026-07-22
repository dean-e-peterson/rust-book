fn main() {
    println!("Hello, world!");

    print_labeled_measurement(5, 'h');
    curly_brace_scope_as_expression();
    println!("{}", five());
    println!("{}", optionally_return_early(true));
    println!("{}", optionally_return_early(false));
}

fn print_labeled_measurement(value: i32, unit_label: char) {
    println!("The measurement is: {value}{unit_label}");
}

// Statements end in semicolons and do not return a value.
// Expressions do not end in semicolons an do return a value.
fn curly_brace_scope_as_expression()
{
    let y = {
        let x = 3;
        x + 1
    };
    println!("Here y is {y}.");
}

fn five() -> i32 {
    5
}

fn optionally_return_early(early: bool) -> isize
{
    if early {
        return 1;
    }
    
    0
}

