import React, { FunctionComponent, useCallback, useEffect, useState } from 'react';
import { Overlay as OverlayType } from '../api/types';
import AddOverlay from '../components/AddOverlay';
import { useApi } from '../hooks/useApi';
import Overlay from '../components/Overlay';
import { useDialog } from '../hooks/useDialog';
import CDialog from '../components/core/CDialog';

const Dashboard: FunctionComponent = () => {
  const api = useApi();
  const dialog = useDialog();
  const [overlays, setOverlays] = useState<OverlayType[]>([]);

  const addOverlay = useCallback(
    (forUser: string) =>
      dialog.async(
        async () => {
          const newOverlay = await api.createOverlay(forUser.trim().toLowerCase());
          setOverlays(overlays => [...overlays, newOverlay]);
        },
        { rethrow: true },
      ),
    [setOverlays, api, dialog.async],
  );

  const removeOverlay = useCallback(
    (id: number) =>
      dialog.async(async () => {
        await api.deleteOverlay(id);
        setOverlays(overlays => overlays.filter(o => o.id !== id));
      }),
    [setOverlays, api, dialog.async],
  );

  const rerollSecret = useCallback(
    (id: number) => {
      dialog.async(async () => {
        const updated = await api.rerollSecret(id);
        setOverlays(overlays => overlays.map(o => (o.id === updated.id ? updated : o)));
      });
    },
    [setOverlays, api, dialog.async],
  );

  useEffect(() => {
    dialog.promise(
      api.getOverlays().then(overlays => setOverlays(overlays)),
      { openWhileLoading: false },
    );
  }, [setOverlays, api, dialog.promise]);

  return (
    <div className="flex items-center flex-col md:max-w-2xl w-full mx-10">
      <ul className="flex w-full flex-col gap-4">
        {overlays.map(overlay => (
          <Overlay
            key={overlay.id}
            overlay={overlay}
            rerollSecret={() => rerollSecret(overlay.id)}
            deleteOverlay={() => removeOverlay(overlay.id)}
          />
        ))}
      </ul>
      <AddOverlay addOverlay={addOverlay} />
      <CDialog {...dialog.data} />
    </div>
  );
};

export default Dashboard;
