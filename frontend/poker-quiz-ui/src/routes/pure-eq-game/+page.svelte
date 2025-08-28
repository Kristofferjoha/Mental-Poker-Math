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
            <div class="input-area">
                <input
                    type="number"
                    step="0.1"
                    bind:value={answer}
                    placeholder="Your equity %"
                    bind:this={answerInput}
                    class="equity-input"
                />
                <div class="input-hint">Enter percentage (e.g., 65.2)</div>
            </div>
        {:else}
            <div class="loading-state">
                <div class="loading-spinner"></div>
                <p>Loading problem...</p>
            </div>
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
    main { 
        font-family: 'Inter', system-ui, sans-serif; 
        text-align: center; 
        padding: 1rem; 
        max-width: 1000px;
        margin: 0 auto;
    }
    
    .menu-box { 
        max-width: 600px; 
        margin: auto; 
        padding: 2rem; 
        background: #1e1e1e;
        border: 1px solid #3f3f46;
        border-radius: 12px; 
        box-shadow: 0 10px 30px rgba(0, 0, 0, 0.3);
    }
    
    .menu-box h1 {
        background: linear-gradient(135deg, #d4af37 0%, #fbbf24 100%);
        -webkit-background-clip: text;
        -webkit-text-fill-color: transparent;
        background-clip: text;
        margin-bottom: 1rem;
    }
    
    .menu-box p {
        color: #a1a1aa;
        line-height: 1.6;
        margin-bottom: 2rem;
    }
    
    button { 
        padding: 1rem 2rem; 
        font-size: 1.2rem; 
        cursor: pointer; 
        border-radius: 12px; 
        border: none; 
        background: linear-gradient(135deg, #d4af37 0%, #b8941f 100%);
        color: #2a2a2a; 
        font-weight: 600;
        transition: all 0.2s ease;
        box-shadow: 0 4px 12px rgba(0, 0, 0, 0.2);
    }
    
    button:hover {
        transform: translateY(-2px);
        box-shadow: 0 6px 20px rgba(0, 0, 0, 0.3);
    }
    
    .input-area {
        display: flex;
        flex-direction: column;
        align-items: center;
        gap: 0.5rem;
        margin-top: 1.5rem;
    }
    
    .equity-input { 
        padding: 1rem 1.5rem; 
        font-size: 1.3rem; 
        width: 240px; 
        text-align: center;
        border: 2px solid #3f3f46;
        border-radius: 12px;
        background: #2a2a2a;
        color: white;
        font-family: inherit;
        font-weight: 500;
        transition: all 0.2s ease;
    }
    
    .equity-input:focus {
        outline: none;
        border-color: #d4af37;
        box-shadow: 0 0 0 3px rgba(212, 175, 55, 0.2);
    }
    
    .input-hint {
        font-size: 0.875rem;
        color: #71717a;
        font-style: italic;
    }
    
    .game-header { 
        display: flex; 
        justify-content: space-around; 
        font-size: 1.5rem; 
        margin-bottom: 1.5rem; 
        width: 100%;
        max-width: 700px;
        margin-left: auto;
        margin-right: auto;
        padding: 1rem;
        background: rgba(30, 30, 30, 0.8);
        border-radius: 12px;
        border: 1px solid #3f3f46;
    }
    
    .game-header span {
        display: flex;
        align-items: center;
        gap: 0.5rem;
        font-weight: 600;
    }
    
    .loading-state {
        display: flex;
        flex-direction: column;
        align-items: center;
        gap: 1rem;
        padding: 2rem;
        color: #a1a1aa;
    }
    
    .loading-spinner {
        width: 40px;
        height: 40px;
        border: 3px solid rgba(255, 255, 255, 0.3);
        border-top: 3px solid #d4af37;
        border-radius: 50%;
        animation: spin 1s linear infinite;
    }
    
    @keyframes spin {
        0% { transform: rotate(0deg); }
        100% { transform: rotate(360deg); }
    }
    
    .correct-flash, .wrong-flash { 
        position: relative;
        transition: all 0.3s ease-out; 
    }
    
    .score {
        padding: 0.5rem 1rem;
        border-radius: 8px;
        background: rgba(212, 175, 55, 0.1);
        border: 1px solid rgba(212, 175, 55, 0.3);
    }
    
    .score.correct-flash { 
        background: linear-gradient(135deg, #10b981 0%, #059669 100%);
        color: white; 
        transform: scale(1.05);
    }
    
    .score.wrong-flash { 
        background: linear-gradient(135deg, #ef4444 0%, #dc2626 100%);
        color: white; 
        transform: scale(1.05);
    }
    
    @media (max-width: 768px) {
        .game-header {
            font-size: 1.2rem;
            flex-direction: column;
            gap: 1rem;
        }
        
        .equity-input {
            width: 200px;
            font-size: 1.1rem;
            padding: 0.8rem 1.2rem;
        }
    }
</style>