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
    let timeLeft = 10;
    let timerInterval = null;
    let feedback = '';

    let sessionHistory = [];

    let feedbackClass = '';

    async function fetchProblem() {
        try {
            const res = await fetch('/api/get-problem');
            if (!res.ok) throw new Error(`Server error: ${res.status}`);
            return await res.json();
        } catch (e) {
            error = e.message;
            return null;
        }
    }

    let isRefilling = false;

    async function refillStack() {
        if (isRefilling) return;
        isRefilling = true;

        while (problemStack.length < STACK_SIZE && !error) {
            const prob = await fetchProblem();
            if (prob) {
                problemStack = [...problemStack, prob];
                console.log(`Stack size: ${problemStack.length}`);
            } else {
                break;
            }
        }

        isRefilling = false;
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
        if (gameState !== 'playing') return;
        if (!currentProblem) return;

        try {
            const res = await fetch('/api/check-answer', {
                method: 'POST',
                headers: { 'Content-Type': 'application/json' },
                body: JSON.stringify({
                    problemId: currentProblem.problem_id,
                    decision: userChoseToCall,
                }),
            });
            if (!res.ok) throw new Error(`Server error: ${res.status}`);
            const data = await res.json();

            const isCorrect = data.isCorrect;
            const correctDecision = data.correctDecision;
            const playerEquity = data.playerEquity;
            const potOdds = data.potOdds;
            console.log(`Player equity: ${playerEquity}, Pot odds: ${potOdds}`);

            sessionHistory.push({
                problem: {
                    ...currentProblem,
                    player_equity: playerEquity,
                    pot_odds: potOdds,
                    correct_decision: correctDecision,
                },
                userDecision: userChoseToCall,
                correctDecision,
                isCorrect,
            });

            if (isCorrect) {
                score += 1;
                feedbackClass = 'correct-flash';
            } else {
                score -= 1;
                feedbackClass = 'wrong-flash';
            }

            await tick(); 

            nextProblem();

            setTimeout(() => {
                feedbackClass = '';
            }, 300);
        } catch (e) {
            error = e.message;
        }
    }


    function startGame() {
        clearInterval(timerInterval);
        sessionHistory =[];
        score = 0;
        timeLeft = 10;
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
        refillStack();

        const handleKeyDown = (e) => {
            // Check the game state right away
            if (gameState !== 'playing') {
                return;
            }
            
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
      <PokerTable
        {currentProblem}
        on:decision={event => handleDecision(event.detail.choseToCall)}
        disabled={!currentProblem}
        gameStats={{ time: timeLeft, score: score, feedbackClass }}
      />
    </div>


  {:else if gameState === 'finished'}
    <div class="game-over-box">
      <p>Your final score:</p>
      <p class="final-score">{score}</p>
      <button on:click={startGame}>Try Again</button>
    </div>

    <div class="history-section">
      <h2>Hand History</h2>
      {#if sessionHistory.length > 0}
        <div class="history-scroll">
          {#each sessionHistory as round, index (round.problem.problem_id)}
            <SessionReviewItem {round} {index} />
          {/each}
        </div>
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
  :root {
    --primary-color: #000000;
    --background-color: #f0f2f5;
    --container-bg: #ffffff;
    --text-color: #333;
    --border-radius: 12px;
    --box-shadow: 0 4px 12px rgba(0,0,0,0.1);
  }

  main {
    font-family: -apple-system, BlinkMacSystemFont, 'Segoe UI', Roboto, 'Helvetica Neue', Arial, sans-serif;
    text-align: center;
    padding: 1rem;
    background-color: var(--background-color);
    color: var(--text-color);
  }

  .game-container {
    display: flex;
    flex-direction: column;
    align-items: center;
    justify-content: center;
    min-height: 90vh;
    padding: 0.5rem;
  }

  .menu-box {
    max-width: 600px;
    margin: 2rem auto;
    padding: 2.5rem;
    background: var(--container-bg);
    border-radius: var(--border-radius);
    box-shadow: var(--box-shadow);
  }
  
  .menu-box p {
    line-height: 1.7;
    margin-bottom: 1rem;
  }

  .game-over-box {
    max-width: 250px;
    margin: auto;
    padding: 0.5rem;
    background: var(--container-bg);
    border-radius: var(--border-radius);
    box-shadow: var(--box-shadow);
  }
  
  .loading-info {
    font-style: italic;
    color: #888;
  }
  
  button {
    padding: 0.6rem 1.5rem;
    font-size: 1.1rem;
    cursor: pointer;
    border-radius: 8px;
    border: none;
    background-color: var(--primary-color);
    color: white;
    transition: background-color 0.2s ease, transform 0.2s ease;
  }
  
  button:hover {
    background-color: #46494c;
    transform: translateY(-2px);
  }

  .final-score {
    font-size: 3rem;
    font-weight: bold;
    margin-top: 0;
    margin-bottom: 1.5rem;
    color: var(--primary-color);
  }
  
  .history-section {
    margin-top: 3rem;
    max-width: 700px;
    margin-left: auto;
    margin-right: auto;
  }

  .history-scroll {
    max-height: 45vh;
    overflow-y: auto;  
    padding-right: 15px;
    border-top: 1px solid #ddd;
    padding-top: 1rem;
  }

  @media (max-width: 768px) {
    .menu-box, .game-over-box {
      padding: 1.5rem;
    }
  }
</style>