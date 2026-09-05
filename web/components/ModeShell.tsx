import Link from 'next/link';
import styles from './ModeShell.module.css';

export function ModeShell({
  title,
  score,
  controls,
  error,
  children
}: {
  title: string;
  score?: { correct: number; total: number; rate?: boolean };
  controls?: React.ReactNode;
  error?: string | null;
  children: React.ReactNode;
}) {
  return (
    <main className={styles.main}>
      <div className={styles.nav}>
        <Link href="/" className={styles.back}>← Modes</Link>
        <h1 className={styles.title}>{title}</h1>
        {score && (
          <span className={`num ${styles.score}`}>
            {score.rate ? (
              <>
                {score.correct}
                <span className={styles.suffix}> solved</span>
              </>
            ) : (
              <>
                {score.correct} / {score.total}
              </>
            )}
          </span>
        )}
      </div>

      {controls && <div className={styles.controls}>{controls}</div>}
      {error && <p className={styles.error}>{error}</p>}
      {children}
    </main>
  );
}

export function Control({ label, children }: { label: string; children: React.ReactNode }) {
  return (
    <span className={styles.control}>
      <span className="label">{label}</span>
      {children}
    </span>
  );
}

export function Segmented<T extends string>({
  options,
  selected,
  onToggle
}: {
  options: readonly T[];
  selected: readonly T[];
  onToggle: (value: T) => void;
}) {
  return (
    <span className={styles.segmented}>
      {options.map((option) => (
        <button
          key={option}
          type="button"
          className={selected.includes(option) ? styles.segOn : styles.segOff}
          onClick={() => onToggle(option)}
        >
          {option}
        </button>
      ))}
    </span>
  );
}
