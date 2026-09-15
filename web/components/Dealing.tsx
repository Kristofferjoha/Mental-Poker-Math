'use client';

import { useEffect, useState } from 'react';
import styles from './Dealing.module.css';

/** Placeholder while the next problem is in flight.
 *  `reserve` holds the table's footprint so nothing below it moves when the deal lands. */
export function Dealing({ reserve = false }: { reserve?: boolean }) {
  const [show, setShow] = useState(false);

  useEffect(() => {
    const timer = window.setTimeout(() => setShow(true), 180);
    return () => window.clearTimeout(timer);
  }, []);

  if (!reserve) return show ? <p className={`label ${styles.dealing}`}>dealing</p> : null;

  return (
    <div className={styles.reserve}>
      {show && <p className={`label ${styles.dealing}`}>dealing</p>}
    </div>
  );
}
