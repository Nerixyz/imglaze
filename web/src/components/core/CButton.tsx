import React, { FC, useCallback } from 'react';
import cntl from 'cntl';

interface Props {
  href?: string;
  disabled?: boolean;
  secondColor?: boolean;
}

const CButton = React.forwardRef<
  HTMLButtonElement & HTMLAnchorElement,
  Props & React.HTMLAttributes<HTMLButtonElement & HTMLAnchorElement>
>(({ href, disabled, children, secondColor, ...props }, ref) => {
  const preventClick = useCallback(
    (e: React.MouseEvent) => {
      if (disabled) {
        e.stopPropagation();
      }
    },
    [disabled],
  );

  return (
    <MaybeButton
      {...props}
      children={children}
      onClickCapture={preventClick}
      href={href}
      disabled={disabled}
      className={[
        cntl`
      inline-flex
      justify-center
      items-center
      gap-2
      px-6
      m-1
      h-8
      select-none
      uppercase
      bg-gradient-to-r
      rounded-md
      text-black
      text-sm
      font-bold
      shadow-sm
      disabled:from-gray-400
      disabled:to-gray-500
      disabled:cursor-not-allowed
      disabled:ring-gray-600
      disabled:text-gray-700
      hover:bg-red-dark
      hover:shadow-md
      transition-colors
      transition-shadow
      focus:ring-2
      focus:outline-none
      focus:shadow-md
      `,
        secondColor ? 'from-pink-400 to-pink-500 focus:ring-pink-700' : 'from-red-400 to-red-500 focus:ring-red-700',
      ].join(' ')}
      ref={ref}
    />
  );
});

const MaybeButton = React.forwardRef<
  HTMLButtonElement & HTMLAnchorElement,
  { href?: string; disabled?: boolean } & React.HTMLAttributes<HTMLButtonElement & HTMLAnchorElement>
>((props, ref) => {
  return props.href ? <a {...props} ref={ref} /> : <button {...props} ref={ref} />;
});

export default CButton;
