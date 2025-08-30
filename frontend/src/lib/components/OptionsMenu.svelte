<script lang="ts">
	import { onMount } from 'svelte';
	import type { OptionConfig } from '$lib/types';

	export let optionDefinitions: OptionConfig[] = [];
	export let selectedOptions: Record<string, any> = {};

	onMount(() => {
		const defaults: Record<string, any> = {};
		for (const option of optionDefinitions) {
			defaults[option.id] = option.defaultValue;
		}
		selectedOptions = defaults;
	});
</script>

<div class="options-container">
	{#each optionDefinitions as option (option.id)}
		<div class="option-row">
			<span class="label">{option.label}</span>

			<div class="control">
				{#if option.type === 'select'}
					<select bind:value={selectedOptions[option.id]}>
						{#if option.choices}
							{#each option.choices as choice (choice.value)}
								<option value={choice.value}>{choice.label}</option>
							{/each}
						{/if}
					</select>

				{:else if option.type === 'checkbox'}
					<label class="switch">
						<input type="checkbox" bind:checked={selectedOptions[option.id]} />
						<span class="slider"></span>
					</label>

				{:else if option.type === 'checkbox-group'}
					<div class="checkbox-group">
						{#if option.choices}
							{#each option.choices as choice (choice.value)}
								<label>
									<input
										type="checkbox"
										bind:group={selectedOptions[option.id]}
										value={choice.value}
									/>
									{choice.label}
								</label>
							{/each}
						{/if}
					</div>
				{/if}
			</div>
		</div>
	{/each}
</div>

<style>
	.options-container {
		display: flex;
		flex-direction: column;
		gap: 1.5rem;
		background-color: var(--dark-bg);
		padding: 1.5rem;
		border-radius: var(--border-radius-md);
		border: 1px solid var(--border-color);
		max-width: 450px;
		margin: 0 auto;
	}

	.option-row {
		display: flex;
		justify-content: space-between;
		align-items: center;
		gap: 1rem;
	}

	.label {
		font-weight: 500;
		color: var(--text-secondary);
		text-align: left;
	}

	select {
		background-color: var(--content-bg);
		color: var(--text-primary);
		border: 1px solid var(--border-color);
		border-radius: var(--border-radius-md);
		padding: 0.5rem 0.8rem;
		font-family: inherit;
		font-size: 0.9rem;
	}

	.checkbox-group {
		display: flex;
		gap: 1rem;
		flex-wrap: wrap;
		justify-content: flex-end;
	}
	.checkbox-group label {
		display: flex;
		align-items: center;
		gap: 0.4rem;
		font-size: 0.9rem;
		cursor: pointer;
	}
	input[type='checkbox'] {
		accent-color: var(--gold);
	}

	.switch {
		position: relative;
		display: inline-block;
		width: 44px;
		height: 24px;
	}
	.switch input {
		opacity: 0;
		width: 0;
		height: 0;
	}
	.slider {
		position: absolute;
		cursor: pointer;
		top: 0;
		left: 0;
		right: 0;
		bottom: 0;
		background-color: #ccc;
		transition: 0.4s;
		border-radius: 24px;
	}
	.slider:before {
		position: absolute;
		content: '';
		height: 18px;
		width: 18px;
		left: 3px;
		bottom: 3px;
		background-color: white;
		transition: 0.4s;
		border-radius: 50%;
	}
	input:checked + .slider {
		background-color: var(--gold);
	}
	input:checked + .slider:before {
		transform: translateX(20px);
	}
</style>