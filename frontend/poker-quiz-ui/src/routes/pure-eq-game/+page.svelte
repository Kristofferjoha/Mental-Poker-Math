<script>
    import { onMount, onDestroy, tick } from 'svelte';
    import PureEqPokerTable from '$lib/components/PureEqPokerTable.svelte';

    const STACK_SIZE = 25;
    let problemStack = [];
    let currentProblem = null;
    let error = null;

    let gameState = 'ready';
    let score = 0;
    let timeLeft = 60;
    let timerInterval = null;
    let feedbackClass = '';

    let isRefilling = false;
    let answer = '';
    let answerInput;
    let debounceTimeout;

    $: if (answer && gameState === 'playing' && currentProblem) {
        debouncedCheckAnswer();
    }

    function debouncedCheckAnswer() {
        clearTimeout(debounceTimeout);
        debounceTimeout = setTimeout(() => {
            checkAnswer();
        }, 300);
    }
    // ----------------------

    async function fetchProblem() {
        try {
            const res = await fetch('/api/pure-eq-get-problem');
            if (!res.ok) throw new Error(`Server error: ${res.status}`);
            return await res.json();
        } catch (e) {
            error = e.message;
            return null;
        }
    }

    async function refillStack() {
        if (isRefilling) return;
        isRefilling = true;
        while (problemStack.length < STACK_SIZE && !error) {
            const prob = await fetchProblem();
            if (prob) problemStack = [...problemStack, prob];
            else break;
        }
        isRefilling = false;
    }

    async function nextProblem() {
        answer = '';
        feedbackClass = '';
        if (problemStack.length > 0) {
            currentProblem = problemStack[0];
            problemStack = problemStack.slice(1);
            refillStack();
        } else {
            const prob = await fetchProblem();
            currentProblem = prob;
        }
        await tick();
        if (answerInput) {
            answerInput.focus();
        }
    }

    async function checkAnswer() {
        const answerString = String(answer);
        const guessVal = parseFloat(answerString);

        if (isNaN(guessVal) || !currentProblem) {
            return;
        }

        try {
            const res = await fetch('/api/pure-eq-check-answer', {
                method: 'POST',
                headers: { 'Content-Type': 'application/json' },
                body: JSON.stringify({
                    problemId: currentProblem.problem_id,
                    guess_value: guessVal
                }),
            });
            if (!res.ok) throw new Error(`Server error: ${res.status}`);
            const data = await res.json();

            if (data.isCorrect) {
                score++;
                feedbackClass = 'correct-flash';
                nextProblem();
            } else {
                feedbackClass = 'wrong-flash';
            }

            await tick();
        } catch (e) {
            error = e.message;
        }
    }

    function startGame() {
        score = 0;
        timeLeft = 60;
        gameState = 'playing';
        refillStack().then(nextProblem);

        timerInterval = setInterval(() => {
            timeLeft--;
            if (timeLeft <= 0) endGame();
        }, 1000);
    }

    function endGame() {
        clearInterval(timerInterval);
        gameState = 'finished';
    }

    onMount(() => {
        refillStack();
    });

    onDestroy(() => {
        clearInterval(timerInterval);
        clearTimeout(debounceTimeout);
    });
</script>

<main>
    {#if gameState === 'ready'}
        <div class="menu-box">
            <h1>PureEq Poker Equity Speed Test</h1>
            <p>Type the estimated player equity. The game will automatically check your answer.</p>
            <button on:click={startGame}>Start Game</button>
        </div>

    {:else if gameState === 'playing'}
        <div class="game-header">
            <span>🕒 {timeLeft}</span>
            <span class="score" class:correct-flash={feedbackClass === 'correct-flash'} class:wrong-flash={feedbackClass === 'wrong-flash'}>
                🏆 {score}
            </span>
        </div>

        {#if currentProblem}
            <PureEqPokerTable
                {currentProblem}
                hidePot={true}
            />
            <input
                type="number"
                step="0.1"
                bind:value={answer}
                placeholder="Your equity %"
                bind:this={answerInput}
            />
        {:else}
            <p>Loading problem...</p>
        {/if}

    {:else if gameState === 'finished'}
        <div class="menu-box">
            <h2>Time's up!</h2>
            <p>Your final score: {score}</p>
            <button on:click={startGame}>Play Again</button>
        </div>
    {/if}
</main>

<style>
    main { font-family: sans-serif; text-align: center; padding: 1rem; }
    .menu-box { max-width: 600px; margin: auto; padding: 2rem; border: 1px solid #ccc; border-radius: 8px; }
    button { padding: 1rem 2rem; font-size: 1.2rem; cursor: pointer; border-radius: 8px; border: none; background-color: #007bff; color: white; }
    input { padding: 0.5rem; font-size: 1.2rem; margin-top: 1rem; width: 200px; text-align: center; }
    .game-header { display: flex; justify-content: space-around; font-size: 1.5rem; margin-bottom: 1rem; }
    .correct-flash { animation: flash-green 0.3s ease-out; }
    .wrong-flash { animation: flash-red 0.3s ease-out; }
    @keyframes flash-green {
        0% { background-color: #28a745; }
        100% { background-color: transparent; }
    }
    @keyframes flash-red {
        0% { background-color: #dc3545; }
        100% { background-color: transparent; }
    }
    .correct-flash, .wrong-flash { animation: none; transition: background-color 0.1s ease-out; }
    .score.correct-flash { background-color: #28a745; color: white; border-radius: 5px; padding: 0 5px; }
    .score.wrong-flash { background-color: #dc3545; color: white; border-radius: 5px; padding: 0 5px; }
</style>