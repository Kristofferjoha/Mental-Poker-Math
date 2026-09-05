'use client';

import { useCallback, useEffect, useRef, useState } from 'react';
import { ApiError } from './api';

export function useProblemQueue<T>(options: {
  fetch: () => Promise<T>;
  active: boolean;
  onError: (message: string) => void;
  onAdvance?: () => void;
}) {
  const [problem, setProblem] = useState<T | null>(null);

  const latest = useRef(options);
  latest.current = options;

  const spare = useRef<Promise<T> | null>(null);

  const queue = useCallback(() => {
    const pending = latest.current.fetch();
    pending.catch(() => {});
    return pending;
  }, []);

  const advance = useCallback(async () => {
    const next = spare.current ?? queue();
    spare.current = queue();
    latest.current.onAdvance?.();
    try {
      setProblem(await next);
    } catch (e) {
      latest.current.onError(e instanceof ApiError ? e.message : String(e));
      spare.current = null;
    }
  }, [queue]);

  useEffect(() => {
    if (!options.active) {
      setProblem(null);
      return;
    }
    spare.current = null;
    void advance();
    // eslint-disable-next-line react-hooks/exhaustive-deps -- only the session starting or stopping deals
  }, [options.active]);

  return { problem, advance };
}
