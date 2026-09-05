'use client';

import { useCallback, useEffect, useRef, useState } from 'react';
import type { Card } from './types';

export type SessionState = 'idle' | 'playing' | 'finished';

export const DURATIONS = [60, 90, 120] as const;
export type Duration = (typeof DURATIONS)[number];

export const VERDICT_MS = 200;

export const DEFAULT_DURATION: Duration = 120;

export function durationLabel(seconds: Duration) {
  return `${seconds}s`;
}

export interface ReviewEntry {
  correct: boolean;
  points?: number;
  outOf?: number;
  hands: Card[][];
  board: Card[];
  yours?: string;
  truth?: string;
  detail?: Array<{ label: string; value: string }>;
  handLabels?: string[];

  handNotes?: string[];
  handAsides?: string[];
  handMarks?: boolean[];
}

export function useSession(duration: Duration) {
  const [state, setState] = useState<SessionState>('idle');
  const [secondsLeft, setSecondsLeft] = useState<number>(duration);
  const [history, setHistory] = useState<ReviewEntry[]>([]);
  const endsAt = useRef(0);

  const start = useCallback(() => {
    setHistory([]);
    setSecondsLeft(duration);
    endsAt.current = performance.now() + duration * 1000;
    setState('playing');
  }, [duration]);

  const finish = useCallback(() => setState('finished'), []);

  const reset = useCallback(() => {
    setHistory([]);
    setSecondsLeft(duration);
    setState('idle');
  }, [duration]);

  const record = useCallback((entry: ReviewEntry) => {
    setHistory((h) => [...h, entry]);
  }, []);

  useEffect(() => {
    if (state !== 'playing') return;
    let frame = 0;
    const tick = () => {
      const left = Math.max(0, (endsAt.current - performance.now()) / 1000);
      setSecondsLeft(Math.ceil(left));
      if (left > 0) frame = requestAnimationFrame(tick);
      else finish();
    };
    frame = requestAnimationFrame(tick);
    return () => cancelAnimationFrame(frame);
  }, [state, finish]);

  useEffect(() => {
    if (state === 'idle') setSecondsLeft(duration);
  }, [duration, state]);

  const correct = history.reduce((n, h) => n + (h.points ?? (h.correct ? 1 : 0)), 0);
  const total = history.reduce((n, h) => n + (h.outOf ?? 1), 0);

  return {
    state,
    secondsLeft,
    history,
    correct,
    total,
    start,
    reset,
    record
  };
}
