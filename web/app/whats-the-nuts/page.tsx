'use client';

import { useCallback, useEffect, useRef, useState } from 'react';
import { api, ApiError } from '@/lib/api';
import type { Difficulty, NutsAnswer, NutsProblem } from '@/lib/types';
import { CardRow } from '@/components/Card';
import { Control, ModeShell } from '@/components/ModeShell';
import { SessionFrame } from '@/components/SessionFrame';
import { Dealing } from '@/components/Dealing';
import { DEFAULT_DURATION, useSession, VERDICT_MS, type Duration } from '@/lib/session';
import { useProblemQueue } from '@/lib/useProblemQueue';
import styles from './page.module.css';

const DIFFICULTIES: Difficulty[] = ['easy', 'medium', 'hard'];

export default function WhatsTheNutsPage() {
  const [answer, setAnswer] = useState<NutsAnswer | null>(null);
  const [picked, setPicked] = useState<number | null>(null);
  const [error, setError] = useState<string | null>(null);
  const [busy, setBusy] = useState(false);

  const [difficulty, setDifficulty] = useState<Difficulty>('medium');
  const [duration, setDuration] = useState<Duration>(DEFAULT_DURATION);

  const session = useSession(duration);
  const { state, record } = session;

  const hold = useRef<number | undefined>(undefined);
  const answered = useRef<string | null>(null);
  useEffect(() => () => window.clearTimeout(hold.current), []);

  const { problem, advance } = useProblemQueue({
    fetch: useCallback(() => api.whatsTheNuts.generate({ difficulty }), [difficulty]),
    active: state === 'playing',
    onError: setError,
    onAdvance: () => {
      answered.current = null;
      setAnswer(null);
      setPicked(null);
      setBusy(false);
    }
  });

  useEffect(() => {
    if (state !== 'playing') window.clearTimeout(hold.current);
  }, [state]);

  const submit = useCallback(
    async (index: number) => {
      if (!problem || answered.current === problem.problem_id) return;
      answered.current = problem.problem_id;
      setBusy(true);
      setPicked(index);
      try {
        const result = await api.whatsTheNuts.check(problem.problem_id, index);
        setAnswer(result);
        hold.current = window.setTimeout(() => void advance(), VERDICT_MS);
        record({
          correct: result.correct,
          hands: result.correct
            ? [result.correctHand]
            : [problem.candidates[index], result.correctHand],
          handLabels: result.correct ? ['The nuts'] : ['Your pick', 'The nuts'],
          board: problem.board,
          yours: result.correct ? 'correct' : 'wrong'
        });
      } catch (e) {
        if (e instanceof ApiError && e.isGone) {
          void advance();
          return;
        }
        setError(e instanceof ApiError ? e.message : String(e));
        answered.current = null;
        setPicked(null);
        setBusy(false);
      }
    },
    [problem, record, advance]
  );

  useEffect(() => {
    function onKey(e: KeyboardEvent) {
      const n = Number.parseInt(e.key, 10);
      if (problem && n >= 1 && n <= problem.candidates.length) void submit(n - 1);
    }
    window.addEventListener('keydown', onKey);
    return () => window.removeEventListener('keydown', onKey);
  }, [problem, submit]);

  return (
    <ModeShell
      title="What&rsquo;s the Nuts"
      score={state === 'playing' ? { correct: session.correct, total: session.total } : undefined}
      error={error}
      controls={
        state === 'idle' ? (
        <Control label="Difficulty">
          <select
            value={difficulty}
            onChange={(e) => setDifficulty(e.target.value as Difficulty)}
          >
            {DIFFICULTIES.map((d) => (
              <option key={d} value={d}>
                {d}
              </option>
            ))}
          </select>
        </Control>
        ) : undefined
      }
    >
      <SessionFrame
        mode="whats-the-nuts"
        state={state}
        duration={duration}
        onDurationChange={setDuration}
        secondsLeft={session.secondsLeft}
        correct={session.correct}
        total={session.total}
        history={session.history}
        onStart={session.start}
        onReset={session.reset}
        blurb="One board. Pick the nuts."
      >
      <div className={styles.spot}>
      {problem ? (
        <>
          <section className={styles.boardBlock}>
            <span className="label">Board</span>
            <CardRow cards={problem.board} size="lg" />
          </section>

          <div>
            <span className="label">Which holding makes the best possible hand?</span>
            <div className={styles.candidates}>
              {problem.candidates.map((hand, i) => {
                const isAnswer = answer?.correctIndex === i;
                const isPick = picked === i;
                const classes = [styles.candidate];
                if (answer && isAnswer) classes.push(styles.isAnswer);
                else if (answer && isPick) classes.push(styles.isWrong);
                else if (answer) classes.push(styles.faded);

                return (
                  <button
                    key={i}
                    className={classes.join(' ')}
                    onClick={() => void submit(i)}
                    disabled={answer !== null || busy}
                  >
                    <span className={`num ${styles.key}`}>{i + 1}</span>
                    <CardRow cards={hand} size="md" />
                  </button>
                );
              })}
            </div>
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
