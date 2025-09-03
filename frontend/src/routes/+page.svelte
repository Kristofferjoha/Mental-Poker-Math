<script lang="ts">
	const games = [
		{
			slug: 'pot-odds-ev',
			title: 'Pot Odds + EQ Decision',
			description: 'Heads-up all-in scenario. Decide to call or fold based on pot odds vs. equity.'
		},
		{
			slug: 'pure-equity',
			title: 'Equity Intuition Training',
			description: "See a heads-up all-in scenario and estimate your hand's equity."
		},
		{
			slug: 'pure-pot-odds',
			title: 'Pure Pot Odds Drill',
			description: 'No cards, just numbers. Quickly decide to call or fold based on pot odds vs. equity.'
		}
	];
</script>

<svelte:head>
	<title>Mental Poker Math</title>
	<meta name="description" content="A collection of minigames to sharpen your poker math intuition." />
</svelte:head>

<main>
	<h1>Mental Poker Math</h1>
	<p class="subtitle">Sharpen your poker intuition and numerical precision with these targeted drills.</p>

	<nav class="game-menu">
		{#each games as game}
			<a href="/{game.slug}" class="game-link">
				<h2>{game.title}</h2>
				<p>{game.description}</p>
			</a>
		{/each}
	</nav>
</main>

<style>
	.subtitle {
		font-size: 1.2rem;
		max-width: 600px;
		margin: 0 auto 3rem auto;
		color: var(--text-secondary);
	}

	.game-menu {
		display: grid;
		gap: 1.5rem;
	}

	.game-link {
		display: block;
		background-color: var(--content-bg);
		border: 1px solid var(--border-color);
		border-radius: var(--border-radius-md);
		padding: 1.5rem;
		text-decoration: none;
		color: var(--text-primary);
		transition: transform 0.2s ease, border-color 0.2s ease;
		text-align: left;
	}

	.game-link:hover {
		transform: translateY(-4px);
		border-color: var(--gold);
	}

	.game-link h2 {
		margin: 0 0 0.5rem 0;
		color: var(--text-primary);
		font-size: 1.25rem;
	}

	.game-link p {
		margin: 0;
		color: var(--text-secondary);
		font-size: 1rem;
		line-height: 1.5;
	}
</style>