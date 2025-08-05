<script>
  import { onMount } from 'svelte';
  import Deck from '$lib/components/deck.svelte';

  let problem = null;
  let error = null;

  onMount(async () => {
    try {
      console.log("Attempting to fetch a new problem...");
      const res = await fetch('/api/new-problem');
      console.log("Fetch response:", res);

      if (!res.ok) {
        console.error("Response not OK. Status:", res.status);
        throw new Error("Failed to fetch problem");
      }

      problem = await res.json();
      console.log("Problem data:", problem);

    } catch (e) {
      console.error("Caught an error during fetch:", e);
      error = e.message;
    }
  });
</script>

{#if error}
  <p class="error">Error: {error}</p>
{:else if !problem}
  <p>Loading...</p>
{:else}
  <div class="section">
    <h2>Player Hand</h2>
    <div class="hand">
      {#each problem.player_hand as card}
        <Deck rank={card.rank} suit={card.suit} />
      {/each}
    </div>

    <h2>Opponent Hand</h2>
    <div class="hand">
      {#each problem.opponent_hand as card}
        <Deck rank={card.rank} suit={card.suit} />
      {/each}
    </div>

    <h2>Board</h2>
    <div class="board">
      {#each problem.board as card}
        <Deck rank={card.rank} suit={card.suit} />
      {/each}
    </div>

    <div class="details">
      <p><strong>Stage:</strong> {problem.stage}</p>
      <p><strong>Pot Size:</strong> ${problem.pot_size}</p>
      <p><strong>Bet to Call:</strong> ${problem.bet_to_call}</p>
      <p><strong>Player Equity:</strong> {problem.player_equity}%</p>
      <p><strong>Pot Odds:</strong> {problem.pot_odds}%</p>
      <p><strong>Correct Decision:</strong> {problem.correct_decision}</p>
    </div>
  </div>
{/if}

<style>
  .section {
    padding: 20px;
    font-family: sans-serif;
  }

  .hand, .board {
    display: flex;
    margin-bottom: 10px;
  }

  .details {
    margin-top: 20px;
    background: #f9f9f9;
    padding: 15px;
    border-radius: 8px;
  }

  .details p {
    margin: 5px 0;
  }

  .error {
    color: red;
  }
</style>