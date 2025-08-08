<script>
  import { createEventDispatcher } from 'svelte';
  import Card from '$lib/components/deck.svelte';

  export let currentProblem = {};
  export let disabled = false;

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

  // These reactive statements have different dependencies, so they remain separate.
  $: potSize = currentProblem.pot_size ?? '?';
  $: betToCall = currentProblem.bet_to_call ?? '?';
</script>

<div class="poker-table-wrapper">
  {#if Object.keys(currentProblem).length}
    <div class="poker-table">
      <div class="table-surface">
        <!-- Opponent's Area -->
        <div class="player-area opponent">
          <div class="hand-container">
            <!-- --- OPTIMIZATION: Added a unique key to the #each block --- -->
            <!-- This helps Svelte efficiently update the DOM instead of recreating it. -->
            {#each opponentHandDisplay as card, i (card ? `${card.rank}-${card.suit}` : `opp-ph-${i}`)}
              {#if card}
                <Card rank={card.rank} suit={card.suit} />
              {:else}
                <div class="card-placeholder"></div>
              {/if}
            {/each}
          </div>
          <div class="player-info">
            <h4>Opponent</h4>
            <span class="bet-info">All-in</span>
          </div>
        </div>

        <!-- Community Card Area -->
        <div class="community-area">
          <div class="pot-display">Pot: {potSize}</div>
          <div class="board-container">
            <!-- --- OPTIMIZATION: Added a unique key to the #each block --- -->
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
            <!-- --- OPTIMIZATION: Added a unique key to the #each block --- -->
            {#each playerHandDisplay as card, i (card ? `${card.rank}-${card.suit}` : `player-ph-${i}`)}
              {#if card}
                <Card rank={card.rank} suit={card.suit} />
              {:else}
                <div class="card-placeholder"></div>
              {/if}
            {/each}
          </div>
          <div class="player-info">
            <h4>Your Hand</h4>
            <span class="bet-info">To Call: {betToCall}</span>
          </div>
        </div>
      </div>
    </div>
  {:else}
    <div class="poker-table">
      <div class="table-surface">
        <p class="loading-info">Loading next hand...</p>
      </div>
    </div>
  {/if}

  <div class="decision-buttons">
    <button on:click={() => dispatch('decision', { choseToCall: true })} {disabled}>[C] Call</button>
    <button on:click={() => dispatch('decision', { choseToCall: false })} {disabled}>[F] Fold</button>
  </div>
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
    justify-content: center;
    align-items: center;
  }

  .table-surface {
    background-color: #0a6b3d;
    border-radius: 120px / 200px;
    width: 650px;
    height: 400px;
    border: 12px solid #5a3a2e;
    box-shadow: inset 0 0 15px rgba(0, 0, 0, 0.5);
    display: flex;
    flex-direction: column;
    justify-content: space-between;
    align-items: center;
    padding: 1.5rem;
    color: white;
    text-shadow: 1px 1px 2px rgba(0, 0, 0, 0.7);
  }

  .player-area {
    display: flex;
    flex-direction: column;
    align-items: center;
  }

  .hand-container, .board-container {
    display: flex;
    justify-content: center;
    align-items: center;
    min-height: 95px;
  }

  .player-info {
    background-color: rgba(0, 0, 0, 0.4);
    padding: 0.2rem 0.8rem;
    border-radius: 8px;
    text-align: center;
    margin-top: 0.3rem;
  }

  .player-info h4 {
    margin: 0.1rem;
    font-size: 0.9rem;
  }

  .bet-info {
    font-weight: bold;
    font-size: 1rem;
    color: #ffdb58;
  }

  .community-area {
    text-align: center;
  }

  .pot-display {
    font-size: 1.3rem;
    font-weight: bold;
    margin-bottom: 0.5rem;
  }

  .board-container {
    background-color: rgba(0, 0, 0, 0.2);
    padding: 0.4rem;
    border-radius: 8px;
  }

  .card-placeholder {
    box-sizing: border-box;
    width: 54px;
    height: 80px;
    margin: 3px;
    border-radius: 4px;
    border: 2px dashed rgba(255, 255, 255, 0.3);
  }

  .decision-buttons button {
    font-size: 1.3rem;
    padding: 0.7rem 2rem;
    margin: 0 0.8rem;
    border-radius: 8px;
    border: none;
    color: white;
    cursor: pointer;
    transition: background-color 0.2s;
  }

  .decision-buttons button:first-child {
      background-color: #28a745;
  }

  .decision-buttons button:last-child {
      background-color: #dc3545;
  }
  
  .decision-buttons button:hover:not(:disabled) {
    filter: brightness(1.1);
  }

  .decision-buttons button:disabled {
      background-color: #6c757d;
      cursor: not-allowed;
  }

  .loading-info {
    font-style: italic;
    font-size: 1.5rem;
  }
</style>