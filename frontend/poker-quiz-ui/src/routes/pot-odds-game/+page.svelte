<script>
  import { onMount, onDestroy, tick } from 'svelte';
  import Card from '$lib/components/deck.svelte';
  import SessionReviewItem from '$lib/components/SessionReviewItem.svelte';

  const STACK_SIZE = 25;
  let problemStack = [];
  let currentProblem = null;
  let error = null;

  let gameState = 'ready'; 
  let score = 0;
  let timeLeft = 120;
  let timerInterval = null;
  let feedback = '';

  let sessionHistory = [];

  let feedbackClass = '';

  async function fetchProblem() {
    try {
      const res = await fetch('/api/new-problem');
      if (!res.ok) throw new Error(`Server error: ${res.status}`);
      return await res.json();
    } catch (e) {
      error = e.message;
      return null;
    }
  }

  async function refillStack() {
    while (problemStack.length < STACK_SIZE && !error) {
      const prob = await fetchProblem();
      if (prob) {
        problemStack = [...problemStack, prob];
        console.log(`Stack size: ${problemStack.length}`);
      } else {
        break; 
      }
    }
  }

  onMount(() => {
    refillStack();
  });

  function nextProblem() {
    feedback = '';
    if (problemStack.length > 0) {
      currentProblem = problemStack[0];
      problemStack = problemStack.slice(1);
      refillStack(); 
    } else {
      fetchProblem().then(prob => currentProblem = prob);
    }
  }

  async function handleDecision(userChoseToCall) {
    if (!currentProblem) return;

    const isCorrect = userChoseToCall === currentProblem.correct_decision;

    sessionHistory.push({
      problem: currentProblem,
      userDecision: userChoseToCall,
      isCorrect: isCorrect
    });

    if (isCorrect) {
      score += 1;
      feedbackClass = 'correct-flash';
    } else {
      score -= 1;
      feedbackClass = 'wrong-flash';
    }

    await tick(); // wait for DOM to update

    nextProblem();

    setTimeout(() => {
      feedbackClass = '';
    }, 300);
  }

  function startGame() {
    clearInterval(timerInterval);
    sessionHistory =[];
    score = 0;
    timeLeft = 120;
    gameState = 'playing';
    nextProblem();

    timerInterval = setInterval(() => {
      timeLeft -= 1;
      if (timeLeft <= 0) {
        endGame();
      }
    }, 1000);
  }

  function endGame() {
    clearInterval(timerInterval);
    gameState = 'finished';
    refillStack();
  }


  onMount(() => {
    const handleKeyDown = (e) => {
      if (!currentProblem) return;

      const key = e.key.toLowerCase();
      if (key === 'c') {
        handleDecision(true); 
      } else if (key === 'f') {
        handleDecision(false);
      }
    };

    window.addEventListener('keydown', handleKeyDown);
    return () => window.removeEventListener('keydown', handleKeyDown);
  });

  onDestroy(() => {
    clearInterval(timerInterval);
  });

</script>

<main>
  {#if gameState === 'ready'}
    <div class="menu-box">
      <h1>Pot Odds Decision</h1>
      <p>A 2-minute timed quiz. You will be shown a poker hand scenario where you are facing an all-in bet.</p>
      <p>Your goal is to quickly decide if calling is profitable (+EV) based on your hand's equity vs. the pot odds.</p>
      <button on:click={startGame}>Start Game</button>
      {#if problemStack.length > 0}
        <p class="loading-info">{problemStack.length} problems preloaded...</p>
      {:else}
        <p class="loading-info">Loading problems...</p>
      {/if}
    </div>

  {:else if gameState === 'playing' && currentProblem}
    <div class="game-header">
      <span>🕒 Time: {timeLeft}</span>
      <span class="score" class:correct-flash={feedbackClass === 'correct-flash'} class:wrong-flash={feedbackClass === 'wrong-flash'}>
      🏆 Score: {score}
      </span>
    </div>
    <div class="problem-display">
      <div class="hand-section">
        <h4>Opponent's Hand</h4>
        {#each currentProblem.opponent_hand as card} <Card rank={card.rank} suit={card.suit} /> {/each}
      </div>
      <div class="board-section">
        <h3>Board</h3>
        {#each currentProblem.board as card} <Card rank={card.rank} suit={card.suit} /> {/each}
      </div>
      <div class="hand-section">
        <h4>Your Hand</h4>
        {#each currentProblem.player_hand as card} <Card rank={card.rank} suit={card.suit} /> {/each}
      </div>
      <div class="info-section">
        <h2>Pot: {currentProblem.pot_size} | To Call: {currentProblem.bet_to_call}</h2>
      </div>
    </div>
    
    <div class="decision-buttons">
      <button id="call-btn" on:click={() => handleDecision(true)} disabled={!currentProblem}>[C] Call</button>
      <button id="fold-btn" on:click={() => handleDecision(false)} disabled={!currentProblem}>[F] Fold</button>
    </div>

    <div class="feedback">{feedback}&nbsp;</div>

  {:else if gameState === 'finished'}
    <div class="menu-box">
      <h2>Game Over!</h2>
      <p>Your final score is:</p>
      <p class="final-score">{score}</p>
      <button on:click={startGame}>Play Again</button>
      <a href="/" class="home-link">Back to Menu</a>
    </div>

    <div class="history-section">
      <h2>Hand History</h2>
      {#if sessionHistory.length > 0}
        {#each sessionHistory as round, index}
          <SessionReviewItem {round} {index} />
        {/each}
      {:else}
        <p>No hands were played.</p>
      {/if}
    </div>

  {:else}
    <div class="menu-box">
      <p>Loading...</p>
    </div>
  {/if}
</main>

<style>
  main { font-family: sans-serif; text-align: center; padding: 2rem; }
  .menu-box { max-width: 600px; margin: auto; padding: 2rem; border: 1px solid #ccc; border-radius: 8px; }
  .menu-box p { line-height: 1.6; }
  .loading-info { font-style: italic; color: #888; }
  button { padding: 1rem 2rem; font-size: 1.2rem; cursor: pointer; border-radius: 8px; border: none; background-color: #007bff; color: white; }
  .home-link { display: block; margin-top: 1rem; }
  
  .game-header { display: flex; justify-content: space-around; font-size: 1.5rem; margin-bottom: 1.5rem; }
  .problem-display { margin-bottom: 1rem; }
  .hand-section, .board-section, .info-section { margin-bottom: 1rem; }
  h2, h3, h4 { margin: 0.5rem; }
  .decision-buttons button { font-size: 1.5rem; padding: 0.8rem 2.5rem; margin: 0 1rem; }
  .feedback { font-weight: bold; font-size: 1.5rem; height: 2rem; }
  .final-score { font-size: 3rem; font-weight: bold; margin: 1rem 0; }

  .history-section {
    margin-top: 3rem;
    max-width: 600px;
    margin-left: auto;
    margin-right: auto;
  }

  .score {
    padding: 0.2rem 0.5rem;
    border-radius: 5px;
    transition: background-color 0.1s, transform 0.1s;
  }

  .correct-flash {
    animation: flash-green 0.3s ease-out;
  }

  .wrong-flash {
    animation: flash-red 0.3s ease-out;
  }

  @keyframes flash-green {
    0% { background-color: #28a745; color: white; transform: scale(1.2); }
    100% { background-color: transparent; color: initial; transform: scale(1); }
  }

  @keyframes flash-red {
    0% { background-color: #dc3545; color: white; transform: scale(1.2); }
    100% { background-color: transparent; color: initial; transform: scale(1); }
  }
</style>