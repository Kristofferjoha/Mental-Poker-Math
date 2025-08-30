<script lang="ts">
	import type { PurePotOddsProblem } from '$lib/types';

	export let currentProblem: PurePotOddsProblem;

	const numberFormatter = new Intl.NumberFormat('en-US');

	$: potSize = numberFormatter.format(currentProblem.pot_size);
	$: betToCall = numberFormatter.format(currentProblem.bet_to_call);
	$: equity = currentProblem.equity.toFixed(1);
</script>

<div class="display-container">
	<div class="info-grid">
		<div class="info-box">
			<div class="label">Pot Size</div>
			<div class="value pot">{potSize}</div>
		</div>

		<div class="info-box">
			<div class="label">Bet to Call</div>
			<div class="value bet">{betToCall}</div>
		</div>

		<div class="info-box wide">
			<div class="label">Your Equity</div>
			<div class="value equity">{equity}%</div>
		</div>
	</div>
</div>

<style>
	.display-container {
		background: var(--content-bg);
		border: 1px solid var(--border-color);
		border-radius: var(--border-radius-lg);
		padding: 2rem;
		box-shadow: 0 10px 30px rgba(0, 0, 0, 0.2);
		max-width: 600px;
		margin: 0 auto;
	}
	.info-grid {
		display: grid;
		grid-template-columns: 1fr 1fr;
		gap: 1.5rem;
	}
	.info-box {
		background: var(--dark-bg);
		padding: 1.5rem;
		border-radius: var(--border-radius-md);
		text-align: center;
	}
	.info-box.wide {
		grid-column: 1 / -1;
	}
	.label {
		font-size: 1rem;
		font-weight: 500;
		color: var(--text-secondary);
		margin-bottom: 0.75rem;
		text-transform: uppercase;
	}
	.value {
		font-size: 2.5rem;
		font-weight: 700;
		line-height: 1.1;
	}
	.value.pot { color: var(--gold); }
	.value.bet { color: var(--red); }
	.value.equity { color: var(--blue); }

    @media (max-width: 480px) {
		.info-grid { grid-template-columns: 1fr; }
		.value { font-size: 2rem; }
	}
</style>