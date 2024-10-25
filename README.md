# winelist-generator
Auto generates PDF winelist from cellartracker data

# How-to
1. Install dependencies
Ubuntu/Debian:
```
    sudo apt-get install pandoc texlive-full -y
```

2. Define environment variables
For instance, add the following to ~/.bashrc
```
    export CELLARTRACKER_USR="<your_username>"
    export CELLARTRACKER_PW="<your_password>"
```

3. Run
Clone repo and run `cargo run`

4. FAQ
   1. But.. this is already a feature in CellarTracker?
      1. Yes, but it is a paid feature
   2. You could've done this way faster in python!
      1. Yes, but then I wouldn't have learned about rust

5. TODO
   1. Improve error handling

