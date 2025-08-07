<script>
  import Card from './deck.svelte'; 

  export let round;
  export let index;
  const yourDecisionText = round.userDecision ? 'Call' : 'Fold';
  const correctDecisionText = round.problem.correct_decision ? 'Call' : 'Fold';
</script>

<div class="history-item" class:correct={round.isCorrect} class:wrong={!round.isCorrect}>
  <h3>Hand #{index + 1}</h3>
  
  <div class="card-display-area">
    <div class="hand-row">
      <strong>Opponent:</strong>
      <div class="cards">
        {#each round.problem.opponent_hand as card}
          <Card rank={card.rank} suit={card.suit} />
        {/each}
      </div>
    </div>
    <div class="hand-row">
      <strong>Board:</strong>
      <div class="cards">
        {#each round.problem.board as card}
          <Card rank={card.rank} suit={card.suit} />
        {/each}
      </div>
    </div>
    <div class="hand-row">
      <strong>You:</strong>
      <div class="cards">
        {#each round.problem.player_hand as card}
          <Card rank={card.rank} suit={card.suit} />
        {/each}
      </div>
    </div>
  </div>

  <div class="details">
    <p>Pot Odds Required: {Math.round(round.problem.pot_odds * 100)}%</p>
    <p>Your Equity: {Math.round(round.problem.player_equity * 100)}%</p>
    <p>Your Answer: <strong>{yourDecisionText}</strong></p>
    {#if !round.isCorrect}
      <p>Correct Answer: <strong>{correctDecisionText}</strong></p>
    {/if}
  </div>
</div>

<style>
  .history-item {
    border: 2px solid #ccc;
    border-radius: 8px;
    padding: 1rem;
    margin-bottom: 1rem;
    text-align: left;
  }
  .correct { border-color: #28a745; background-color: #e9f7ec; }
  .wrong { border-color: #dc3545; background-color: #fceaea; }

  .card-display-area {
    margin: 1rem 0;
  }
  .hand-row {
    display: flex;
    align-items: center;
    margin-bottom: 0.5rem;
  }
  .hand-row strong {
    min-width: 90px; 
  }
  .cards {
    display: flex;
    flex-wrap: wrap;
  }

  :global(.history-item .card-image) {
    height: 70px;
  }
  
  .details { margin-top: 1rem; }
  .details p { margin: 0.3rem 0; }
</style>