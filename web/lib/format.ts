export function signed(delta: number, digits = 2) {
  const sign = delta > 0 ? '+' : delta < 0 ? '−' : '±';
  return `${sign}${Math.abs(delta).toFixed(digits)}`;
}
