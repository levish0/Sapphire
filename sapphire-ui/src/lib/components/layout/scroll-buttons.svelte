<script lang="ts">
	import { ChevronUp, ChevronDown, Icon } from 'svelte-hero-icons';

	function scrollToTop() {
		window.scrollTo({ top: 0, behavior: 'smooth' });
	}

	function scrollToBottom() {
		window.scrollTo({ top: document.documentElement.scrollHeight, behavior: 'smooth' });
	}
</script>

<div class="flex flex-col gap-2">
	<button
		onclick={scrollToTop}
		aria-label="Scroll to top"
		class="flex size-10 items-center justify-center rounded-full border bg-card shadow-md transition-colors hover:bg-muted"
	>
		<Icon src={ChevronUp} solid size="20" />
	</button>
	<button
		onclick={scrollToBottom}
		aria-label="Scroll to bottom"
		class="flex size-10 items-center justify-center rounded-full border bg-card shadow-md transition-colors hover:bg-muted"
	>
		<Icon src={ChevronDown} solid size="20" />
	</button>
</div>
