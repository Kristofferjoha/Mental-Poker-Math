<script lang="ts">
	import { createEventDispatcher, onDestroy } from 'svelte';

	export let title: string;
	export let description: string;
	export let score: number;
	export let gameDuration: number = 60; // defaults at 60

	let gameState: 'ready' | 'playing' | 'finished' = 'ready';
	let timeLeft = gameDuration;
	let timerInterval: ReturnType<typeof setInterval>;

	const dispatch = createEventDispatcher<{ start: void; end: void }>();

	function startGame() {
		timeLeft = gameDuration;
		score = 0;
		gameState = 'playing';
		dispatch('start'); 

		timerInterval = setInterval(() => {
			timeLeft--;
			if (timeLeft <= 0) {
				endGame();
			}
		}, 1000);
	}

	function endGame() {
		clearInterval(timerInterval);
		gameState = 'finished';
		dispatch('end');
	}

	onDestroy(() => {
		clearInterval(timerInterval);
	});
</script>

<div class="game-shell">
	{#if gameState === 'ready'}
		<div class="menu-box">
			<h1>{title}</h1>
			<p>{description}</p>

			<div class="options-area">
				<slot name="options" />
			</div>

			<button on:click={startGame}>Start Game</button>
		</div>
	{:else if gameState === 'playing'}
		<header class="game-header">
			<span>Timer: {timeLeft}</span>
			<span class="score">Score: {score}</span>
		</header>

		<div class="game-content">
			<slot />
		</div>
	{:else if gameState === 'finished'}
		<div class="menu-box">
			<h2>Final Score:</h2>
			<p class="final-score">{score}</p>
			
			<div class="button-group">
				<button on:click={startGame}>Play Again</button>
				<a href="/" class="button-secondary">Main Menu</a>
			</div>
			<div class="results-area">
				<slot name="results" />
			</div>
		</div>
	{/if}
</div>

<style>
	.game-shell {
		max-width: 800px;
		margin: auto;
		padding: 1rem;
	}

	.game-header {
		display: flex;
		justify-content: space-around;
		align-items: center;
		font-size: 1.5rem;
		font-weight: 600;
		margin-bottom: 1rem;
		padding: 1rem;
		background-color: var(--content-bg);
		border-radius: var(--border-radius-lg);
		border: 1px solid var(--border-color);
	}

	.menu-box {
		margin: auto;
		padding: 1.rem;
		background: var(--content-bg);
		border: 1px solid var(--border-color);
		border-radius: var(--border-radius-lg);
		box-shadow: 0 10px 30px rgba(0, 0, 0, 0.2);
		text-align: center;
	}

	.menu-box p {
		max-width: 600px;
		margin: 0 auto .5rem auto;
	}

	.options-area {
		margin-bottom: 2.5rem;
		min-height: 2rem;
	}

	.final-score {
		font-size: 3.5rem;
		font-weight: 700;
		margin: 1rem 0 2rem 0;
		color: var(--gold);
	}

	.results-area {
		margin-top: 1.5rem;
		text-align: left;
	}
	.button-group {
		display: flex;
		justify-content: center;
		gap: 1rem;
		margin-bottom: 1rem;
	}
	.button-secondary {
    display: inline-block;
    padding: 0.75rem 1.5rem;
    border: 1px solid var(--border-color);
    border-radius: var(--border-radius-md);
    text-decoration: none;
    font-weight: 600;
    color: var(--text-secondary);
    background-color: transparent;
    transition: background-color 0.2s, border-color 0.2s;
}

.button-secondary:hover {
    background-color: var(--content-bg-alt);
    border-color: var(--text-secondary);
}
</style>