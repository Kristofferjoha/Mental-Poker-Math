<script lang="ts">
	import type { Card as CardType, PotEquityProblem, PureEquityProblem } from '$lib/types';
	import Card from '$lib/components/deck.svelte';

	export let problem: PotEquityProblem | PureEquityProblem | null = null;

	function padHand(cards: CardType[] = [], length: number): (CardType | null)[] {
		const hand: (CardType | null)[] = [...cards];
		while (hand.length < length) {
			hand.push(null);
		}
		return hand;
	}

	let playerHandDisplay: (CardType | null)[] = [];
	let opponentHandDisplay: (CardType | null)[] = [];
	let boardDisplay: (CardType | null)[] = [];

	$: {
		if (problem) {
			playerHandDisplay = padHand(problem.player_hand, 2);
			opponentHandDisplay = padHand(problem.opponent_hand, 2);
			boardDisplay = padHand(problem.board, 5);
		} else {
			playerHandDisplay = padHand([], 2);
			opponentHandDisplay = padHand([], 2);
			boardDisplay = padHand([], 5);
		}
	}

	function isPotEquityProblem(p: any): p is PotEquityProblem {
		return p && p.pot_size !== undefined;
	}
	
</script>

<div class="poker-table-wrapper">
	{#if problem}
		<div class="poker-table">
			<div class="table-surface">
				<div class="player-area opponent">
					<div class="player-label">
						<span class="player-name">Opponent</span>
						{#if isPotEquityProblem(problem)}
							<div class="bet-chip">
								<span class="bet-amount">{problem.bet_to_call}</span>
								<span class="bet-label">All-in</span>
							</div>
						{/if}
					</div>
					<div class="hand-container">
						{#each opponentHandDisplay as card, i (card ? `${card.rank}-${card.suit}` : `opp-ph-${i}`)}
							{#if card}
								<Card rank={card.rank} suit={card.suit} />
							{:else}
								<div class="card-placeholder"></div>
							{/if}
						{/each}
					</div>
				</div>

				<div class="community-area">
					{#if isPotEquityProblem(problem)}
						<div class="pot-info">
							<div class="pot-chip">
								<span class="pot-amount">{problem.pot_size}</span>
								<span class="pot-label">Pre All-In Pot</span>
							</div>
						</div>
					{/if}

					<div class="board-container">
						{#each boardDisplay as card, i (card ? `${card.rank}-${card.suit}` : `board-ph-${i}`)}
							{#if card}
								<Card rank={card.rank} suit={card.suit} />
							{:else}
								<div class="card-placeholder"></div>
							{/if}
						{/each}
					</div>
				</div>

				<div class="player-area hero">
					<div class="hand-container">
						{#each playerHandDisplay as card, i (card ? `${card.rank}-${card.suit}` : `player-ph-${i}`)}
							{#if card}
								<Card rank={card.rank} suit={card.suit} />
							{:else}
								<div class="card-placeholder"></div>
							{/if}
						{/each}
					</div>
					<div class="player-label">
						<span class="player-name">Your Hand</span>
						{#if isPotEquityProblem(problem)}
							<div class="info-chip green">
								<span>{problem.bet_to_call} to call</span>
							</div>
						{:else}
							<div class="info-chip blue">
								<span>What's your equity?</span>
							</div>
						{/if}
					</div>
				</div>
			</div>
		</div>
	{:else}
		<div class="poker-table">
			</div>
	{/if}
</div>

<style>
	.poker-table-wrapper {
		display: flex;
		flex-direction: column;
		align-items: center;
		gap: 1.5rem;
		max-width: 700px;
		margin: 0 auto;
	}
	.poker-table {
		width: 100%;
	}
	.table-surface {
		background: linear-gradient(135deg, #1a5f3f 0%, #0d4d32 100%);
		border-radius: 60px;
		width: 100%;
		min-height: 400px;
		border: 3px solid #2d4a3e;
		box-shadow: 0 10px 30px rgba(0, 0, 0, 0.3), inset 0 1px 0 rgba(255, 255, 255, 0.1);
		display: flex;
		flex-direction: column;
		justify-content: space-around;
		align-items: center;
		padding: 1rem 1.5rem;
		position: relative;
	}
	.community-area {
		display: flex;
		flex-direction: column;
		gap: 1rem;
		align-items: center;
		margin: 0.5rem 0;
	}
	.player-area {
		display: flex;
		flex-direction: column;
		align-items: center;
		gap: 1rem;
		width: 100%;
	}
	.player-label {
		display: flex;
		flex-direction: column;
		align-items: center;
		gap: 0.5rem;
	}
	.player-name {
		font-size: 0.9rem;
		font-weight: 600;
		color: var(--text-secondary);
		text-transform: uppercase;
	}
	.hand-container {
		display: flex;
		gap: 0.5rem;
		min-height: 80px;
		align-items: center;
	}
	.board-container {
		display: flex;
		gap: 0.5rem;
		background: rgba(0, 0, 0, 0.2);
		padding: 1rem;
		border-radius: var(--border-radius-lg);
		border: 1px solid rgba(255, 255, 255, 0.1);
	}
	.card-placeholder {
    box-sizing: border-box;
    width: 54px;
    height: 80px;
    border-radius: 6px;
    border: 2px dashed rgba(255, 255, 255, 0.2);
    background: rgba(255, 255, 255, 0.05);
}
	.info-chip {
		border: 1px solid;
		background: color-mix(in srgb, currentColor 20%, transparent);
		border-radius: 16px;
		padding: 0.3rem 0.8rem;
		font-size: 0.8rem;
		font-weight: 500;
	}
	.info-chip.green { color: var(--green); }
	.info-chip.blue { color: var(--blue); }

	.pot-info {
		display: flex;
		justify-content: center;
	}
	.pot-chip, .bet-chip {
		border: 2px solid;
		border-radius: 20px;
		padding: 0.5rem 1rem;
		display: flex;
		flex-direction: column;
		align-items: center;
		box-shadow: 0 4px 8px rgba(0, 0, 0, 0.3);
		font-weight: 700;
	}
	.pot-chip {
		background: linear-gradient(135deg, var(--gold) 0%, var(--gold-hover) 100%);
		border-color: #f4e4a6;
		color: var(--content-bg);
	}
	.bet-chip {
		background: linear-gradient(135deg, var(--red) 0%, #dc2626 100%);
		border-color: #fca5a5;
		color: white;
	}
	.pot-amount, .bet-amount {
		font-size: 1.1rem;
		line-height: 1;
	}
	.pot-label, .bet-label {
		font-size: 0.7rem;
		text-transform: uppercase;
		letter-spacing: 0.5px;
		opacity: 0.9;
	}
</style>