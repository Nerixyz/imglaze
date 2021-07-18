import React, { FunctionComponent } from 'react';
import CButton from '../components/core/CButton';
import LazyImage from '../components/core/LazyImage';
const TwitchIcon = React.lazy(() => import('../components/icons/TwitchIcon'));

const Login: FunctionComponent = () => {
  return (<>
    <main className="flex flex-col gap-10 mt-10">
      <h1 className="font-serif font-bold text-white text-5xl">
        Simple Live Overlays
      </h1>
      <CButton href="/api/v1/auth/twitch-auth">
        <LazyImage children={<TwitchIcon/>}/> Authenticate with twitch
      </CButton>
    </main>
  </>);
};

export default Login;
