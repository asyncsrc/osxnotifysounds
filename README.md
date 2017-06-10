# osxnotifysounds

For my own personal interest, I thought it'd be nice to have custom notification sounds for any application.
This initially was specifically for Slack, but I've expanded the config to support any number of different applications that use the OSX notification center for sending alerts.

Currently the program expects to find the config.json file inside `~/.config/osxnotifysounds/config.json`

Inside the config, your primary concerns are:
- app_id value
- look_for values
- sound value

the **app_id** needs to match the app_id for the application you'd like to monitor in the notification center

the **look_for** values are what trigger a custom sound if they're found in the particular alert

the **sound** value should point to a playable .aiff file or any other filetype afplay supports
