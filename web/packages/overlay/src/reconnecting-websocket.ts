import { EventMap } from './types';

export class WsMessageEvent<K extends keyof EventMap> extends Event {
  constructor(public messageType: K, public data: EventMap[K]) {
    super(`ws:${messageType}`);
  }
}

export class ReconnectingWebsocket extends EventTarget {
  private ws?: WebSocket;
  private shouldReconnect = false;
  private nextDelay = 1;

  public listen<K extends keyof EventMap>(key: K, fn: (e: WsMessageEvent<K>) => void): this {
    this.addEventListener(`ws:${key}`, fn as EventListener);
    return this;
  }

  public connect(): Promise<void> {
    return new Promise<void>((resolve, reject) => {
      this.shouldReconnect = false;

      const ws = new WebSocket(createWebsocketUrl());
      this.ws = ws;

      // functions to make the first connection a promise
      const clearListeners = () => {
        ws.removeEventListener('open', openListener);
        ws.removeEventListener('error', errorListener);
        clearTimeout(timeoutId);
      };
      const openListener = () => {
        resolve();
        this.shouldReconnect = true;
        clearListeners();
      };
      const errorListener = (e: Event) => {
        reject(e);
        clearListeners();
      };

      // the actual handlers
      ws.addEventListener('open', openListener);
      ws.addEventListener('error', errorListener);

      const closeListener = () => {
        ws.removeEventListener('message', messageListener);
        this.reconnectLater();
      };
      const messageListener = (e: MessageEvent) => {
        if (typeof e.data === 'string') {
          try {
            const json = JSON.parse(e.data);
            if (typeof json === 'object' && json !== null && typeof json.type === 'string') {
              this.handleMessage(json.type, json.content);
            } else {
              console.warn('invalid messgae', json);
            }
          } catch (e) {
            console.warn('invalid json', e);
          }
        }
      };
      ws.addEventListener('close', closeListener, { once: true });
      ws.addEventListener('message', messageListener);

      const timeoutId = setTimeout(() => {
        ws.close();
        reject('?????');
      }, 2000);
    });
  }

  private reconnectLater() {
    setTimeout(() => {
      this.nextDelay = Math.min(this.nextDelay * 2 + 5, 120);
      this.connect().then(() => (this.nextDelay = 1));
    }, this.nextDelay * 1000);
  }

  private handleMessage<K extends keyof EventMap>(type: K, content: EventMap[K]) {
    this.dispatchEvent(new WsMessageEvent(type, content));

    if (type === 'Ping') {
      this.ws?.send(JSON.stringify({ type: 'Pong' }));
    }
  }
}

function createWebsocketUrl() {
  const query = new URLSearchParams(location.hash?.substring(1)).toString();
  const path = `/api/v1/overlay-ws?${query}`;
  if (process.env.APP_BASE) {
    return `${process.env.APP_BASE.includes('localhost') ? 'ws' : 'wss'}://${process.env.APP_BASE}${path}`;
  } else {
    return `wss://imglaze.nerixyz.de${path}`;
  }
}
