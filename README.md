## Random Cat Wallpaper
A simple rust program that changes your wallpaper to a random picture of a cat using [The Cat API](https://thecatapi.com/).<br>
You can find a list of supported desktops [here](https://github.com/reujab/wallpaper.rs?tab=readme-ov-file#wallpaper--).

## Setup
1. Go to https://thecatapi.com/#pricing and create a new API key
1. Replace `live_******` with your api key in [`main.rs:6`](https://github.com/icudev/random-cat-wallpaper/blob/master/src/main.rs#L6)
1. Run the program using
    ```
    cargo run
    ```
    or build it using 
    ```
    cargo build --release
    ```
