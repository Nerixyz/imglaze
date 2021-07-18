import cntl from 'cntl';
import React, { FC, useCallback, useMemo, useState } from 'react';
// import './CTextBox.css';

interface Props {
  text: string;
  setText: (text: string) => void;
  label: string;
}

const CTextBox: FC<Props> = ({ text, setText, label }) => {
  const changeCallback = useCallback(
    (e: React.ChangeEvent<HTMLInputElement>) => {
      setText(e.target.value);
    },
    [setText],
  );

  const [focused, setFocused] = useState(false);
  const isOccupied = useMemo(() => focused || !!text, [focused, text]);
  const onFocus = useCallback(() => setFocused(true), [setFocused]);
  const onBlur = useCallback(() => setFocused(false), [setFocused]);

  return (
    <div className="block text-black">
      <div
        className={cntl`
      bg-transparent
      rounded-lg
      bg-opacity-40
      bg-white 
      hover:bg-gray-100 
      hover:bg-opacity-40
      transition-colors
      relative`}
      >
        <span
          className={cntl`
          ${isOccupied ? '-translate-y-3 scale-75' : ''}
          ${focused ? 'text-black' : 'text-gray-600'}
          transform
          transition-transform
          transition-colors
          duration-200
          ease-cubic-out
          absolute
          left-2
          right-auto
          max-w-full
          overflow-hidden overflow-ellipsis
          whitespace-nowrap
          pointer-events-none
          origin-top-left
          top-3`}
        >
          {label}
        </span>
        <input
          type="text"
          value={text}
          onChange={changeCallback}
          className="bg-opacity-0 bg-white w-full h-full px-3 py-2 border-none mt-2 outline-none font-mono"
          onFocus={onFocus}
          onBlur={onBlur}
        />
      </div>
    </div>
  );
};

export default CTextBox;
