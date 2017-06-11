# osxnotifysounds

I thought it'd be nice to have custom notification sounds for any application.

## Running
Currently the program expects to find the config.json file inside `~/.config/osxnotifysounds/config.json`

Inside the config, your primary concerns are:

- app_id value
- look_for values
- sound value

### App ID

In order to find the app_id for an application you're interested in monitoring, use the '-a' cli argument, e.g.,

```
./osxnotifysounds -a slack
Matched application: com.tinyspeck.slackmacgap -- app_id: 25
```

You can now add a new entry to the config.json file for this app and then define the look_for and sound values as desired.

### Look For
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



