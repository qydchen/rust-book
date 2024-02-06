enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter(UsState),
}

#[derive(Debug)] // so we can inspect the state in a minute
enum UsState {
    Alabama,
    Alaska,
    // --snip--
}

fn value_in_cents(coin: Coin) -> u8 {
    match coin {
        Coin::Penny => 1,
        Coin::Nickel => 5,
        Coin::Dime => 10,
        Coin::Quarter(state) => {
            println!("State quarter from {:?}!", state);
            25
        }
    }
}

// matches are exhaustive


fn main() {
    value_in_cents(Coin::Quarter(UsState::Alaska));


    let dice_roll = 7;
    match dice_roll {
        3 => {
            println!("Adding fancy hat");
            add_fancy_hat()
        }
        7 => {
            println!("Remove fancy hat");
            remove_fancy_hat();
        }
        _ => {
            println!("Reroll");
            reroll();
        }
    }

}

fn add_fancy_hat() {}
fn remove_fancy_hat() {}
fn reroll() {}