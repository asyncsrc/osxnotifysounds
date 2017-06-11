# osxnotifysounds

I thought it'd be nice to have custom notification sounds for any application.

## Running
Currently the program expects to find the config.json file inside `~/.config/osxnotifysounds/config.json`

Inside the config, your primary concerns are:

- app_id value

In order to find the app_id for an application you're interested in monitoring, use the '-a' cli argument, e.g.,

```
./osxnotifysounds -a slack
Matched application: com.tinyspeck.slackmacgap -- app_id: 25
```

- look_for values

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

- sound value

the **app_id** needs to match the app_id for the application you'd like to monitor in the notification center

the **look_for** values are what trigger a custom sound if they're found in the particular alert

the **sound** value should point to a playable .aiff file or any other filetype afplay supports

Now you can create a new config entry for this app_id and have custom sounds for it.
