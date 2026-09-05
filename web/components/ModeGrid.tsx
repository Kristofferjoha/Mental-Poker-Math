'use client';

import { useEffect, useState } from 'react';
import Link from 'next/link';
import { loadStats, RATE_MODES, totals, type ModeSlug, type Stats } from '@/lib/stats';
import styles from './ModeGrid.module.css';

export interface ModeLink {
  slug: ModeSlug;
  title: string;
  blurb: string;
}

export function ModeGrid({ modes }: { modes: readonly ModeLink[] }) {
  const [stats, setStats] = useState<Stats | null>(null);

  useEffect(() => {
    setStats(loadStats());
  }, []);

  const summary = stats ? totals(stats) : null;
  const hasHistory = summary !== null && summary.answered > 0;

  return (
    <>
      <div className={styles.grid}>
        {modes.map((mode) => {
          const record = stats?.modes[mode.slug];
          const played = record && record.sessions > 0;
          const rate = RATE_MODES.includes(mode.slug);
          return (
            <Link key={mode.slug} href={`/${mode.slug}`} className={styles.tile}>
              <span className={styles.tileTitle}>{mode.title}</span>
              <span className={styles.tileBlurb}>{mode.blurb}</span>
              <span className={`num ${styles.tileBest}`}>
                {!played
                  ? 'not played yet'
                  : rate
                    ? `best ${record.bestCorrect}`
                    : `best ${record.bestCorrect}/${record.bestTotal}`}
              </span>
            </Link>
          );
        })}
      </div>

      {hasHistory && stats && (
        <div className={styles.record}>
          <Figure value={summary.answered.toLocaleString()} label="answered" />
          {stats.streak > 0 && <Figure value={`${stats.streak}d`} label="streak" />}
        </div>
      )}
    </>
  );
}

function Figure({ value, label }: { value: string; label: string }) {
  return (
    <span className={styles.figure}>
      <span className={`num ${styles.figureValue}`}>{value}</span>
      <span className="label">{label}</span>
    </span>
  );
}
