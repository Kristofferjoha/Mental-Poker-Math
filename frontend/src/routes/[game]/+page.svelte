<script lang="ts">
	import { page } from '$app/stores';
	import { tick } from 'svelte';
	import type {
		AnyProblem,
		AnyFeedback,
		PurePotOddsProblem,
		PurePotOddsCheckResponse,
		PureEquityProblem,
		PureEquityCheckResponse,
		PotEquityProblem,
		PotEquityCheckResponse,
		HistoryItem,
		OptionConfig
	} from '$lib/types';

	import GameShell from '$lib/components/GameShell.svelte';
	import PotOddsDisplay from '$lib/components/PotOddsDisplay.svelte';
	import PokerTable from '$lib/components/PokerTable.svelte';
	import SessionReviewItem from '$lib/components/SessionReviewItem.svelte';
	import OptionsMenu from '$lib/components/OptionsMenu.svelte';

	// Struct for game configurations
	type GameConfig = {
		title: string;
		description: string;
		api: { getProblem: string; checkAnswer: string };
		options: OptionConfig[];
	};

	// Dynamic game selection based on URL
	const { game } = $page.params;

	const gameConfigs = new Map<string, GameConfig>([
		[
			'pure-pot-odds',
			{
				title: 'Pure Pot Odds Drill',
				description: 'No cards, just numbers. Quickly decide to call or fold based on pot odds vs. equity.',
				api: {
					getProblem: '/api/pure-pot-odds-get-problem',
					checkAnswer: '/api/pure-pot-odds-check-answer'
				},
				options: [
					{
						id: 'duration',
						label: 'Game Duration',
						type: 'select',
						defaultValue: 60,
						choices: [
							{ label: '60 Seconds', value: 60 },
							{ label: '90 Seconds', value: 90 },
							{ label: '120 Seconds', value: 120 }
						]
					},
					{
						id: 'allowOverbets',
						label: 'Allow Overbets',
						type: 'checkbox',
						defaultValue: true
					}
				]
			}
		],
		[
			'pure-equity',
			{
				title: 'Pure Equity Trainer',
				description: "See a heads-up all-in scenario and estimate your hand's raw equity.",
				api: {
					getProblem: '/api/pure-equity-get-problem',
					checkAnswer: '/api/pure-equity-check-answer'
				},
				options: [
					{
						id: 'duration',
						label: 'Game Duration',
						type: 'select',
						defaultValue: 60,
						choices: [
							{ label: '60 Seconds', value: 60 },
							{ label: '90 Seconds', value: 90 },
							{ label: '120 Seconds', value: 120 }
						]
					},
					{
						id: 'streets',
						label: 'Streets',
						type: 'checkbox-group',
						defaultValue: ['pre-flop', 'flop', 'turn', 'river'],
						choices: [
							{ label: 'Pre-flop', value: 'pre-flop' },
							{ label: 'Flop', value: 'flop' },
							{ label: 'Turn', value: 'turn' },
							{ label: 'River', value: 'river' }
						]
					},
					{
						id: 'tolerance',
						label: 'Guess Tolerance (±%)',
						type: 'select',
						defaultValue: 5,
						choices: [
							{ label: '2%', value: 2 },
							{ label: '5%', value: 5 },
							{ label: '10%', value: 10 }
						]
					},
					{
						id: 'directionalHints',
						label: 'Show Directional Hints',
						type: 'checkbox',
						defaultValue: false
					}
				]
			}
		],
		[
			'pot-odds-equity',
			{
				title: 'Pot Odds + EV Decision',
				description: 'Cards on the table. Face an all-in and decide if calling is profitable.',
				api: {
					getProblem: '/api/pot-equity-get-problem',
					checkAnswer: '/api/pot-equity-check-answer'
				},
				options: [
					{
						id: 'duration',
						label: 'Game Duration',
						type: 'select',
						defaultValue: 60,
						choices: [
							{ label: '60 Seconds', value: 60 },
							{ label: '90 Seconds', value: 90 },
							{ label: '120 Seconds', value: 120 }
						]
					},
					{
						id: 'streets',
						label: 'Streets',
						type: 'checkbox-group',
						defaultValue: ['pre-flop', 'flop', 'turn', 'river'],
						choices: [
							{ label: 'Pre-flop', value: 'pre-flop' },
							{ label: 'Flop', value: 'flop' },
							{ label: 'Turn', value: 'turn' },
							{ label: 'River', value: 'river' }
						]
					}
				]
			}
		]
	]);
	const config = game ? gameConfigs.get(game) : undefined;
	let selectedOptions: Record<string, any> = {};

	if (config?.options) {
		for (const option of config.options) {
			selectedOptions[option.id] = option.defaultValue;
		}
	}

	let score = 0;
	let currentProblem: AnyProblem | null = null;
	let feedback: AnyFeedback | null = null;
	let isCheckingAnswer = false;
	let error: string | null = null;
	let directionalHint: 'Higher' | 'Lower' | 'Not-Active' = 'Not-Active';

	let equityGuess = '';
	let equityInput: HTMLInputElement;
	let feedbackClass = '';
	let sessionHistory: HistoryItem[] = [];

	function isPurePotOddsProblem(p: AnyProblem | null): p is PurePotOddsProblem {
		return !!p && 'equity' in p;
	}
	function isPureEquityProblem(p: AnyProblem | null): p is PureEquityProblem {
		return !!p && 'player_hand' in p && !('pot_size' in p);
	}
	function isPotEquityProblem(p: AnyProblem | null): p is PotEquityProblem {
		return !!p && 'pot_size' in p && 'player_hand' in p;
	}
	function isPurePotOddsFeedback(f: AnyFeedback | null): f is PurePotOddsCheckResponse {
		return !!f && 'potOdds' in f;
	}

	function handleGameStart() {
		score = 0;
		feedback = null;
		currentProblem = null;
		sessionHistory = [];
		nextProblem();
	}

	async function nextProblem() {
		feedback = null;
		feedbackClass = '';
		equityGuess = '';
		isCheckingAnswer = false;
		directionalHint = 'Not-Active';

		try {
			if (!config) throw new Error('Game configuration not found!');

			const params = new URLSearchParams();
			const frontendOnlyOptions = ['duration'];

			for (const key in selectedOptions) {
				if (frontendOnlyOptions.includes(key)) {
					continue;
				}


				const value = selectedOptions[key];
				if (Array.isArray(value)) {
					params.append(key, value.join(','));
				} else {
					params.append(key, String(value));
				}
			}

			const queryString = params.toString();
			const url = queryString ? `${config.api.getProblem}?${queryString}` : config.api.getProblem;

			console.log('Fetching from URL:', url); // debugging (husk at fjernennenene)

			const res = await fetch(url);

			if (!res.ok) throw new Error(`Server error: ${res.status}`);
			currentProblem = await res.json();

			if (game === 'pure-equity') {
				await tick();
				equityInput?.focus();
			}
		} catch (e: any) {
			error = e.message;
		}
	}

	function handleEquityKeyDown(event: KeyboardEvent) {
		if (event.key === 'Enter') {
			event.preventDefault();
			checkEquityAnswer();
		}
	}

	async function checkPotOddsAnswer(decision: boolean) {
		console.log('User decision (Pot Odds):', decision); // Debugging log
		if (isCheckingAnswer || !currentProblem || !config || !isPurePotOddsProblem(currentProblem)) return;
		isCheckingAnswer = true;
		try {
			const res = await fetch(config.api.checkAnswer, {
				method: 'POST',
				headers: { 'Content-Type': 'application/json' },
				body: JSON.stringify({ problemId: currentProblem.problem_id, user_decision: decision })
			});
			if (!res.ok) throw new Error(`Server error: ${res.status}`);
			const data: PurePotOddsCheckResponse = await res.json();
			if (data.userGuessIsCorrect) score++

			feedbackClass = data.userGuessIsCorrect ? 'correct-flash' : 'incorrect-flash';
			setTimeout(() => {
				nextProblem();
			}, 100);
		} catch (e: any) {
			error = e.message;
			isCheckingAnswer = false;
		}
	}

	async function checkEquityAnswer() {
		if (isCheckingAnswer || !currentProblem || !config || !isPureEquityProblem(currentProblem)) return;

		const guessVal = parseFloat(equityGuess);
		if (isNaN(guessVal)) return;

		isCheckingAnswer = true;
		try {
			const res = await fetch(config.api.checkAnswer, {
				method: 'POST',
				headers: { 'Content-Type': 'application/json' },
				body: JSON.stringify({ problemId: currentProblem.problem_id, guess_value: guessVal })
			});
			if (!res.ok) throw new Error(`Server error: ${res.status}`);
			const data: PureEquityCheckResponse = await res.json();

			directionalHint = data.directionalHint;

			sessionHistory = [
				...sessionHistory,
				{
					game: 'pure-equity',
					problem: currentProblem,
					response: data,
					userGuess: guessVal
				}
			];

			if (data.userGuessIsCorrect) {
				console.log('Correct answer!');
				score++;
				feedbackClass = 'correct';
				setTimeout(nextProblem, 400);
			} else {
				console.log('Incorrect answer.');
				feedbackClass = 'incorrect';
				isCheckingAnswer = false;
				await tick();
				equityInput?.select();
			}
		} catch (e: any) {
			error = e.message;
			isCheckingAnswer = false;
		}
	}

	async function checkPotEquityAnswer(decision: boolean) {
		if (isCheckingAnswer || !currentProblem || !config || !isPotEquityProblem(currentProblem)) return;
		isCheckingAnswer = true;
		try {
			const res = await fetch(config.api.checkAnswer, {
				method: 'POST',
				headers: { 'Content-Type': 'application/json' },
				body: JSON.stringify({ problemId: currentProblem.problem_id, decision })
			});
			if (!res.ok) throw new Error(`Server error: ${res.status}`);
			const data: PotEquityCheckResponse = await res.json();

			console.log('Pot Equity Answer Response:', data); // Debugging log
			if (data.userGuessIsCorrect) {
				console.log('Correct answer!');
				score++;
			}

			sessionHistory = [
				...sessionHistory,
				{
					problem: currentProblem,
					response: data,
					game: 'pot-odds-equity',
					userDecision: decision
				}
			];
			nextProblem();
		} catch (e: any) {
			error = e.message;
		} finally {
			isCheckingAnswer = false;
		}
	}

	async function handleKeyPress(event: KeyboardEvent) {
        if (event.target instanceof HTMLInputElement) {
            return;
        }

        const key = event.key.toLowerCase();

        if (key !== 'c' && key !== 'f') {
            return;
        }

        event.preventDefault();

        const decision = key === 'c';

        if (game === 'pure-pot-odds') {
            checkPotOddsAnswer(decision);
        } else if (game === 'pot-odds-equity') {
            checkPotEquityAnswer(decision);
        }
    }

</script>

<svelte:window on:keydown={handleKeyPress} />

{#if config}
	
	<GameShell
		bind:score
		title={config.title}
		description={config.description}
		on:start={handleGameStart}
		gameDuration={selectedOptions.duration}
	>
		<!-- Options Panel UI -->
		<div slot="options">
			{#if config.options}
				<OptionsMenu optionDefinitions={config.options} bind:selectedOptions />
			{/if}
		</div>

		<!-- Main Game UI -->
		{#if currentProblem}
			<!-- Pure Pot Odds Game -->
			{#if game === 'pure-pot-odds' && isPurePotOddsProblem(currentProblem)}
				<PotOddsDisplay currentProblem={currentProblem} />
				<div class="action-area">
					{#if feedback && isPurePotOddsFeedback(feedback)}
						<div class="feedback-box" class:correct={feedback.userGuessIsCorrect} class:wrong={!feedback.userGuessIsCorrect}>
							<p class="feedback-title">{feedback.userGuessIsCorrect ? 'Correct!' : 'Incorrect!'}</p>
							<p>
								Correct decision: <strong>{feedback.expectedDecision ? 'CALL' : 'FOLD'}</strong>.
							</p>
							<p>Required equity: <strong>{feedback.potOdds.toFixed(1)}%</strong></p>
						</div>
					{:else}
						<div class="button-group">
							<button class="call-btn" on:click={() => checkPotOddsAnswer(true)} disabled={isCheckingAnswer}>CALL <kbd>C</kbd></button>
							<button class="fold-btn" on:click={() => checkPotOddsAnswer(false)} disabled={isCheckingAnswer}>FOLD <kbd>F</kbd></button>
						</div>
					{/if}
				</div>

			<!-- Pure Equity Game -->
			{:else if game === 'pure-equity' && isPureEquityProblem(currentProblem)}
				<PokerTable problem={currentProblem} />
				<div class="action-area">

					<div class="input-wrapper">
						<input
							type="number"
							step="0.1"
							bind:value={equityGuess}
							placeholder="Your equity %"
							bind:this={equityInput}
							class="equity-input"
							class:correct={feedbackClass === 'correct'}
							class:incorrect={feedbackClass === 'incorrect'} disabled={isCheckingAnswer}
							on:keydown={handleEquityKeyDown}
						/>

						<div class="hint-arrow">
							{#if directionalHint === 'Higher'}
								<span class="arrow-up" title="Higher!">▲</span>
							{:else if directionalHint === 'Lower'}
								<span class="arrow-down" title="Lower!">▼</span>
							{/if}
						</div>
					</div>
					<div class="input-hint">Press Enter</div>
				</div>

			<!-- Pot Odds + EV Game -->
			{:else if game === 'pot-odds-equity' && isPotEquityProblem(currentProblem)}
				<PokerTable problem={currentProblem} />
				<div class="action-area">
					<div class="button-group">
						<button class="call-btn" on:click={() => checkPotEquityAnswer(true)} disabled={isCheckingAnswer}>CALL <kbd>C</kbd></button>
						<button class="fold-btn" on:click={() => checkPotEquityAnswer(false)} disabled={isCheckingAnswer}>FOLD <kbd>F</kbd></button>
					</div>
				</div>
			{/if}
		{:else if error}
			<div class="error-box"><p><strong>Error:</strong> {error}</p></div>
		{:else}
			<div class="loading-state">
				<div class="loading-spinner"></div>
				<p>Loading problem...</p>
			</div>
		{/if}

		<!-- Post-Game Results -->
		<div slot="results">
			{#if (game === 'pot-odds-equity' || game === 'pure-equity') && sessionHistory.length > 0}
				<div class="history-container">
					<h3>Hand History</h3>
					<div class="history-scroll-area">
						{#each sessionHistory as item, index}
							<SessionReviewItem {item} {index} />
						{/each}
					</div>
				</div>
			{/if}
		</div>
	</GameShell>
{:else}
	<main>
		<h1>404 - Game Not Found</h1>
		<p>Sorry, the game "{game}" does not exist.</p>
		<a href="/" class="menu-link">Return to Main Menu</a>
	</main>
{/if}

<style>
	.action-area {
		margin-top: 1rem;
		text-align: center;
	}
	.button-group button {	
        display: inline-flex;
        align-items: center;
        justify-content: center;
        gap: 0.6rem;
    }

    .button-group button kbd {
        font-family: monospace;
        font-size: 0.9em;
        padding: 0.1em 0.5em;
        border-radius: var(--border-radius-sm, 4px);
        border: 1px solid rgba(255, 255, 255, 0.3);
        background-color: rgba(255, 255, 255, 0.1);
        box-shadow: 0 2px 0 rgba(0, 0, 0, 0.2);
        line-height: 1;
        position: relative;
        top: -1px;
    }
	.call-btn {
		background: var(--green);
		color: white;
	}
	.fold-btn {
		background: var(--red);
		color: white;
	}
	.feedback-box {
		padding: 1.5rem;
		border-radius: var(--border-radius-lg);
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
	.feedback-box p {
		margin: 0.3rem 0;
		color: inherit;
	}
	.feedback-title {
		font-size: 1.4rem;
		font-weight: 700;
		margin-bottom: 0.8rem !important;
	}
	.loading-state {
		padding: 2rem;
		color: var(--text-secondary);
	}
	.loading-spinner {
		width: 40px;
		height: 40px;
		margin: 1rem auto;
		border: 3px solid rgba(255, 255, 255, 0.2);
		border-top: 3px solid var(--gold);
		border-radius: 50%;
		animation: spin 1s linear infinite;
	}
	.error-box {
		margin-top: 2rem;
		padding: 1rem;
		border: 1px solid var(--red);
		background-color: rgba(239, 68, 68, 0.1);
		color: #fca5a5;
		border-radius: var(--border-radius-md);
	}
	.menu-link {
		color: var(--gold);
		text-decoration: none;
		margin-top: 1rem;
		display: inline-block;
	}
	.menu-link:hover {
		text-decoration: underline;
	}
	.equity-input {
		padding: 1rem 1.5rem;
		font-size: 1.3rem;
		width: 240px;
		text-align: center;
		border: 2px solid var(--border-color);
		border-radius: var(--border-radius-lg);
		background: var(--content-bg);
		color: var(--text-primary);
		font-family: inherit;
		font-weight: 500;
		transition: all 0.2s ease;
		-moz-appearance: textfield;
		appearance: textfield;
	}
	.equity-input::-webkit-outer-spin-button,
	.equity-input::-webkit-inner-spin-button {
		-webkit-appearance: none;
		margin: 0;
	}
	.equity-input:focus {
		outline: none;
		border-color: var(--gold);
		box-shadow: 0 0 0 3px color-mix(in srgb, var(--gold) 20%, transparent);
	}
	.equity-input.correct {
		border-color: var(--green);
		animation: flash-green 0.3s ease;
	}
	.input-hint {
		font-size: 0.875rem;
		color: var(--text-secondary);
		margin-top: 0.5rem;
	}
	.history-container h3 {
		text-align: left;
		margin-top: 0;
		border-bottom: 1px solid var(--border-color);
		padding-bottom: 0.5rem;
	}
	.history-scroll-area {
		max-height: 50vh;
		overflow-y: auto;
		padding-right: 0.5rem;
	}

	@keyframes spin { 0% { transform: rotate(0deg); } 100% { transform: rotate(360deg); } }
	@keyframes fadeIn { from { opacity: 0; transform: scale(0.95); } to { opacity: 1; transform: scale(1); } }
	@keyframes flash-green {
		0%, 100% { box-shadow: none; }
		50% { box-shadow: 0 0 0 4px color-mix(in srgb, var(--green) 30%, transparent); }
	}
	.input-wrapper {
		position: relative;
		display: inline-block;
	}

	.hint-arrow {
		position: absolute;
		top: 50%;
		right: 1rem;
		transform: translateY(-50%);
		font-size: 1.5rem;
		pointer-events: none;
		animation: fadeIn 0.3s;
	}

	.arrow-up {
		color: var(--green);
	}

	.arrow-down {
		color: var(--red);
	}

	.equity-input.incorrect {
		border-color: var(--red);
		animation: flash-red 0.4s ease;
	}

	@keyframes flash-red {
		0%, 100% { box-shadow: none; }
		50% { box-shadow: 0 0 0 4px color-mix(in srgb, var(--red) 30%, transparent); }
	}
</style>