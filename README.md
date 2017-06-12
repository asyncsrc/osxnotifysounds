# osxnotifysounds
I thought it'd be nice to have custom sounds for any application that pushes events/alerts to the OSX notification center.

## Building and Running
Below steps can be used to build the binary.

```
git clone https://github.com/asyncsrc/osxnotifysounds
cd osxnotifysounds
cargo build --release

mkdir -p ~/.config/osxnotifysounds
cp config.json.sample ~/.config/osxnotifysounds/config.json
```

## Tests
Test coverage is weak at the moment as I'm only validating one field in the json config file so far. But here we are!

```
cargo test

[...]

running 1 test
test tests::tests::app_id_invalid ... ok

test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out
```

## Configuration

Inside the config file at `~/.config/osxnotifysounds/config.json`, your primary concerns are:

- app_id value
- look_for values
- sound value

### Find the App ID for your desired application

In order to find the app_id for an application you're interested in monitoring, use the '-a' cli argument, e.g.,

```
./osxnotifysounds -a slack
Matched application: com.tinyspeck.slackmacgap -- app_id: 25
```

You can now add a new entry to the config.json file for this app and set its **app_id** value to this value.

### Triggering sounds for different strings
The program will do substring matches to confirm whether a value within the look_for list is found in the notification/alert text.  

```
Slack alert: (app_id: 25)

Example alert:  "New message from Joe Bob"

Example look_for value:

"look_for": [
  "Joe Bob"
]
```

In this case, Joe Bob would be found in the alert, so the sound value is triggered.

### Sound
Once a sound is triggered, we'll use `afplay` to attempt to play the sound file.  I've only tested `*.aiff` files, but I'm sure we could make any type of audio file/player work.



