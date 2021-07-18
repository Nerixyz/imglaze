import React, { useState } from 'react';
import { ApiClient } from './api/ApiClient';
import { ApiProvider } from './context/Api';
import Login from './views/Login'
import Dashboard from './views/Dashboard';

export function App() {
  const [token, setToken] = useState(getToken());
  return <div className="bg-gray-900 h-screen w-screen flex justify-center pt-20 overflow-y-auto">
    {(token ? <ApiProvider value={ new ApiClient(token, () => setToken(undefined)) }>
        <Dashboard/>
      </ApiProvider> : <Login/>)}
  </div>
}

function getToken() {
  let cookie: string | undefined | null = localStorage.getItem('imglazeAuthToken');
  if (cookie) return cookie;

  cookie = document.cookie.match(/auth_token=([^;]+)/)?.[1];
  if (!cookie) return;

  localStorage.setItem('imglazeAuthToken', cookie);
  document.cookie = 'auth_token=;expires=0;SameSite=None; Secure';

  return cookie;
}
