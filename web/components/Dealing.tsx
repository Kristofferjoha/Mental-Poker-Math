'use client';

import { useEffect, useState } from 'react';
import styles from './Dealing.module.css';

export function Dealing() {
  const [show, setShow] = useState(false);

  useEffect(() => {
    const timer = window.setTimeout(() => setShow(true), 180);
    return () => window.clearTimeout(timer);
  }, []);

  if (!show) return null;
  return <p className={`label ${styles.dealing}`}>dealing</p>;
}
