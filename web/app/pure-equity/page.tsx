'use client';

import { useCallback, useRef, useState } from 'react';
import { api } from '@/lib/api';
import type { PureEquityProblem } from '@/lib/types';
import { DEFAULT_DURATION, useSession, type Duration } from '@/lib/session';
import { useProblemQueue } from '@/lib/useProblemQueue';
import { Table } from '@/components/Table';
import { Control, ModeShell, Segmented } from '@/components/ModeShell';
import { SessionFrame } from '@/components/SessionFrame';
import { Dealing } from '@/components/Dealing';
import styles from './page.module.css';

const STREETS = ['pre-flop', 'flop', 'turn', 'river'] as const;
type Street = (typeof STREETS)[number];

function clean(text: string) {
  const kept = text.replace(/[^\d.]/g, '');
  const [head, ...rest] = kept.split('.');
  return (rest.length > 0 ? `${head}.${rest.join('')}` : head).slice(0, 5);
}

export default function PureEquityPage() {
  const [guess, setGuess] = useState('');
  const [error, setError] = useState<string | null>(null);

  const [duration, setDuration] = useState<Duration>(DEFAULT_DURATION);
  const [numPlayers, setNumPlayers] = useState(2);
  const [tolerance, setTolerance] = useState(5);
  const [streets, setStreets] = useState<Street[]>([...STREETS]);

  const session = useSession(duration);
  const { state, record } = session;

  const fetchOne = useCallback(
    () =>
      api.pureEquity.generate({
        numPlayers,
        tolerance,
        streets: streets.join(',')
      }),
    [numPlayers, tolerance, streets]
  );

  const { problem, advance } = useProblemQueue({
    fetch: fetchOne,
    active: state === 'playing',
    onError: setError,
    onAdvance: () => setGuess('')
  });

  const answered = useRef<string | null>(null);

  function onGuess(text: string) {
    const next = clean(text);
    setGuess(next);

    if (!problem || next === '' || problem.problem_id === answered.current) return;
    const value = Number.parseFloat(next);
    if (!(value >= problem.lower_bound && value <= problem.upper_bound)) return;

    answered.current = problem.problem_id;
    record({
      correct: true,
      hands: problem.hands,
      handLabels: ['You', 'Opponents'],
      board: problem.board,
      yours: `${value}%`,
      truth: `${problem.player_equity.toFixed(1)}%`
    });
    void api.pureEquity.check(problem.problem_id, value).catch(() => {});
    setGuess('');
    void advance();
  }

  return (
    <ModeShell
      title="Equity Intuition"
      score={
        state === 'playing' ? { correct: session.correct, total: session.total, rate: true } : undefined
      }
      error={error}
      controls={
        state === 'idle' ? (
          <>
            <Control label="Players">
              <select value={numPlayers} onChange={(e) => setNumPlayers(Number(e.target.value))}>
                {[2, 3, 4, 5, 6].map((n) => (
                  <option key={n} value={n}>{n}</option>
                ))}
              </select>
            </Control>
            <Control label="Tolerance">
              <select value={tolerance} onChange={(e) => setTolerance(Number(e.target.value))}>
                {[2, 3, 5, 8].map((t) => (
                  <option key={t} value={t}>&plusmn;{t}%</option>
                ))}
              </select>
            </Control>
            <Segmented
              options={STREETS}
              selected={streets}
              onToggle={(s) =>
                setStreets((cur) => (cur.includes(s) ? cur.filter((x) => x !== s) : [...cur, s]))
              }
            />
          </>
        ) : undefined
      }
    >
      <SessionFrame
        mode="pure-equity"
        state={state}
        duration={duration}
        onDurationChange={setDuration}
        secondsLeft={session.secondsLeft}
        correct={session.correct}
        total={session.total}
        history={session.history}
        onStart={session.start}
        onReset={session.reset}
        ready={streets.length > 0}
        scoring="rate"
        blurb={`Estimate your share of the pot. Get within ±${tolerance}% and the next hand deals.`}
      >
        <div className={styles.spot}>
          {problem ? <Table hands={problem.hands} board={problem.board} /> : <Dealing />}

          <div className={styles.answerArea}>
            <input
              type="text"
              inputMode="decimal"
              autoComplete="off"
              value={guess}
              onChange={(e) => onGuess(e.target.value)}
              placeholder="equity (%)"
              aria-label="Your equity estimate, in percent"
              className={`${styles.input} num`}
              autoFocus
            />
            <p className={styles.tip}>&plusmn;{tolerance}%</p>
          </div>
        </div>
      </SessionFrame>
    </ModeShell>
  );
}
