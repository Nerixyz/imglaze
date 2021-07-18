import { useCallback, useState } from 'react';

interface PromiseOptions {
  openWhileLoading?: boolean;
  rethrow?: boolean;
}

export function useDialog() {
  const [content, setContent] = useState({ title: 'Dialog', detail: 'uhm' });
  const [open, setOpen] = useState(false);
  const [loading, setLoading] = useState(false);

  const promiseCallback = useCallback(
    <T>(
      promise: Promise<T>,
      { openWhileLoading, rethrow }: PromiseOptions = { openWhileLoading: true, rethrow: false },
    ): Promise<T | void> => {
      openWhileLoading ??= true;
      rethrow ??= false;

      setLoading(true);
      setContent({ title: 'Loading', detail: '' });

      if (openWhileLoading) setOpen(true);

      return promise
        .then(res => {
          setContent({ title: 'Done', detail: '' });
          setOpen(false);
          return res;
        })
        .catch(e => {
          setContent({ title: 'Error', detail: extractMessage(e) });
          if (!openWhileLoading) setOpen(true);
          if (rethrow) throw e;
        })
        .finally(() => setLoading(false));
    },
    [setContent, setOpen, setLoading],
  );
  return {
    promise: promiseCallback,
    async: useCallback(
      <T>(
        fn: () => Promise<T>,
        { openWhileLoading, rethrow }: PromiseOptions = { openWhileLoading: true, rethrow: false },
      ): Promise<T | void> => {
        return promiseCallback(fn(), { openWhileLoading, rethrow });
      },
      [promiseCallback],
    ),
    data: {
      content,
      isOpen: open,
      loading,
      close: useCallback(() => setOpen(false), [setOpen]),
    },
  };
}

function extractMessage(e: unknown) {
  if (e instanceof Error) {
    return e.message;
  } else {
    return String(e);
  }
}
