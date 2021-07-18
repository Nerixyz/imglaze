import { EventMap } from './types';

export class WsMessageEvent<K extends keyof EventMap> extends Event {
  constructor(public messageType: K, public data: EventMap[K]) {
    super(`ws:${messageType}`);
  }
}

export class ReconnectingWebsocket extends EventTarget {
  private ws?: WebSocket;
  private shouldReconnect = false;

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
        if (this.shouldReconnect) {
          // don't reconnect if an error occurred
          this.connect().catch(console.error);
        }
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
    });
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
