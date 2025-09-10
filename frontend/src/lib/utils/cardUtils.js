/**
 * Takes a card object from the backend and returns the correct image URL path.
 * @param { { rank: string; suit: string; } | null } card
 */
export function getCardImageUrl(card) {
    if (!card || !card.rank || !card.suit) {
        return '';
    }

    const rank = card.rank;
    const suit = card.suit.toLowerCase();

    return `/cards/${rank}_of_${suit}.svg`;
}