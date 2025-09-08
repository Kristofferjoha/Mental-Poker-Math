<script lang="ts">
	import type { HistoryItem, PotEquityCheckResponse, PureEquityCheckResponse } from '$lib/types';
	import Card from '$lib/components/deck.svelte';

	export let item: HistoryItem;
	export let index: number;

	function isPotEquityResponse(response: HistoryItem['response']): response is PotEquityCheckResponse {
		return 'correctDecision' in response;
	}
	function isPureEquityResponse(response: HistoryItem['response']): response is PureEquityCheckResponse {
		return !('correctDecision' in response);
	}

	$: isCorrect = item.response.userGuessIsCorrect;
	$: yourDecisionText =
		item.game === 'pot-odds-equity' && item.userDecision !== undefined
			? item.userDecision
				? 'Called'
				: 'Folded'
			: item.game === 'pure-equity' && item.userGuess !== undefined
			? `Guessed ${(item.userGuess || 0).toFixed(1)}%`
			: '';
</script>

<div class="history-item" class:correct={isCorrect} class:wrong={!isCorrect}>
	<div class="header">
		<span class="hand-number">Hand #{index + 1}</span>
		<div class="decision">
			{yourDecisionText}
			{#if !isCorrect && item.game === 'pot-odds-equity' && isPotEquityResponse(item.response)}
				<span class="correct-decision">(Correct: {item.response.expectedDecision ? 'Call' : 'Fold'})</span>
			{:else if !isCorrect && item.game === 'pure-equity' && isPureEquityResponse(item.response)}
				<span class="correct-decision">(Actual: {(item.response.playerEquity).toFixed(1)}%)</span>
			{/if}
		</div>
	</div>

	<div class="cards-layout">
		<div class="hand-group">
			<div class="label">You</div>
			<div class="cards">
				{#each item.problem.player_hand as card}
					<Card rank={card.rank} suit={card.suit} />
				{/each}
			</div>
		</div>
		<div class="hand-group">
			<div class="label">Opponent</div>
			<div class="cards">
				{#each item.problem.opponent_hand as card}
					<Card rank={card.rank} suit={card.suit} />
				{/each}
			</div>
		</div>
	</div>

	<div class="board">
		<div class="label">Board</div>
		<div class="cards">
			{#each item.problem.board as card}
				<Card rank={card.rank} suit={card.suit} />
			{/each}
		</div>
	</div>

	<div class="details">
		{#if item.game === 'pot-odds-equity' && isPotEquityResponse(item.response)}
			<span>Pot Odds: <strong>{(item.response.potOdds * 100).toFixed(1)}%</strong></span>
			<span>Your Equity: <strong>{(item.response.playerEquity * 100).toFixed(1)}%</strong></span>
		{:else if item.game === 'pure-equity' && isPureEquityResponse(item.response)}
			<span>Your Equity: <strong>{(item.response.playerEquity).toFixed(1)}%</strong></span>
		{/if}
	</div>
</div>

<style>
	:global(.history-item .card-image) {
		height: 80px;
		width: auto;
		margin: 0;
	}
	.history-item {
		background-color: var(--content-bg);
		border: 1px solid var(--border-color);
		border-left-width: 5px;
		border-radius: var(--border-radius-md);
		padding: 0.5rem;
		margin-bottom: 1rem;
	}
	.history-item.correct { border-left-color: var(--green); }
	.history-item.wrong { border-left-color: var(--red); }

	.header { display: flex; justify-content: space-between; align-items: center; margin-bottom: 1rem; }
	.hand-number { font-weight: 600; }
	.decision { font-size: 0.9rem; color: var(--text-secondary); }
	.correct-decision { margin-left: 0.5rem; font-weight: 600; color: var(--gold); }
	
	.cards-layout { display: flex; justify-content: space-around; gap: 0.5rem; margin-bottom: 1rem; }
	.hand-group, .board { text-align: center; }
	.label { font-size: 0.8rem; text-transform: uppercase; color: var(--text-secondary); margin-bottom: 0.5rem; }
	.cards { display: flex; gap: 0.25rem; justify-content: center; }

	.details {
		display: flex;
		justify-content: space-between;
		font-size: 0.9rem;
		border-top: 1px solid var(--border-color);
		padding-top: 0.5rem;
		margin-top: 1rem;
	}
</style>