'use client';

import { useCallback, useEffect, useRef, useState } from 'react';
import { api, ApiError } from '@/lib/api';
import type { PotEquityProblem } from '@/lib/types';
import { Table } from '@/components/Table';
import { Control, ModeShell, Segmented } from '@/components/ModeShell';
import { SessionFrame } from '@/components/SessionFrame';
import { Dealing } from '@/components/Dealing';
import { DEFAULT_DURATION, useSession, VERDICT_MS, type Duration } from '@/lib/session';
import { useProblemQueue } from '@/lib/useProblemQueue';
import { signed } from '@/lib/format';
import styles from './page.module.css';

const STREETS = ['pre-flop', 'flop', 'turn'] as const;
type Street = (typeof STREETS)[number];

export default function PotOddsPage() {
  const [error, setError] = useState<string | null>(null);
  const [busy, setBusy] = useState(false);

  const [verdict, setVerdict] = useState<{ chose: boolean; correct: boolean; expected: boolean } | null>(
    null
  );
  const hold = useRef<number | undefined>(undefined);
  const answered = useRef<string | null>(null);
  useEffect(() => () => window.clearTimeout(hold.current), []);

  const [duration, setDuration] = useState<Duration>(DEFAULT_DURATION);
  const [numPlayers, setNumPlayers] = useState(2);
  const [streets, setStreets] = useState<Street[]>([...STREETS]);

  const session = useSession(duration);
  const { state, record } = session;

  const fetchOne = useCallback(
    () =>
      api.potEquity.generate({
        numPlayers,
        streets: streets.join(',')
      }),
    [numPlayers, streets]
  );

  const { problem, advance } = useProblemQueue({
    fetch: fetchOne,
    active: state === 'playing',
    onError: setError,
    onAdvance: () => {
      answered.current = null;
      setVerdict(null);
      setBusy(false);
    }
  });

  useEffect(() => {
    if (state === 'playing') return;
    window.clearTimeout(hold.current);
    setVerdict(null);
  }, [state]);

  const decide = useCallback(
    async (call: boolean) => {
      if (!problem || answered.current === problem.problem_id) return;
      answered.current = problem.problem_id;
      setBusy(true);
      try {
        const result = await api.potEquity.check(problem.problem_id, call);
        const equity = result.playerEquity * 100;
        const needed = result.potOdds * 100;
        const chose = call ? 'Call' : 'Fold';
        const right = result.expectedDecision ? 'Call' : 'Fold';
        record({
          correct: result.userGuessIsCorrect,
          hands: problem.hands,
          handLabels: ['You', 'Opponents'],
          board: problem.board,
          yours: result.userGuessIsCorrect ? chose : `${chose} (${right})`,
          detail: [
            { label: 'Pot', value: (problem.pot_size + problem.bet_to_call).toLocaleString() },
            { label: 'To call', value: problem.bet_to_call.toLocaleString() },
            { label: 'Equity', value: `${equity.toFixed(1)}%` },
            { label: 'Need', value: `${needed.toFixed(1)}%` },
            { label: 'Edge', value: signed(equity - needed, 1) }
          ]
        });
        setVerdict({
          chose: call,
          correct: result.userGuessIsCorrect,
          expected: result.expectedDecision
        });
        hold.current = window.setTimeout(() => void advance(), VERDICT_MS);
      } catch (e) {
        if (e instanceof ApiError && e.isGone) {
          void advance();
          return;
        }
        setError(e instanceof ApiError ? e.message : String(e));
        answered.current = null;
        setBusy(false);
      }
    },
    [problem, record, advance]
  );

  useEffect(() => {
    function onKey(e: KeyboardEvent) {
      if (e.key === 'c' || e.key === 'C') void decide(true);
      else if (e.key === 'f' || e.key === 'F') void decide(false);
    }
    window.addEventListener('keydown', onKey);
    return () => window.removeEventListener('keydown', onKey);
  }, [decide]);

  return (
    <ModeShell
      title="Pot Odds &amp; EV"
      score={state === 'playing' ? { correct: session.correct, total: session.total } : undefined}
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
        mode="pot-odds"
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
        blurb="You are facing a bet. Call or fold."
      >
        <div className={styles.spot}>
          {problem ? (
            <Table
              hands={problem.hands}
              board={problem.board}
              pot={problem.pot_size + problem.bet_to_call}
              toCall={problem.bet_to_call}
            />
          ) : (
            <Dealing />
          )}

          <div className={styles.actions}>
            <button
              className={`${styles.call} ${tone(verdict, true)}`}
              onClick={() => void decide(true)}
              disabled={busy}
            >
              Call <span className={styles.kbd}>C</span>
            </button>
            <button
              className={`${styles.fold} ${tone(verdict, false)}`}
              onClick={() => void decide(false)}
              disabled={busy}
            >
              Fold <span className={styles.kbd}>F</span>
            </button>
          </div>
        </div>
      </SessionFrame>
    </ModeShell>
  );
}

function tone(
  verdict: { chose: boolean; correct: boolean; expected: boolean } | null,
  isCall: boolean
) {
  if (!verdict) return '';
  if (verdict.chose === isCall) return verdict.correct ? styles.right : styles.wrong;
  return verdict.expected === isCall ? styles.right : '';
}
