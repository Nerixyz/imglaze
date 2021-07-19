# Imglaze

This is a simple overlay and server.

# Dashboard

You can find your dashboard at [imglaze.nerixyz.de](https://imglaze.nerixyz.de).
There you can login with your Twitch account.

To create a new overlay for a broadcaster, type their name in the `Username` input.

**You can only add an overlay for a channel you have mod in!**

Copy the overlay url and paste it into a new browser source in OBS.

If you think you may have leaked the url somewhere, you can reset the secret. This makes the old url not usable anymore.

# Features

## Image

Type `::img { link }` in the chat (as a mod). 
This will change the image on screen. 
Images will fit to the _width_ of the browser and adjust their height while preserving the aspect ratio (`width: 100%; height: auto;`).
Images are saved per overlay and will be restored on reload.

These are the currently supported image formats: `png`, `jpg`, `jpeg`, `webp`, `avif`, `jxl`, `bmp` and `svg`.