import type { Card as CardType, Hands } from '@/lib/types';
import { CardRow } from './Card';
import styles from './Table.module.css';


const ARC_FROM = Math.PI * 1.08;
const ARC_TO = Math.PI * 1.92;

function villainPoint(index: number, total: number) {
  const t =
    total === 1 ? 0.5 : total === 2 ? (index === 0 ? 0.28 : 0.72) : index / (total - 1);
  const angle = ARC_FROM + t * (ARC_TO - ARC_FROM);
  return {
    left: `${50 + 40 * Math.cos(angle)}%`,
    top: `${52 + 36 * Math.sin(angle)}%`
  };
}

const STREETS = ['pre-flop', '', '', 'flop', 'turn', 'river'] as const;

export function Table({
  hands,
  board,
  equities,
  boardSlots = 5,
  pot,
  toCall
}: {
  hands: Hands;
  board: CardType[];
  equities?: number[];
  boardSlots?: number;
  pot?: number;
  toCall?: number;
}) {
  const [hero, ...villains] = hands;
  const seats = hands.length;

  return (
    <section
      className={styles.table}
      style={{ '--seats': seats } as React.CSSProperties}
      aria-label={`${seats}-handed spot, ${STREETS[board.length] || 'in progress'}`}
    >
      <div className={styles.ring}>
        {villains.map((cards, i) => (
          <div key={i} className={styles.seat} style={villainPoint(i, villains.length)}>
            <CardRow cards={cards} slots={2} size="sm" />
            {equities && <span className={`num ${styles.seatEquity}`}>{pct(equities[i + 1])}</span>}
          </div>
        ))}
      </div>

      <div className={styles.centre}>
        <span className={`label ${styles.street}`}>{STREETS[board.length]}</span>
        <CardRow
          cards={board}
          slots={board.length === 0 ? boardSlots : board.length}
          size="md"
          dimmed={board.length === 0}
        />

        {pot !== undefined && (
          <div className={styles.money}>
            <Money label="Pot" value={pot} />
            {toCall !== undefined && <Money label="To call" value={toCall} />}
          </div>
        )}
      </div>

      <div className={`${styles.seat} ${styles.hero}`}>
        <CardRow cards={hero} slots={2} size="md" />
        {equities ? (
          <span className={`num ${styles.heroEquity}`}>{pct(equities[0])}</span>
        ) : (
          <span className={`label ${styles.heroLabel}`}>You</span>
        )}
      </div>
    </section>
  );
}

function Money({ label, value }: { label: string; value: number }) {
  return (
    <span className={styles.moneyItem}>
      <span className="label">{label}</span>
      <span className={`num ${styles.moneyValue}`}>{value.toLocaleString()}</span>
    </span>
  );
}

function pct(value: number | undefined) {
  return value === undefined ? '' : `${(value * 100).toFixed(1)}%`;
}
