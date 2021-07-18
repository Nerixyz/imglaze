import React, { FC } from 'react';

const LazyImage: FC = ({ children }) => {
  return <React.Suspense fallback={<React.Fragment />}>{children}</React.Suspense>;
};

export default LazyImage;
