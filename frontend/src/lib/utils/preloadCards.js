const suits = ['clubs', 'diamonds', 'hearts', 'spades'];
const ranks = ['Ace', 'King', 'Queen', 'Jack', 'Ten', 'Nine', 'Eight', 'Seven', 'Six', 'Five', 'Four', 'Three', 'Two'];

let hasPreloaded = false;

export function preloadCardImages() {
    if (hasPreloaded) return;

    console.log('Preloading all 52 card images...');
    for (const suit of suits) {
        for (const rank of ranks) {
            const img = new Image();
            img.src = `/cards/${rank}_of_${suit}.svg`;
        }
    }
    hasPreloaded = true;
}