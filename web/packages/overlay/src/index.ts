import { ReconnectingWebsocket } from './reconnecting-websocket';

addEventListener('DOMContentLoaded', () => {
  const imageHost = document.getElementById('image-host');
  if (!imageHost) throw new Error('????');

  let imageEl: HTMLImageElement | null = null;

  new ReconnectingWebsocket()
    .listen('Image', event => {
      if (imageEl) {
        imageEl.remove();
      }
      imageEl = document.createElement('img');
      imageEl.src = event.data;
      imageHost.append(imageEl);
    })
    .connect()
    .catch(console.error);
});
