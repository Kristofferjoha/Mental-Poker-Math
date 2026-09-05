import { CardScatter } from '@/components/CardScatter';
import { ModeGrid, type ModeLink } from '@/components/ModeGrid';
import styles from './page.module.css';

const MODES: readonly ModeLink[] = [
  {
    slug: 'pure-equity',
    title: 'Equity Intuition',
    blurb: 'Estimate your share of the pot, two to six handed.'
  },
  {
    slug: 'pot-odds',
    title: 'Pot Odds & EV',
    blurb: 'Facing a bet. Is calling profitable?'
  },
  {
    slug: 'whats-the-nuts',
    title: "What's the Nuts",
    blurb: 'A board appears. Find the best hand.'
  },
  {
    slug: 'king-of-the-hill',
    title: 'King of the Hill',
    blurb: 'Four to six hands. Rank them strongest to weakest.'
  }
];

export default function Home() {
  return (
    <main className={styles.stage}>
      <CardScatter />

      <div className={styles.centre}>
        <h1 className={styles.srOnly}>Mental Poker Math</h1>

        <ModeGrid modes={MODES} />
      </div>
    </main>
  );
}
