import type { Card as CardType } from '@/lib/types';
import { cardLabel, isRed } from '@/lib/cards';
import { PIP_LAYOUTS } from '@/lib/pips';
import { Suit } from './Suit';
import styles from './Card.module.css';

const SHORT: Record<CardType['rank'], string> = {
  Two: '2', Three: '3', Four: '4', Five: '5', Six: '6', Seven: '7',
  Eight: '8', Nine: '9', Ten: '10', Jack: 'J', Queen: 'Q', King: 'K', Ace: 'A'
};

export type CardSize = 'sm' | 'md' | 'lg';

export function Card({
  card,
  size = 'md',
  dimmed = false
}: {
  card: CardType | null;
  size?: CardSize;
  dimmed?: boolean;
}) {
  if (!card) {
    return <div className={`${styles.card} ${styles[size]} ${styles.slot}`} aria-hidden="true" />;
  }

  const layout = size === 'sm' ? undefined : PIP_LAYOUTS[card.rank];

  return (
    <div
      className={`${styles.card} ${styles[size]} ${isRed(card) ? styles.red : styles.black} ${
        dimmed ? styles.dimmed : ''
      }`}
      role="img"
      aria-label={cardLabel(card)}
    >
      <span className={styles.rank}>{SHORT[card.rank]}</span>
      {layout ? (
        <span className={styles.pips}>
          {layout.map(([column, row], i) => (
            <Suit
              key={i}
              suit={card.suit}
              className={`${styles.laidPip} ${row > 7 ? styles.inverted : ''}`}
              style={{ gridColumn: column, gridRow: row }}
            />
          ))}
        </span>
      ) : (
        <Suit suit={card.suit} className={styles.pip} />
      )}
    </div>
  );
}

export function CardRow({
  cards,
  slots,
  size = 'md',
  dimmed = false
}: {
  cards: CardType[];
  slots?: number;
  size?: CardSize;
  dimmed?: boolean;
}) {
  const total = slots ?? cards.length;
  return (
    <div className={styles.row}>
      {Array.from({ length: total }, (_, i) => (
        <Card key={i} card={cards[i] ?? null} size={size} dimmed={dimmed} />
      ))}
    </div>
  );
}
