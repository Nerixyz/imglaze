import { BaseClient } from './BaseClient';
import { Overlay } from './types';

export class ApiClient extends BaseClient {
  async createOverlay(forUser: string): Promise<Overlay> {
    return await this.put({ for_user: forUser }, 'overlays');
  }

  async rerollSecret(id: number): Promise<Overlay> {
    return await this.patch({}, 'overlays', id.toString());
  }

  async deleteOverlay(id: number): Promise<void> {
    await this.delete('overlays', id.toString());
  }

  async getOverlays(): Promise<Overlay[]> {
    return await this.get('overlays');
  }
}
