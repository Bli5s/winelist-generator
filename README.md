# winelist-generator
Auto generates PDF winelist from cellartracker data

## How-to
1. Install dependencies
   1. Ubuntu/Debian:
   ```
       sudo apt-get install pandoc texlive-full -y
       curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
   ```

2. Define environment variables
   1. For instance, add the following to ~/.bashrc
   ```
       export CELLARTRACKER_USR="<your_username>"
       export CELLARTRACKER_PW="<your_password>"
   ```

3. Run
   1. Clone repo and run `cargo run`

4. Build
   1. `cargo build --release`


## FAQ
   1. But.. this is already a feature in CellarTracker?
      1. Yes, but it is a paid feature
   2. You could've done this way faster in python!
      1. Yes, but then I wouldn't have learned about rust

## TODO
   1. Improve error handling
   2. Templates?

