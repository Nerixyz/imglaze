import React, { FunctionComponent, useCallback, useMemo, useRef, useState } from 'react';
import { Overlay as OverlayType } from '../api/types';
import CButton from './core/CButton';
import LazyImage from './core/LazyImage';
const EyeOpenIcon = React.lazy(() => import('./icons/EyeOpenIcon'));
const EyeClosedIcon = React.lazy(() => import('./icons/EyeClosedIcon'));
const CopyIcon = React.lazy(() => import('./icons/CopyIcon'));
const ReloadIcon = React.lazy(() => import('./icons/ReloadIcon'));
const DeleteIcon = React.lazy(() => import('./icons/DeleteIcon'));

interface Props {
  overlay: OverlayType;
  rerollSecret: () => void;
  deleteOverlay: () => void;
}

const Overlay: FunctionComponent<Props> = ({ deleteOverlay, rerollSecret, overlay }) => {
  const overlayUrl = useMemo(
    () =>
      `https://imglaze.nerixyz.de/overlay#${new URLSearchParams({
        id: overlay.id.toString(),
        secret: overlay.secret,
      }).toString()}`,
    [overlay],
  );

  const [secretShown, setSecretShown] = useState(false);

  const toggleSecret = useCallback(
    (e: React.SyntheticEvent) => {
      e.preventDefault();
      setSecretShown(state => !state);
    },
    [setSecretShown],
  );

  const tbRef = useRef<null | HTMLDivElement>(null);

  const copyUrl = useCallback(
    (e: React.SyntheticEvent) => {
      e.preventDefault();
      navigator.clipboard.writeText(overlayUrl).then(() => {
        tbRef.current?.animate(
          {
            transform: ['scale(1)', 'scale(1.05)', 'scale(1)'],
            filter: ['drop-shadow(0 0 0px #fff)', 'drop-shadow(0 0 10px #fff)', 'drop-shadow(10px 0 0px #fff)'],
          },
          { duration: 200 },
        );
      });
    },
    [overlayUrl, tbRef],
  );

  return (
    <div className="bg-gradient-to-r from-pink-400 to-purple-500 rounded-2xl p-4 selection:bg-pink-800 selection:text-white">
      <div>
        <h2 className="font-serif font-bold text-2xl mb-2">
          <span className="opacity-50 select-none">Overlay for</span>{' '}
          <span className="select-all">{overlay.for_user}</span>
        </h2>
        <div className="flex gap-2 w-full items-center">
          <div
            className="flex gap-2 flex-grow px-2 py-1 rounded-lg bg-opacity-30 bg-white hover:bg-gray-200 hover:bg-opacity-30 font-mono text-sm"
            ref={tbRef}
          >
            <input
              className="bg-opacity-0 bg-white flex-grow outline-none"
              readOnly={true}
              type={secretShown ? 'text' : 'password'}
              value={overlayUrl}
              onFocus={e =>
                e.target.select()
              }
            />
            {secretShown ? (
              <LazyImage children={<EyeOpenIcon className="cursor-pointer" onClick={toggleSecret} />}/>
            ) : (
              <LazyImage children={<EyeClosedIcon className="cursor-pointer" onClick={toggleSecret} />}/>
            )}
          </div>
          <LazyImage children={<CopyIcon className="cursor-pointer" onClick={copyUrl} />}/>
        </div>
      </div>
      <div className="flex items-end justify-end mt-3">
        <CButton onClick={rerollSecret}><LazyImage children={<ReloadIcon/>}/> Reset</CButton>
        <CButton onClick={deleteOverlay}><LazyImage children={<DeleteIcon/>}/> Delete</CButton>
      </div>
    </div>
  );
};

export default Overlay;
