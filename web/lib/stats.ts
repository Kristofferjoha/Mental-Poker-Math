'use client';


export const MODES = ['pure-equity', 'pot-odds', 'whats-the-nuts', 'king-of-the-hill'] as const;
export type ModeSlug = (typeof MODES)[number];

export const RATE_MODES: readonly ModeSlug[] = ['pure-equity'];

export interface ModeRecord {
  sessions: number;
  answered: number;
  correct: number;
  bestCorrect: number;
  bestTotal: number;
}

export interface Stats {
  modes: Record<ModeSlug, ModeRecord>;
  lastPlayed: string | null;
  streak: number;
}

const KEY = 'mpm.stats.v1';

function emptyMode(): ModeRecord {
  return { sessions: 0, answered: 0, correct: 0, bestCorrect: 0, bestTotal: 0 };
}

export function emptyStats(): Stats {
  return {
    modes: {
      'pure-equity': emptyMode(),
      'pot-odds': emptyMode(),
      'whats-the-nuts': emptyMode(),
      'king-of-the-hill': emptyMode()
    },
    lastPlayed: null,
    streak: 0
  };
}

export function loadStats(): Stats {
  try {
    const raw = window.localStorage.getItem(KEY);
    if (!raw) return emptyStats();
    const parsed = JSON.parse(raw) as Partial<Stats>;
    const base = emptyStats();
    return {
      modes: { ...base.modes, ...(parsed.modes ?? {}) },
      lastPlayed: parsed.lastPlayed ?? null,
      streak: parsed.streak ?? 0
    };
  } catch {
    return emptyStats();
  }
}

function save(stats: Stats) {
  try {
    window.localStorage.setItem(KEY, JSON.stringify(stats));
  } catch {}
}

function today() {
  const now = new Date();
  const month = String(now.getMonth() + 1).padStart(2, '0');
  const day = String(now.getDate()).padStart(2, '0');
  return `${now.getFullYear()}-${month}-${day}`;
}

function daysBetween(from: string, to: string) {
  const ms = Date.parse(to) - Date.parse(from);
  return Math.round(ms / 86_400_000);
}

export function recordSession(
  mode: ModeSlug,
  result: { correct: number; total: number }
): Stats {
  const stats = loadStats();
  const record = stats.modes[mode] ?? emptyMode();

  record.sessions += 1;
  record.answered += result.total;
  record.correct += result.correct;
  if (
    result.correct > record.bestCorrect ||
    (result.correct === record.bestCorrect && record.bestTotal === 0)
  ) {
    record.bestCorrect = result.correct;
    record.bestTotal = result.total;
  }
  stats.modes[mode] = record;

  const now = today();
  if (stats.lastPlayed === null) {
    stats.streak = 1;
  } else {
    const gap = daysBetween(stats.lastPlayed, now);
    if (gap === 1) stats.streak += 1;
    else if (gap > 1) stats.streak = 1;
  }
  stats.lastPlayed = now;

  save(stats);
  return stats;
}

export function totals(stats: Stats) {
  const answered = Object.values(stats.modes).reduce((n, m) => n + m.answered, 0);
  return { answered };
}

export function clearStats() {
  try {
    window.localStorage.removeItem(KEY);
  } catch {}
}
