<script>
	import '../lib/styles/global.css';
	import { onMount } from 'svelte';
	import { injectAnalytics } from '@vercel/analytics/sveltekit';
	import { preloadCardImages } from '$lib/utils/preloadCards.js';
	import { injectSpeedInsights } from '@vercel/speed-insights/sveltekit';
	injectAnalytics();
	injectSpeedInsights();

	onMount(() => {
        preloadCardImages();
    });
</script>

<slot />