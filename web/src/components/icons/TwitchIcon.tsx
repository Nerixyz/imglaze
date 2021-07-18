import React, { FC } from 'react';

const TwitchIcon: FC<React.HTMLAttributes<SVGElement>> = props => {
  return (
    <svg width="20" height="20" viewBox="0 0 24 24" {...props}>
      <path
        fill="currentColor"
        d="M11.64 5.93H13.07V10.21H11.64M15.57 5.93H17V10.21H15.57M7 2L3.43 5.57V18.43H7.71V22L11.29 18.43H14.14L20.57 12V2M19.14 11.29L16.29 14.14H13.43L10.93 16.64V14.14H7.71V3.43H19.14Z"
      />
    </svg>
  );
};

export default TwitchIcon;
