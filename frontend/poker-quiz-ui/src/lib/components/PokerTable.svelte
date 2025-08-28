<script>
  import { createEventDispatcher } from 'svelte';
  import Card from '$lib/components/deck.svelte';

  export let currentProblem = {};
  export let disabled = false;
  export let gameStats = null; // { time, score, feedbackClass }

  const dispatch = createEventDispatcher();

  // --- OPTIMIZATION: Initialize display variables to avoid template errors on first render ---
  let playerHandDisplay = [];
  let opponentHandDisplay = [];
  let boardDisplay = [];

  /**
   * Pads a card array with nulls to a specific length for consistent display.
   * @param {Array} cards - The array of card objects.
   * @param {number} length - The target length of the array.
   * @returns {Array} The padded array.
   */
  function padHand(cards = [], length) {
    const hand = [...cards];
    while (hand.length < length) hand.push(null);
    return hand;
  }

  // --- OPTIMIZATION: Consolidate reactive statements dependent on `currentProblem` ---
  // This block runs whenever `currentProblem` changes, updating all display hands at once.
  $: {
    if (currentProblem && Object.keys(currentProblem).length) {
      playerHandDisplay = padHand(currentProblem.player_hand, 2);
      opponentHandDisplay = padHand(currentProblem.opponent_hand, 2);
      boardDisplay = padHand(currentProblem.board, 5);
    } else {
      // Ensure hands are empty when there's no problem
      playerHandDisplay = padHand([], 2);
      opponentHandDisplay = padHand([], 2);
      boardDisplay = padHand([], 5);
    }
  }

  $: potBefore = currentProblem.pot_size ?? '?';
  $: betToCall = currentProblem.bet_to_call ?? '?';
  $: potAfter = (potBefore !== '?' && betToCall !== '?')
    ? potBefore + betToCall
    : '?';
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
              <span class="bet-amount">{betToCall}</span>
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
          <div class="board-row">
            <!-- Left side stats -->
            {#if gameStats}
              <div class="left-stat">
                <span class="time-text">{gameStats.time}s</span>
              </div>
            {:else}
              <div class="left-stat"></div>
            {/if}
            
            <!-- Center: Pot and Board -->
            <div class="center-area">
              <div class="pot-info">
                <div class="pot-chip">
                  <span class="pot-amount">{potAfter}</span>
                  <span class="pot-label">Pot (after shove)</span>
                </div>
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
            </div>
            
            <!-- Right side stats -->
            {#if gameStats}
              <div class="right-stat">
                <span 
                  class="score-text" 
                  class:correct-flash={gameStats.feedbackClass === 'correct-flash'} 
                  class:wrong-flash={gameStats.feedbackClass === 'wrong-flash'}
                >
                  {gameStats.score}
                </span>
              </div>
            {:else}
              <div class="right-stat"></div>
            {/if}
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
            <div class="call-info">
              <span class="call-amount">{betToCall}</span>
              <span class="call-label">to call</span>
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

  <div class="action-area">
    <button 
      class="action-btn call-btn" 
      on:click={() => dispatch('decision', { choseToCall: true })} 
      {disabled}
    >
      <span class="btn-key">[C]</span>
      <span class="btn-text">Call</span>
    </button>
    <button 
      class="action-btn fold-btn" 
      on:click={() => dispatch('decision', { choseToCall: false })} 
      {disabled}
    >
      <span class="btn-key">[F]</span>
      <span class="btn-text">Fold</span>
    </button>
  </div>
</div>


<style>

  .poker-table-wrapper {
    display: flex;
    flex-direction: column;
    align-items: center;
    gap: 1rem;
    max-width: 600px;
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
    border-radius: 50px;
    width: 100%;
    max-width: 600px;
    min-height: 350px;
    border: 3px solid #2d4a3e;
    box-shadow: 
      0 8px 25px rgba(0, 0, 0, 0.3),
      inset 0 1px 0 rgba(255, 255, 255, 0.1);
    display: flex;
    flex-direction: column;
    justify-content: space-between;
    align-items: center;
    padding: 1.5rem 1rem;
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
    gap: 0.4rem;
    min-height: 75px;
  }

  .board-container {
    display: flex;
    justify-content: center;
    align-items: center;
    gap: 0.4rem;
    background: rgba(0, 0, 0, 0.2);
    padding: 0.8rem;
    border-radius: 10px;
    width: calc(5 * 54px + 4 * 0.4rem + 2 * 0.8rem);
    border: 1px solid rgba(255, 255, 255, 0.1);
    height: 80px;
  }

  .community-area {
    display: flex;
    flex-direction: column;
    align-items: center;
    gap: 0.8rem;
    width: 100%;
  }

  .board-row {
    display: flex;
    align-items: center;
    justify-content: space-between;
    width: 100%;
    gap: 1rem;
  }

  .center-area {
    display: flex;
    flex-direction: column;
    align-items: center;
    gap: 0.8rem;
    flex: 1;
  }

  .left-stat, .right-stat {
    width: 60px;
    display: flex;
    justify-content: center;
    align-items: center;
  }

  .time-text, .score-text {
    font-size: 0.85rem;
    color: rgba(255, 255, 255, 0.7);
    font-weight: 500;
    padding: 0.25rem 0.5rem;
    border-radius: 4px;
    transition: all 0.2s ease;
  }

  .time-text {
    color: rgba(212, 175, 55, 0.8);
  }

  .score-text {
    color: rgba(16, 185, 129, 0.8);
  }

  .score-text.correct-flash {
    background: rgba(16, 185, 129, 0.3);
    color: #ffffff;
    transform: scale(1.1);
  }

  .score-text.wrong-flash {
    background: rgba(239, 68, 68, 0.3);
    color: #ffffff;
    transform: scale(1.1);
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

  .call-info {
    background: rgba(16, 185, 129, 0.2);
    border: 1px solid rgba(16, 185, 129, 0.4);
    border-radius: 8px;
    padding: 0.3rem 0.8rem;
    display: flex;
    flex-direction: column;
    align-items: center;
  }

  .call-amount {
    font-size: 1rem;
    font-weight: 700;
    color: #10b981;
  }

  .call-label {
    font-size: 0.7rem;
    color: rgba(255, 255, 255, 0.8);
    text-transform: uppercase;
  }

  .card-placeholder {
    width: 54px;
    height: 80px;
    margin: 0;
    border-radius: 6px;
    border: 2px dashed rgba(255, 255, 255, 0.2);
    background: rgba(255, 255, 255, 0.05);
  
  }

  .action-area {
    display: flex;
    gap: 1rem;
    justify-content: center;
    width: 100%;
    max-width: 350px;
  }

  .action-btn {
    flex: 1;
    display: flex;
    flex-direction: column;
    align-items: center;
    gap: 0.2rem;
    padding: 0.8rem 1.2rem;
    border-radius: 10px;
    border: none;
    font-family: inherit;
    font-weight: 600;
    cursor: pointer;
    transition: all 0.2s ease;
    box-shadow: 0 3px 10px rgba(0, 0, 0, 0.2);
    position: relative;
    overflow: hidden;
  }

  .action-btn::before {
    content: '';
    position: absolute;
    top: 0;
    left: -100%;
    width: 100%;
    height: 100%;
    background: linear-gradient(90deg, transparent, rgba(255, 255, 255, 0.2), transparent);
    transition: left 0.5s;
  }

  .action-btn:hover::before {
    left: 100%;
  }

  .call-btn {
    background: linear-gradient(135deg, #10b981 0%, #059669 100%);
    color: white;
  }

  .fold-btn {
    background: linear-gradient(135deg, #6b7280 0%, #4b5563 100%);
    color: white;
  }

  .action-btn:hover:not(:disabled) {
    transform: translateY(-2px);
    box-shadow: 0 6px 20px rgba(0, 0, 0, 0.3);
  }

  .action-btn:active:not(:disabled) {
    transform: translateY(0);
  }

  .action-btn:disabled {
    opacity: 0.5;
    cursor: not-allowed;
    transform: none;
  }

  .btn-key {
    font-size: 0.8rem;
    opacity: 0.8;
    font-weight: 500;
  }

  .btn-text {
    font-size: 1.1rem;
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
      border-radius: 30px;
      padding: 1rem 0.8rem;
      min-height: 300px;
    }
    
    .action-btn {
      padding: 0.7rem 1rem;
    }
    
    .btn-text {
      font-size: 1rem;
    }
  }

  @media (max-width: 480px) {
    .poker-table-wrapper {
      gap: 0.8rem;
    }
    
    .table-surface {
      border-radius: 25px;
      padding: 0.8rem;
      min-height: 280px;
    }
    
    .action-area {
      gap: 0.8rem;
    }
    
    .action-btn {
      padding: 0.6rem 0.8rem;
    }
    
    .hand-container, .board-container {
      min-height: 65px;
    }
    
    .left-stat, .right-stat {
      width: 40px;
    }
    
    .time-text, .score-text {
      font-size: 0.75rem;
    }
  }

  .player-area.opponent {
    display: flex;
    flex-direction: column;
    align-items: center;
    gap: 0.7rem;
    width: 100%;
    z-index: 1;
    transform: translateY(-20px);
  }

  
  .player-area.hero {
    display: flex;
    flex-direction: column;
    align-items: center;
    width: 100%;
    z-index: 1;
    margin-bottom: 1rem; /* Add some extra space below the opponent's cards */
    transform: translateY(30px);
  }

  
</style>