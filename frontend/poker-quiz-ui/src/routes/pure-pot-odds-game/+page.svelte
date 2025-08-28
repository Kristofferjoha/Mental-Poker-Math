<script>
    import { onMount, onDestroy, tick } from 'svelte';
    import PotOddsDisplay from '$lib/components/PotOddsDisplay.svelte';

    const STACK_SIZE = 25;
    let problemStack = [];
    let currentProblem = null;
    let error = null;

    let gameState = 'ready';
    let score = 0;
    let timeLeft = 60;
    let timerInterval = null;
    
    let feedback = null;
    let isCheckingAnswer = false;


    async function fetchProblem() {
        try {
            const res = await fetch('/api/pure-pot-odds-get-problem');
            if (!res.ok) throw new Error(`Server error: ${res.status}`);
            return await res.json();
        } catch (e) {
            error = e.message;
            return null;
        }
    }

    async function refillStack() {
        if (isCheckingAnswer) return;
        isCheckingAnswer = true;
        while (problemStack.length < STACK_SIZE && !error) {
            const prob = await fetchProblem();
            if (prob) problemStack = [...problemStack, prob];
            else break;
        }
        isCheckingAnswer = false;
    }

    async function nextProblem() {
        feedback = null;
        isCheckingAnswer = false;

        if (problemStack.length > 0) {
            currentProblem = problemStack[0];
            problemStack = problemStack.slice(1);
            refillStack();
        } else {
            currentProblem = await fetchProblem();
        }
    }

    async function checkAnswer(decision) {
        if (isCheckingAnswer || !currentProblem) return;
        isCheckingAnswer = true;

        try {
            const res = await fetch('/api/pure-pot-odds-check-answer', {
                method: 'POST',
                headers: { 'Content-Type': 'application/json' },
                body: JSON.stringify({
                    problemId: currentProblem.problem_id,
                    decision: decision
                }),
            });

            if (!res.ok) throw new Error(`Server error: ${res.status}`);
            const data = await res.json();

            if (data.isCorrect) {
                score++;
            }
            feedback = data;

            setTimeout(() => {
                if (gameState === 'playing') {
                   nextProblem();
                }
            }, 1500);

        } catch (e) {
            error = e.message;
            isCheckingAnswer = false;
        }
    }

    function startGame() {
        score = 0;
        timeLeft = 60;
        gameState = 'playing';
        isCheckingAnswer = false;
        feedback = null;
        currentProblem = null;
        
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
    });
</script>

<main>
    {#if gameState === 'ready'}
        <div class="menu-box">
            <h1>Pure Pot Odds Trainer</h1>
            <p>You have 60 seconds. Given the pot size, the bet to call, and your equity, decide whether to CALL or FOLD.</p>
            <button on:click={startGame}>Start Game</button>
        </div>

    {:else if gameState === 'playing'}
        <div class="game-header">
            <span>🕒 {timeLeft}</span>
            <span class="score">🏆 {score}</span>
        </div>

        {#if currentProblem}
            <PotOddsDisplay {currentProblem} />

            <div class="action-area">
                {#if feedback}
                    <div class="feedback-box" class:correct={feedback.isCorrect} class:wrong={!feedback.isCorrect}>
                        <p class="feedback-title">{feedback.isCorrect ? 'Correct!' : 'Incorrect!'}</p>
                        <p>The correct decision was to <strong>{feedback.correctDecision ? 'CALL' : 'FOLD'}</strong>.</p>
                        <p>Pot odds required you to have > <strong>{feedback.potOdds.toFixed(1)}%</strong> equity.</p>
                    </div>
                {:else}
                    <div class="button-group">
                        <button class="call-btn" on:click={() => checkAnswer(true)} disabled={isCheckingAnswer}>
                            CALL
                        </button>
                        <button class="fold-btn" on:click={() => checkAnswer(false)} disabled={isCheckingAnswer}>
                            FOLD
                        </button>
                    </div>
                {/if}
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

    {#if error}
        <div class="error-box">
            <p><strong>Error:</strong> {error}</p>
            <p>Please refresh the page and check the server.</p>
        </div>
    {/if}
</main>

<style>
    :root {
        --gold: #d4af37;
        --dark-bg: #1e1e1e;
        --medium-bg: #2a2a2a;
        --border-color: #3f3f46;
        --text-light: #a1a1aa;
        --green: #10b981;
        --red: #ef4444;
    }

    main { 
        font-family: 'Inter', system-ui, sans-serif; 
        text-align: center; 
        padding: 1rem; 
        max-width: 700px;
        margin: 0 auto;
    }
    
    .menu-box { 
        margin: auto; 
        padding: 2rem; 
        background: var(--dark-bg);
        border: 1px solid var(--border-color);
        border-radius: 12px; 
        box-shadow: 0 10px 30px rgba(0, 0, 0, 0.3);
    }
    
    .menu-box h1 {
        background: linear-gradient(135deg, var(--gold) 0%, #fbbf24 100%);
        -webkit-background-clip: text;
        -webkit-text-fill-color: transparent;
        background-clip: text;
        margin-bottom: 1rem;
    }
    
    .menu-box p {
        color: var(--text-light);
        line-height: 1.6;
        margin-bottom: 2rem;
    }
    
    button { 
        padding: 1rem 2rem; 
        font-size: 1.2rem; 
        cursor: pointer; 
        border-radius: 12px; 
        border: none; 
        color: var(--dark-bg); 
        font-weight: 700;
        transition: all 0.2s ease;
        box-shadow: 0 4px 12px rgba(0, 0, 0, 0.2);
    }
    
    button:hover:not(:disabled) {
        transform: translateY(-2px);
        box-shadow: 0 6px 20px rgba(0, 0, 0, 0.3);
    }

    button:disabled {
        opacity: 0.5;
        cursor: not-allowed;
    }

    .menu-box button {
        background: linear-gradient(135deg, var(--gold) 0%, #b8941f 100%);
    }
    
    .action-area {
        margin-top: 2rem;
    }

    .button-group {
        display: flex;
        justify-content: center;
        gap: 1.5rem;
    }

    .call-btn {
        background: linear-gradient(135deg, var(--green) 0%, #059669 100%);
        color: white;
    }
    .fold-btn {
        background: linear-gradient(135deg, var(--red) 0%, #dc2626 100%);
        color: white;
    }
    
    .game-header { 
        display: flex; 
        justify-content: space-around; 
        align-items: center;
        font-size: 1.5rem; 
        font-weight: 600;
        margin-bottom: 2rem;
        padding: 1rem;
        background: rgba(30, 30, 30, 0.8);
        border-radius: 12px;
        border: 1px solid var(--border-color);
    }
    
    .loading-state {
        padding: 2rem; color: var(--text-light);
    }
    .loading-spinner {
        width: 40px; height: 40px; margin: 1rem auto;
        border: 3px solid rgba(255, 255, 255, 0.3);
        border-top: 3px solid var(--gold);
        border-radius: 50%;
        animation: spin 1s linear infinite;
    }
    
    @keyframes spin {
        0% { transform: rotate(0deg); }
        100% { transform: rotate(360deg); }
    }

    .feedback-box {
        padding: 1.5rem;
        border-radius: 12px;
        border: 2px solid;
        max-width: 400px;
        margin: 0 auto;
        animation: fadeIn 0.3s ease-in-out;
    }

    .feedback-box.correct {
        background-color: rgba(16, 185, 129, 0.1);
        border-color: var(--green);
        color: #a7f3d0;
    }

    .feedback-box.wrong {
        background-color: rgba(239, 68, 68, 0.1);
        border-color: var(--red);
        color: #fca5a5;
    }

    .feedback-box p { margin: 0.3rem 0; }
    .feedback-title {
        font-size: 1.4rem;
        font-weight: 700;
        margin-bottom: 0.8rem !important;
    }

    .error-box {
        margin-top: 2rem;
        padding: 1rem;
        border: 1px solid var(--red);
        background-color: rgba(239, 68, 68, 0.1);
        color: #fca5a5;
        border-radius: 8px;
    }

    @keyframes fadeIn {
        from { opacity: 0; transform: scale(0.95); }
        to { opacity: 1; transform: scale(1); }
    }
</style>