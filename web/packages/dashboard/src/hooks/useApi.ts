import { useContext } from 'react';
import { ApiContext } from '../context/Api';
import { ApiClient } from '../api/ApiClient';

export function useApi() {
  return useContext(ApiContext) as ApiClient;
}
