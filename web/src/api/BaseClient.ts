// eslint-disable-next-line @typescript-eslint/no-explicit-any
type AnyObject = Record<string, any>;

export class BaseClient {
  constructor(private authToken: string, private clearToken: () => void) {
  }

  async logout(): Promise<void> {
    await this.delete('auth').catch(console.error);
    this.authToken = '';
    this.clearToken();
    localStorage.removeItem('authToken');
  }

  protected get<T>(...segments: string[]): Promise<T> {
    return this.baseRequest(segments.join('/'), {});
  }

  protected put<T>(data: AnyObject | undefined, ...segments: string[]): Promise<T> {
    return this.baseRequest(segments.join('/'), {
      method: 'PUT',
      body: data && JSON.stringify(data),
      headers: { 'Content-Type': 'application/json' },
    });
  }

  protected patch<T>(data: AnyObject, ...segments: string[]): Promise<T> {
    return this.baseRequest(segments.join('/'), {
      method: 'PATCH',
      body: JSON.stringify(data),
      headers: { 'Content-Type': 'application/json' },
    });
  }

  protected async delete(...segments: string[]): Promise<void> {
    await this.baseRequest(segments.join('/'), { method: 'DELETE' });
  }

  private async baseRequest<T>(url: string, opts: RequestInit): Promise<T> {
    const response = await fetch(makeApiUrl(url), {
      ...opts,
      headers: {
        ...opts.headers,
        Authorization: this.authToken ? `Bearer ${this.authToken}` : '',
      },
    });
    if (response.headers.get('content-type')?.startsWith('application/json')) {
      const json = await response.json();

      if (isOk(response.status)) return json;

      if (response.status === 401) this.logout().catch(console.error);
      throw new Error(json.error ?? 'An error occurred.');
    } else {
      const text = await response.text();

      if (isOk(response.status)) return text as unknown as T;

      if (response.status === 401) this.logout().catch(console.error);
      throw new Error(text ?? 'An error occurred.');
    }
  }
}

function makeApiUrl(path: string) {
  return `${process.env.NODE_ENV === 'development' ? process.env.API_BASE_URL ?? '' : ''}/api/v1/${path}`;
}

function isOk(status: number) {
  return status >= 200 && status < 300;
}
