'use client';

import { useCallback, useEffect, useRef, useState } from 'react';
import { api, ApiError } from '@/lib/api';
import { CardRow } from '@/components/Card';
import { Control, ModeShell, Segmented } from '@/components/ModeShell';
import { SessionFrame } from '@/components/SessionFrame';
import { Dealing } from '@/components/Dealing';
import { DEFAULT_DURATION, useSession, VERDICT_MS, type Duration } from '@/lib/session';
import { useProblemQueue } from '@/lib/useProblemQueue';
import styles from './page.module.css';

const STREETS = ['pre-flop', 'flop', 'turn'] as const;
type Street = (typeof STREETS)[number];

const ORDINALS = ['1st', '2nd', '3rd', '4th', '5th', '6th'];

export default function KingOfTheHillPage() {
  const [error, setError] = useState<string | null>(null);
  const [busy, setBusy] = useState(false);

  const [ranking, setRanking] = useState<number[]>([]);

  const [verdict, setVerdict] = useState<boolean[] | null>(null);
  const hold = useRef<number | undefined>(undefined);
  const answered = useRef<string | null>(null);
  useEffect(() => () => window.clearTimeout(hold.current), []);

  const [duration, setDuration] = useState<Duration>(DEFAULT_DURATION);
  const [numHands, setNumHands] = useState(4);
  const [streets, setStreets] = useState<Street[]>([...STREETS]);

  const session = useSession(duration);
  const { state, record } = session;

  const fetchOne = useCallback(
    () =>
      api.kingOfTheHill.generate({
        numHands,
        streets: streets.join(',')
      }),
    [numHands, streets]
  );

  const { problem, advance } = useProblemQueue({
    fetch: fetchOne,
    active: state === 'playing',
    onError: setError,
    onAdvance: () => {
      answered.current = null;
      setRanking([]);
      setVerdict(null);
      setBusy(false);
    }
  });

  useEffect(() => {
    if (state === 'playing') return;
    window.clearTimeout(hold.current);
    setVerdict(null);
  }, [state]);

  const place = useCallback((index: number) => {
    setRanking((current) =>
      current.includes(index) ? current.filter((i) => i !== index) : [...current, index]
    );
  }, []);

  const submit = useCallback(async () => {
    if (!problem || ranking.length !== problem.hands.length) return;
    if (answered.current === problem.problem_id) return;
    answered.current = problem.problem_id;
    setBusy(true);
    try {
      const result = await api.kingOfTheHill.check(problem.problem_id, ranking);

      const placed = problem.hands.map((_, i) => placedWell(i, ranking, result.equities));

      record({
        correct: result.perfect,
        points: placed.filter(Boolean).length,
        outOf: problem.hands.length,
        hands: result.correctOrder.map((i) => problem.hands[i]),
        board: problem.board,
        handNotes: result.correctOrder.map((i) => `${(result.equities[i] * 100).toFixed(1)}%`),
        handAsides: result.correctOrder.map((i) => `you ${ORDINALS[ranking.indexOf(i)]}`),
        handMarks: result.correctOrder.map((i) => placed[i])
      });
      setVerdict(placed);
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
  }, [problem, ranking, record, advance]);

  useEffect(() => {
    function onKey(e: KeyboardEvent) {
      if (e.key === 'Backspace') {
        e.preventDefault();
        setRanking((current) => current.slice(0, -1));
        return;
      }
      if (e.key === 'Enter') {
        void submit();
        return;
      }
      const n = Number.parseInt(e.key, 10);
      if (problem && n >= 1 && n <= problem.hands.length) place(n - 1);
    }
    window.addEventListener('keydown', onKey);
    return () => window.removeEventListener('keydown', onKey);
  }, [problem, place, submit]);

  const complete = problem !== null && ranking.length === problem.hands.length;

  return (
    <ModeShell
      title="King of the Hill"
      score={state === 'playing' ? { correct: session.correct, total: session.total } : undefined}
      error={error}
      controls={
        state === 'idle' ? (
        <>
          <Control label="Hands">
            <select value={numHands} onChange={(e) => setNumHands(Number(e.target.value))}>
              {[4, 5, 6].map((n) => (
                <option key={n} value={n}>
                  {n}
                </option>
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
        mode="king-of-the-hill"
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
        blurb="Several hands, one board. Rank them strongest to weakest."
      >
      <div className={styles.spot}>
      {problem ? (
        <>
          <section className={styles.boardBlock}>
            <span className="label">{problem.board.length === 0 ? 'Pre-flop' : 'Board'}</span>
            <CardRow cards={problem.board} slots={5} size="md" dimmed={problem.board.length === 0} />
          </section>

          <span className="label">
            Click strongest first &mdash; {ranking.length} of {problem.hands.length} placed
          </span>
          <div className={styles.pool}>
            {problem.hands.map((hand, i) => {
              const slot = ranking.indexOf(i);
              return (
                <button
                  key={i}
                  className={`${styles.hand} ${
                    verdict ? (verdict[i] ? styles.right : styles.wrong) : slot >= 0 ? styles.placed : ''
                  }`}
                  onClick={() => place(i)}
                  disabled={busy}
                >
                  <span className={`num ${styles.key}`}>{i + 1}</span>
                  <CardRow cards={hand} size="sm" />
                  <span className={`num ${styles.slot}`}>{slot >= 0 ? ORDINALS[slot] : ''}</span>
                </button>
              );
            })}
          </div>

          <div className={styles.actions}>
            <button onClick={() => void submit()} disabled={!complete || busy}>
              Submit ranking <span className={styles.kbd}>&crarr;</span>
            </button>
          </div>
        </>
      ) : (
        <Dealing />
      )}
      </div>
      </SessionFrame>
    </ModeShell>
  );
}

function placedWell(hand: number, ranking: number[], equities: number[]) {
  return equities.every((equity, other) => {
    if (other === hand || Math.abs(equity - equities[hand]) < 1e-9) return true;
    return ranking.indexOf(hand) < ranking.indexOf(other) === equities[hand] > equity;
  });
}
