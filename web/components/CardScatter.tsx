'use client';

import { useEffect, useMemo, useRef, useState } from 'react';
import { RANKS, SUITS, type Card as CardType } from '@/lib/types';
import { Card } from './Card';
import styles from './CardScatter.module.css';


const HOLD_MS = 420;

function mulberry32(seed: number) {
  return () => {
    seed = (seed + 0x6d2b79f5) | 0;
    let t = Math.imul(seed ^ (seed >>> 15), 1 | seed);
    t = (t + Math.imul(t ^ (t >>> 7), 61 | t)) ^ t;
    return ((t ^ (t >>> 14)) >>> 0) / 4294967296;
  };
}

type BandName = 'top' | 'bottom' | 'left' | 'right';

const BANDS: Array<{ name: BandName; columns: number; rows: number }> = [
  { name: 'top', columns: 8, rows: 2 },
  { name: 'bottom', columns: 8, rows: 2 },
  { name: 'left', columns: 2, rows: 5 },
  { name: 'right', columns: 2, rows: 5 }
];

interface Placed {
  card: CardType;
  band: BandName;
  left: number;
  top: number;
  rotation: number;
  delay: number;
}

function layout(): Placed[] {
  const random = mulberry32(0x5eed);

  const deck: CardType[] = [];
  for (const suit of SUITS) {
    for (const rank of RANKS) {
      deck.push({ rank, suit });
    }
  }
  for (let i = deck.length - 1; i > 0; i--) {
    const j = Math.floor(random() * (i + 1));
    [deck[i], deck[j]] = [deck[j], deck[i]];
  }

  const placed: Placed[] = [];
  let next = 0;

  for (const band of BANDS) {
    for (let row = 0; row < band.rows; row++) {
      for (let column = 0; column < band.columns; column++) {
        const card = deck[next++];
        if (!card) break;
        placed.push({
          card,
          band: band.name,
          left: ((column + 0.5 + (random() - 0.5) * 0.62) / band.columns) * 100,
          top: ((row + 0.5 + (random() - 0.5) * 0.62) / band.rows) * 100,
          rotation: (random() - 0.5) * 42,
          delay: random() * 550
        });
      }
    }
  }

  return placed;
}

function ScatterCard({ placed }: { placed: Placed }) {
  const [faceUp, setFaceUp] = useState(false);
  const timer = useRef<number | undefined>(undefined);

  useEffect(() => () => window.clearTimeout(timer.current), []);

  const turnUp = () => {
    window.clearTimeout(timer.current);
    setFaceUp(true);
  };

  const turnBack = () => {
    window.clearTimeout(timer.current);
    timer.current = window.setTimeout(() => setFaceUp(false), HOLD_MS);
  };

  return (
    <div
      className={`${styles.slot} ${faceUp ? styles.raised : ''}`}
      style={
        {
          left: `${placed.left}%`,
          top: `${placed.top}%`,
          '--rotation': `${placed.rotation}deg`,
          '--enter-delay': `${placed.delay}ms`
        } as React.CSSProperties
      }
      onPointerEnter={turnUp}
      onPointerLeave={turnBack}
    >
      <div className={`${styles.flipper} ${faceUp ? styles.faceUp : ''}`}>
        <div className={styles.back} />
        <div className={styles.front}>
          <Card card={placed.card} size="sm" />
        </div>
      </div>
    </div>
  );
}

export function CardScatter() {
  const placed = useMemo(layout, []);

  return (
    <div className={styles.scatter} aria-hidden="true">
      {BANDS.map((band) => (
        <div key={band.name} className={`${styles.band} ${styles[band.name]}`}>
          {placed
            .filter((p) => p.band === band.name)
            .map((p, i) => (
              <ScatterCard key={i} placed={p} />
            ))}
        </div>
      ))}
    </div>
  );
}
