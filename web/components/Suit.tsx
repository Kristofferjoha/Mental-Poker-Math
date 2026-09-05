import type { Suit as SuitName } from '@/lib/types';

export function Suit({
  suit,
  className,
  style
}: {
  suit: SuitName;
  className?: string;
  style?: React.CSSProperties;
}) {
  return (
    <svg viewBox="0 0 24 24" className={className} style={style} aria-hidden="true" focusable="false">
      {suit === 'Spades' && (
        <path
          fill="currentColor"
          d="M12 2.4C12 2.4 21 9.3 21 15.1c0 3-2.3 5.2-5.1 5.2-1.4 0-2.5-.6-3.1-1.5.2 2 .8 3.6 1.7 4.3H9.5c.9-.7 1.5-2.3 1.7-4.3-.6.9-1.7 1.5-3.1 1.5C5.3 20.3 3 18.1 3 15.1 3 9.3 12 2.4 12 2.4z"
        />
      )}
      {suit === 'Hearts' && (
        <path
          fill="currentColor"
          d="M12 21.6S3 14.4 3 8.7C3 5.5 5.4 3 8.5 3c1.7 0 3 .9 3.5 1.9C12.5 3.9 13.8 3 15.5 3 18.6 3 21 5.5 21 8.7c0 5.7-9 12.9-9 12.9z"
        />
      )}
      {suit === 'Diamonds' && <path fill="currentColor" d="M12 1.8 20.4 12 12 22.2 3.6 12z" />}
      {suit === 'Clubs' && (
        <g fill="currentColor">
          <circle cx="12" cy="7.1" r="4.1" />
          <circle cx="6.6" cy="14.2" r="4.1" />
          <circle cx="17.4" cy="14.2" r="4.1" />
          <path d="M10.9 13.6h2.2c0 4.4.6 7.7 1.9 8.8H9c1.3-1.1 1.9-4.4 1.9-8.8z" />
        </g>
      )}
    </svg>
  );
}
