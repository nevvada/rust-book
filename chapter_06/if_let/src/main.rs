fn main() {
    let config_max = Some(3u8);
    match config_max {
        Some(max) => println!("The maximum is configured to be {}", max),
        _ => (),
    }

    if let Some(max) = config_max {
        println!("The max is configured to be {}", max);
    }

    enum UsState {
        Alabama,
    }

    enum Coin {
        Quarter(UsState),
    }

    let mut count = 0;
    // if let Coin::Quarter(state) = coin {
    //     println!("State quarter from {:?}", state);
    // } else {
    //     count += 1;
    // }
}
