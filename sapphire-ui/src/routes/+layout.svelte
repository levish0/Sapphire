<script lang="ts">
	import './layout.css';
	import { Toaster } from '$lib/components/ui/sonner/index.js';
	import { ModeWatcher, setMode } from 'mode-watcher';
	import favicon from '$lib/assets/favicon.svg';

	let { children } = $props();
</script>

<svelte:head><link rel="icon" href={favicon} /></svelte:head>
<ModeWatcher defaultMode="system" />
<Toaster />
{@render children()}
