import { Dialog, Transition } from '@headlessui/react';
import React, { FC, Fragment } from 'react';
import cntl from 'cntl';
import CButton from './CButton';

export interface DialogProps {
  isOpen: boolean;
  close: () => void;
  content: { title: string; detail?: string };
  loading: boolean;
}

const CDialog: FC<DialogProps> = ({ content, loading, isOpen, close }) => {
  return (
    <Transition show={isOpen} as={Fragment}>
      <Dialog as="div" onClose={close} className="fixed inset-0 z-10 overflow-y-auto">
        <div className="min-h-screen px-4 text-center">
          <Transition.Child
            as={Fragment}
            enter="ease-out duration-300"
            enterFrom="opacity-0 backdrop-blur-none"
            enterTo="opacity-100 backdrop-blur-md"
            entered="backdrop-blur-md"
            leave="ease-in duration-200"
            leaveFrom="opacity-100 backdrop-blur-md"
            leaveTo="opacity-0 backdrop-blur-none"
          >
            <Dialog.Overlay className="fixed inset-0 bg-black bg-opacity-30 backdrop-filter" />
          </Transition.Child>
          <span className="inline-block h-screen align-middle" aria-hidden="true">
            &#8203;
          </span>
          <Transition.Child
            as={Fragment}
            enter="ease-out duration-300"
            enterFrom="opacity-0 scale-95"
            enterTo="opacity-100 scale-100"
            leave="ease-in duration-200"
            leaveFrom="opacity-100 scale-100"
            leaveTo="opacity-0 scale-95"
          >
            <div
              className={cntl`
            inline-flex
            flex-col
            w-full
            max-w-md
            p-6 my-8
            overflow-hidden 
            text-left
            align-middle 
            transition-all
            transform
            shadow-xl
            bg-gradient-to-r
            from-violet-400
            to-violet-500
            rounded-2xl
            `}
            >
              <Dialog.Title as="h3" className="font-serif font-bold text-2xl">
                {content.title}
              </Dialog.Title>
              <div className="mt-2">
                <p className="">{loading ? 'Loading...' : content.detail ?? '<no detail>'}</p>
              </div>

              <div className="self-end mt-4">
                <CButton onClick={close}>Close</CButton>
              </div>
            </div>
          </Transition.Child>
        </div>
      </Dialog>
    </Transition>
  );
};

export default CDialog;
