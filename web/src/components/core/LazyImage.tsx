import React, { FC } from 'react';

interface Props {}

const LazyImage: FC<Props> = ({children}) => {
  return (<React.Suspense fallback={<React.Fragment/>}>
    {children}
  </React.Suspense>);
};

export default LazyImage;
