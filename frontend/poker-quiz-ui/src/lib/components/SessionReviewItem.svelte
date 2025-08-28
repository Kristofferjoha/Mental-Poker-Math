<script>
    import Card from './deck.svelte'; 

    export let round;
    export let index;

    const yourDecisionText = round.userDecision ? 'Call' : 'Fold';
    const correctDecisionText = round.problem.correct_decision ? 'Call' : 'Fold';
    const potOddsRatio = (round.problem.pot_odds > 0) 
        ? `1:${(1 / round.problem.pot_odds).toFixed(1)}` 
        : '—';
    $: isEquityHigher = round.problem.player_equity > round.problem.pot_odds;
</script>

<div class="history-item" class:correct={round.isCorrect} class:wrong={!round.isCorrect}>
    <div class="header-row">
        <h4>Hand #{index + 1}</h4>
        <div class="decision-pills">
            <span class="decision-pill user-decision">{yourDecisionText}</span>
            {#if !round.isCorrect}
                <span class="decision-pill correct-decision">(Correct: {correctDecisionText})</span>
            {/if}
        </div>
    </div>

    <div class="cards-display">
        <div class="hand-group">
            <strong>Opponent:</strong>
            <div class="cards">
            {#each round.problem.opponent_hand as card}
                <Card rank={card.rank} suit={card.suit} />
            {/each}
            </div>
        </div>
        <div class="hand-group">
            <strong>Board:</strong>
            <div class="cards">
            {#each round.problem.board as card}
                <Card rank={card.rank} suit={card.suit} />
            {/each}
            </div>
        </div>
        <div class="hand-group">
            <strong>You:</strong>
            <div class="cards">
            {#each round.problem.player_hand as card}
                <Card rank={card.rank} suit={card.suit} />
            {/each}
            </div>
        </div>
    </div>

    <div class="details-row">
        <p>Pot Odds: <strong>{potOddsRatio}</strong></p>
        <p class:positive={isEquityHigher} class:negative={!isEquityHigher}>
            Equity: <strong>{(round.problem.player_equity * 100).toFixed(1)}%</strong>
        </p>
    </div>
</div>

<style>
    .history-item {
        border-radius: 8px;
        padding: 1rem;
        margin-bottom: 1rem;
        transition: box-shadow 0.2s ease;
    }
    .history-item:hover {
        box-shadow: 0 4px 8px rgba(0,0,0,0.08);
    }
    .correct { background-color: #e9f7ec; border-left: 5px solid #28a745; }
    .wrong { background-color: #fceaea; border-left: 5px solid #dc3545; }

    .header-row {
        display: flex;
        justify-content: space-between;
        align-items: center;
        margin-bottom: 0.8rem;
    }
    .header-row h4 {
        margin: 0;
        font-size: 1.1rem;
    }
    .decision-pills {
        display: flex;
        gap: 0.5rem;
    }
    .decision-pill {
        padding: 0.3rem 0.7rem;
        border-radius: 12px;
        font-size: 0.8rem;
        font-weight: 500;
    }
    .user-decision {
        background-color: #e0e0e0;
        color: #333;
    }
    .correct-decision {
        background-color: #fff0c1;
        color: #5d4000;
    }

    .cards-display {
        display: flex;
        justify-content: space-around;
        gap: 1rem;
        margin-bottom: 0.8rem;
    }
    .hand-group {
        text-align: center;
    }
    .hand-group strong {
        font-size: 0.8rem;
        color: #666;
        display: block;
        margin-bottom: 0.3rem;
    }
    .cards {
        display: flex;
        gap: 2px;
    }

    :global(.history-item .card-image) {
        height: 55px;
    }

    .details-row {
        display: flex;
        justify-content: space-between;
        font-size: 0.9rem;
        color: #555;
        border-top: 1px solid #ddd;
        padding-top: 0.8rem;
    }
    .details-row p {
        margin: 0;
    }
    .positive strong { color: #28a745; }
    .negative strong { color: #dc3545; }
</style>