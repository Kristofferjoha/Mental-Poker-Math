<script>
    import Card from '$lib/components/deck.svelte';

    export let currentProblem = {};

    let playerHandDisplay = [];
    let opponentHandDisplay = [];
    let boardDisplay = [];

    function padHand(cards = [], length) {
        const hand = [...cards];
        while (hand.length < length) hand.push(null);
        return hand;
    }

    $: {
        if (currentProblem && Object.keys(currentProblem).length) {
            playerHandDisplay = padHand(currentProblem.player_hand, 2);
            opponentHandDisplay = padHand(currentProblem.opponent_hand, 2);
            boardDisplay = padHand(currentProblem.board, 5);
        } else {
            playerHandDisplay = padHand([], 2);
            opponentHandDisplay = padHand([], 2);
            boardDisplay = padHand([], 5);
        }
    }
</script>

<div class="poker-table-wrapper">
    {#if Object.keys(currentProblem).length}
        <div class="poker-table">
            <div class="hand-container opponent-hand">
                {#each opponentHandDisplay as card, i (card ? `${card.rank}-${card.suit}` : `opp-ph-${i}`)}
                    {#if card}
                        <Card rank={card.rank} suit={card.suit} />
                    {:else}
                        <div class="card-placeholder"></div>
                    {/if}
                {/each}
            </div>

            <div class="board-container">
                {#each boardDisplay as card, i (card ? `${card.rank}-${card.suit}` : `board-ph-${i}`)}
                    {#if card}
                        <Card rank={card.rank} suit={card.suit} />
                    {:else}
                        <div class="card-placeholder"></div>
                    {/if}
                {/each}
            </div>

            <div class="hand-container player-hand">
                {#each playerHandDisplay as card, i (card ? `${card.rank}-${card.suit}` : `player-ph-${i}`)}
                    {#if card}
                        <Card rank={card.rank} suit={card.suit} />
                    {:else}
                        <div class="card-placeholder"></div>
                    {/if}
                {/each}
            </div>
        </div>
    {:else}
        <p class="loading-info">Loading hand...</p>
    {/if}
</div>

<style>
    .poker-table-wrapper {
        display: flex;
        flex-direction: column;
        align-items: center;
        gap: 1rem;
    }

    .poker-table {
        display: flex;
        flex-direction: column;
        align-items: center;
        gap: 1rem;
        background-color: #0a6b3d;
        padding: 1rem;
        border-radius: 20px;
    }

    .hand-container, .board-container {
        display: flex;
        justify-content: center;
        align-items: center;
        min-height: 80px;
        gap: 5px;
    }

    .card-placeholder {
        width: 54px;
        height: 80px;
        border: 2px dashed rgba(255, 255, 255, 0.3);
        border-radius: 4px;
    }

    .loading-info {
        font-style: italic;
        font-size: 1.2rem;
        color: white;
    }
</style>