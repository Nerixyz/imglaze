import { createContext } from 'react';
import { ApiClient } from '../api/ApiClient';

export const ApiContext = createContext<null | ApiClient>(null);

export const ApiProvider = ApiContext.Provider;
