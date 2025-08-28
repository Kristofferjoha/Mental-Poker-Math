<script>
    import Card from '$lib/components/deck.svelte';

    export let currentProblem = {};
    export let hidePot = false;

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

    $: potSize = currentProblem.pot_size ?? '?';
    $: betToCall = currentProblem.bet_to_call ?? '?';
    $: allInAmount = potSize !== '?' && betToCall !== '?' ? (potSize - betToCall) : '?';
</script>

<div class="poker-table-wrapper">
    {#if Object.keys(currentProblem).length}
        <div class="poker-table">
            <div class="table-surface">
                <!-- Opponent's Area -->
                <div class="player-area opponent">
                    <div class="player-label">
                        <span class="player-name">Opponent</span>
                        <div class="bet-chip">
                            <span class="bet-amount">{allInAmount}</span>
                            <span class="bet-label">All-in</span>
                        </div>
                    </div>
                    <div class="hand-container">
                        {#each opponentHandDisplay as card, i (card ? `${card.rank}-${card.suit}` : `opp-ph-${i}`)}
                            {#if card}
                                <Card rank={card.rank} suit={card.suit} />
                            {:else}
                                <div class="card-placeholder"></div>
                            {/if}
                        {/each}
                    </div>
                </div>

                <!-- Community Card Area -->
                <div class="community-area">
                    {#if !hidePot}
                        <div class="pot-info">
                            <div class="pot-chip">
                                <span class="pot-amount">{potSize}</span>
                                <span class="pot-label">Pot</span>
                            </div>
                        </div>
                    {/if}
                    <div class="board-container">
                        {#each boardDisplay as card, i (card ? `${card.rank}-${card.suit}` : `board-ph-${i}`)}
                            {#if card}
                                <Card rank={card.rank} suit={card.suit} />
                            {:else}
                                <div class="card-placeholder"></div>
                            {/if}
                        {/each}
                    </div>
                </div>

                <!-- Player's Area -->
                <div class="player-area hero">
                    <div class="hand-container">
                        {#each playerHandDisplay as card, i (card ? `${card.rank}-${card.suit}` : `player-ph-${i}`)}
                            {#if card}
                                <Card rank={card.rank} suit={card.suit} />
                            {:else}
                                <div class="card-placeholder"></div>
                            {/if}
                        {/each}
                    </div>
                    <div class="player-label">
                        <span class="player-name">Your Hand</span>
                        <div class="equity-prompt">
                            <span class="equity-text">What's your equity?</span>
                        </div>
                    </div>
                </div>
            </div>
        </div>
    {:else}
        <div class="poker-table">
            <div class="table-surface">
                <div class="loading-content">
                    <div class="loading-spinner"></div>
                    <p class="loading-text">Dealing next hand...</p>
                </div>
            </div>
        </div>
    {/if}
</div>

<style>
    .poker-table-wrapper {
        display: flex;
        flex-direction: column;
        align-items: center;
        gap: 1.5rem;
        max-width: 800px;
        margin: 0 auto;
    }

    .poker-table {
        display: flex;
        justify-content: center;
        align-items: center;
        width: 100%;
    }

    .table-surface {
        background: linear-gradient(135deg, #1a5f3f 0%, #0d4d32 100%);
        border-radius: 60px;
        width: 100%;
        max-width: 700px;
        min-height: 450px;
        border: 3px solid #2d4a3e;
        box-shadow: 
            0 10px 30px rgba(0, 0, 0, 0.3),
            inset 0 1px 0 rgba(255, 255, 255, 0.1);
        display: flex;
        flex-direction: column;
        justify-content: space-between;
        align-items: center;
        padding: 2rem 1.5rem;
        color: white;
        position: relative;
    }

    .table-surface::before {
        content: '';
        position: absolute;
        top: 50%;
        left: 50%;
        transform: translate(-50%, -50%);
        width: 90%;
        height: 70%;
        border: 2px solid rgba(255, 255, 255, 0.1);
        border-radius: 40px;
        pointer-events: none;
    }

    .player-area {
        display: flex;
        flex-direction: column;
        align-items: center;
        gap: 1rem;
        width: 100%;
        z-index: 1;
    }

    .opponent {
        align-items: center;
    }

    .hero {
        align-items: center;
    }

    .player-label {
        display: flex;
        flex-direction: column;
        align-items: center;
        gap: 0.5rem;
    }

    .player-name {
        font-size: 0.9rem;
        font-weight: 600;
        color: rgba(255, 255, 255, 0.9);
        text-transform: uppercase;
        letter-spacing: 0.5px;
    }

    .hand-container {
        display: flex;
        justify-content: center;
        align-items: center;
        gap: 0.5rem;
        min-height: 90px;
    }

    .board-container {
        display: flex;
        justify-content: center;
        align-items: center;
        gap: 0.5rem;
        background: rgba(0, 0, 0, 0.2);
        padding: 1rem;
        border-radius: 12px;
        border: 1px solid rgba(255, 255, 255, 0.1);
        min-height: 90px;
    }

    .community-area {
        display: flex;
        flex-direction: column;
        align-items: center;
        gap: 1rem;
        width: 100%;
    }

    .pot-info {
        display: flex;
        justify-content: center;
    }

    .pot-chip, .bet-chip {
        background: linear-gradient(135deg, #d4af37 0%, #b8941f 100%);
        border: 2px solid #f4e4a6;
        border-radius: 20px;
        padding: 0.5rem 1rem;
        display: flex;
        flex-direction: column;
        align-items: center;
        box-shadow: 0 4px 8px rgba(0, 0, 0, 0.3);
        color: #2a2a2a;
        font-weight: 700;
    }

    .bet-chip {
        background: linear-gradient(135deg, #ef4444 0%, #dc2626 100%);
        border-color: #fca5a5;
        color: white;
    }

    .pot-amount, .bet-amount {
        font-size: 1.1rem;
        line-height: 1;
    }

    .pot-label, .bet-label {
        font-size: 0.7rem;
        text-transform: uppercase;
        letter-spacing: 0.5px;
        opacity: 0.9;
    }

    .equity-prompt {
        background: rgba(59, 130, 246, 0.2);
        border: 1px solid rgba(59, 130, 246, 0.4);
        border-radius: 8px;
        padding: 0.3rem 0.8rem;
        display: flex;
        flex-direction: column;
        align-items: center;
    }

    .equity-text {
        font-size: 0.8rem;
        color: rgba(255, 255, 255, 0.9);
        font-weight: 500;
    }

    .card-placeholder {
        width: 54px;
        height: 80px;
        margin: 0;
        border-radius: 6px;
        border: 2px dashed rgba(255, 255, 255, 0.2);
        background: rgba(255, 255, 255, 0.05);
    }

    .loading-content {
        display: flex;
        flex-direction: column;
        align-items: center;
        gap: 1rem;
    }

    .loading-spinner {
        width: 40px;
        height: 40px;
        border: 3px solid rgba(255, 255, 255, 0.3);
        border-top: 3px solid #d4af37;
        border-radius: 50%;
        animation: spin 1s linear infinite;
    }

    .loading-text {
        font-size: 1.1rem;
        color: rgba(255, 255, 255, 0.8);
        margin: 0;
    }

    @keyframes spin {
        0% { transform: rotate(0deg); }
        100% { transform: rotate(360deg); }
    }

    /* Responsive Design */
    @media (max-width: 768px) {
        .table-surface {
            border-radius: 40px;
            padding: 1.5rem 1rem;
            min-height: 400px;
        }
    }

    @media (max-width: 480px) {
        .poker-table-wrapper {
            gap: 1rem;
        }
        
        .table-surface {
            border-radius: 30px;
            padding: 1rem;
            min-height: 350px;
        }
    }
</style>