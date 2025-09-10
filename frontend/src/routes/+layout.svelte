<script>
	import '../lib/styles/global.css';
	import { injectAnalytics } from '@vercel/analytics/sveltekit';
	injectAnalytics();
</script>

<slot />