<script>
  import { onMount, onDestroy, tick } from 'svelte';
  import Card from '$lib/components/deck.svelte';
  import SessionReviewItem from '$lib/components/SessionReviewItem.svelte';
  import PokerTable from '$lib/components/PokerTable.svelte'; 

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

  {:else if gameState === 'playing'}
    <div class="game-container">
      <div class="game-header">
        <span>🕒 Time: {timeLeft}</span>
        <span class="score" class:correct-flash={feedbackClass === 'correct-flash'} class:wrong-flash={feedbackClass === 'wrong-flash'}>
        🏆 Score: {score}
        </span>
      </div>

      <!--
        The PokerTable component now handles everything.
        We pass the decision handler down to it.
      -->
      <PokerTable
        {currentProblem}
        on:decision={event => handleDecision(event.detail.choseToCall)}
        disabled={!currentProblem}
      />

      <div class="feedback">{feedback}&nbsp;</div>
    </div>


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
        {#each sessionHistory as round (round.problem.id || index)}
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
  /* Reduce the main padding to make everything more compact */
  main {
    font-family: sans-serif;
    text-align: center;
    padding: 1rem;
  }

  .game-container {
    display: flex;
    flex-direction: column;
    align-items: center;
    gap: 1rem;
  }

  .menu-box {
    max-width: 600px;
    margin: auto;
    padding: 2rem;
    border: 1px solid #ccc;
    border-radius: 8px;
  }
  .menu-box p {
    line-height: 1.6;
  }
  .loading-info {
    font-style: italic;
    color: #888;
  }
  button {
    padding: 1rem 2rem;
    font-size: 1.2rem;
    cursor: pointer;
    border-radius: 8px;
    border: none;
    background-color: #007bff;
    color: white;
  }
  .home-link {
    display: block;
    margin-top: 1rem;
  }
  .game-header {
    display: flex;
    justify-content: space-around;
    font-size: 1.5rem;
    margin-bottom: 0.5rem;
    width: 100%;
    max-width: 700px;
  }

  .feedback {
    font-weight: bold;
    font-size: 1.5rem;
    height: 2rem;
  }
  .final-score {
    font-size: 3rem;
    font-weight: bold;
    margin: 1rem 0;
  }
  .history-section {
    margin-top: 3rem;
    max-width: 600px;
    margin-left: auto;
    margin-right: auto;
  }
  .score {
  position: relative; /* Needed for pseudo-element positioning */
  z-index: 1;
  padding: 0.2rem 0.5rem;
  border-radius: 5px;
  transition: transform 0.1s; /* Keep the transform transition */
  }


  .score::before {
    content: '';
    position: absolute;
    top: 0;
    left: 0;
    right: 0;
    bottom: 0;
    border-radius: 5px;
    opacity: 0;
    z-index: -1; /* Place it behind the text */
  }

  .correct-flash {
    animation: flash-green 0.3s ease-out;
  }
  .wrong-flash {
    animation: flash-red 0.3s ease-out;
  }

  @keyframes flash-green {
    0% { transform: scale(1.2); }
    40% { /* Let the color flash peak and hold briefly */ }
    100% { transform: scale(1); }
  }

  /* Animate the ::before pseudo-element's background and opacity */
  .correct-flash::before {
    background-color: #28a745;
    animation: flash-opacity 0.3s ease-out;
  }

  .wrong-flash::before {
    background-color: #dc3545;
    animation: flash-opacity 0.3s ease-out;
  }

  /* A single animation for the opacity flash */
  @keyframes flash-opacity {
    0% { opacity: 1; }
    100% { opacity: 0; }
  }

  /* We don't need the color change on the text itself, but if you want it: */
  @keyframes flash-red { /* or flash-green */
    0% { color: white; transform: scale(1.2); }
    100% { color: initial; transform: scale(1); }
  }
</style>