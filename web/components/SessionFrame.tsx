'use client';

import { useEffect } from 'react';

import { DURATIONS, durationLabel, type Duration, type ReviewEntry, type SessionState } from '@/lib/session';
import { recordSession, type ModeSlug } from '@/lib/stats';
import { CardRow } from './Card';

const ORDINALS = ['1st', '2nd', '3rd', '4th', '5th', '6th'];
import styles from './SessionFrame.module.css';

export function SessionFrame({
  mode,
  state,
  duration,
  onDurationChange,
  secondsLeft,
  correct,
  total,
  history,
  onStart,
  onReset,
  blurb,
  scoring = 'accuracy',
  ready = true,
  children
}: {
  mode: ModeSlug;
  state: SessionState;
  duration: Duration;
  onDurationChange: (d: Duration) => void;
  secondsLeft: number;
  correct: number;
  total: number;
  history: ReviewEntry[];
  onStart: () => void;
  onReset: () => void;
  blurb: string;
  scoring?: 'accuracy' | 'rate';
  ready?: boolean;
  children: React.ReactNode;
}) {
  useEffect(() => {
    if (state !== 'finished' || total === 0) return;
    recordSession(mode, { correct, total });
    // eslint-disable-next-line react-hooks/exhaustive-deps -- history is frozen once finished
  }, [state, mode, correct, total]);

  if (state === 'idle') {
    return (
      <div className={styles.gate}>
        <p className={styles.blurb}>{blurb}</p>
        <div className={styles.durations}>
          <span className="label">Session</span>
          <span className={styles.segmented}>
            {DURATIONS.map((d) => (
              <button
                key={String(d)}
                type="button"
                className={d === duration ? styles.segOn : styles.segOff}
                onClick={() => onDurationChange(d)}
              >
                {durationLabel(d)}
              </button>
            ))}
          </span>
        </div>
        <button className={styles.start} onClick={onStart} disabled={!ready}>
          Start
        </button>
      </div>
    );
  }

  if (state === 'finished') {
    return (
      <div className={styles.summary}>
        <div className={styles.scoreRow}>
          <span className="label">{scoring === 'rate' ? 'Solved' : 'Score'}</span>
          <span className={`num ${styles.bigScore}`}>
            {correct}
            {scoring === 'accuracy' && <span className={styles.outOf}> / {total}</span>}
          </span>
        </div>

        {history.length > 0 && (
          <>
            <span className={`label ${styles.reviewHeading}`}>Review</span>
            <ol className={styles.review}>
              {history.map((entry, i) => {
                const ladder = entry.handNotes !== undefined;

                const groups: Array<{ label: string; hands: typeof entry.hands }> = [];
                entry.hands.forEach((hand, h) => {
                  const label = entry.handLabels?.[h];
                  if (label || groups.length === 0) groups.push({ label: label ?? '', hands: [hand] });
                  else groups[groups.length - 1].hands.push(hand);
                });
                const verdict = entry.yours ?? entry.truth;
                return (
                  <li key={i} className={`${styles.entry} ${verdict ? '' : styles.entryWide}`}>
                    <span className={`num ${styles.entryIndex}`}>{i + 1}</span>

                    <span className={styles.entryCards}>
                      {ladder && entry.board.length > 0 && (
                        <span className={styles.entryBoard}>
                          <span className={`num ${styles.rungRank}`}>board</span>
                          <CardRow cards={entry.board} size="sm" />
                        </span>
                      )}

                      {ladder ? (
                        <span className={styles.rungs}>
                          {entry.hands.map((hand, h) => (
                            <span key={h} className={styles.rung}>
                              <CardRow cards={hand} size="sm" />
                              <span className={`num ${styles.rungLine}`}>
                                <span className={styles.rungRank}>{ORDINALS[h]}</span>{' '}
                                {entry.handNotes?.[h]}
                              </span>
                              <span
                                className={`num ${styles.rungAside} ${
                                  entry.handMarks?.[h] === undefined
                                    ? ''
                                    : entry.handMarks[h]
                                      ? styles.rungAsideGood
                                      : styles.rungAsideBad
                                }`}
                              >
                                {entry.handAsides?.[h]}
                              </span>
                            </span>
                          ))}
                        </span>
                      ) : (
                        <span className={styles.entryHands}>
                          {groups.map((group, g) => (
                            <span key={g} className={styles.group}>
                              {group.label && (
                                <span className={`num ${styles.groupLabel}`}>{group.label}</span>
                              )}
                              <span className={styles.groupCards}>
                                {group.hands.map((hand, h) => (
                                  <CardRow key={h} cards={hand} size="sm" />
                                ))}
                              </span>
                            </span>
                          ))}
                          {entry.board.length > 0 && (
                            <span className={styles.group}>
                              <span className={`num ${styles.groupLabel}`}>board</span>
                              <span className={styles.groupCards}>
                                <CardRow cards={entry.board} size="sm" />
                              </span>
                            </span>
                          )}
                        </span>
                      )}

                      {entry.detail && (
                        <span className={styles.entryDetail}>
                          {entry.detail.map((figure) => (
                            <span key={figure.label} className={styles.figure}>
                              <span className="label">{figure.label}</span>
                              <span className={`num ${styles.figureValue}`}>{figure.value}</span>
                            </span>
                          ))}
                        </span>
                      )}
                    </span>

                    {verdict && (
                      <span className={styles.entryAnswers}>
                        {entry.yours && (
                          <span className={`num ${entry.correct ? styles.good : styles.bad}`}>
                            {entry.yours}
                          </span>
                        )}
                        {entry.truth && (
                          <span className={`num ${styles.truth}`}>{entry.truth}</span>
                        )}
                      </span>
                    )}
                  </li>
                );
              })}
            </ol>
          </>
        )}

        <div className={styles.summaryActions}>
          <button onClick={onStart}>Play again</button>
          <button className={styles.secondary} onClick={onReset}>
            Change settings
          </button>
        </div>
      </div>
    );
  }

  return (
    <>
      <div className={styles.hud}>
        <span className={`num ${secondsLeft <= 10 ? styles.urgent : styles.clock}`}>
          {secondsLeft}s
        </span>
      </div>
      {children}
    </>
  );
}
