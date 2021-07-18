import React, { FunctionComponent, SyntheticEvent, useState } from 'react';
import CTextBox from './core/CTextBox';
import CButton from './core/CButton';
import LazyImage from './core/LazyImage';
const AddIcon = React.lazy(() => import('./icons/AddIcon'));

interface Props {
  addOverlay: (forUser: string) => Promise<void>;
}

const AddOverlay: FunctionComponent<Props> = ({ addOverlay }) => {
  const [input, setInput] = useState('');
  const onSubmit = (e: SyntheticEvent) => {
    e.preventDefault();
    addOverlay(input)
      .then(() => setInput(''))
      .catch(console.warn);
  };
  return (
    <>
      <form
        onSubmit={onSubmit}
        className="flex flex-col w-full justify-center p-8 my-10 bg-gradient-to-br from-red-600 to-red-800 rounded-2xl select-none"
      >
        <h1 className="font-serif font-bold text-3xl mb-5 ml-1">Add Overlay</h1>
        <CTextBox text={input} setText={setInput} label="Username" />
        <div className="self-end flex mt-1">
          <CButton role="submit" secondColor>
            <LazyImage children={<AddIcon />} /> Add
          </CButton>
        </div>
      </form>
    </>
  );
};

export default AddOverlay;
