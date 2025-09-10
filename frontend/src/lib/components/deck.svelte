<script lang="ts">
    import { getCardImageUrl } from '$lib/utils/cardUtils.js';
    import type { Card as CardType } from '$lib/types';

    // We now accept the whole card object as a prop
    export let card: CardType;
</script>

<img
    src={getCardImageUrl(card)}
    alt={`${card.rank} of ${card.suit}`}
    class="card-image"
/>

<style>
    .card-image {
        height: 80px;
        width: 54px;
        box-shadow: 0 2px 5px rgba(0, 0, 0, 0.2);
    }
</style>