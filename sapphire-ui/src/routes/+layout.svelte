<script lang="ts">
	import './layout.css';
	import { Toaster } from '$lib/components/ui/sonner/index.js';
	import { ModeWatcher } from 'mode-watcher';
	import favicon from '$lib/assets/favicon.svg';

	import { Icon, MagnifyingGlass } from 'svelte-hero-icons';

	import LeftSidebar from '$lib/components/layout/left-sidebar.svelte';
	import RightSidebar from '$lib/components/layout/right-sidebar.svelte';

	let { children } = $props();

	const timelineTabs = [
		{ label: 'For you', active: true },
		{ label: 'Following', active: false }
	];
</script>

<svelte:head><link rel="icon" href={favicon} /></svelte:head>
<ModeWatcher defaultMode="dark" />
<Toaster />

<div class="min-h-dvh bg-background text-foreground">
	<div class="mx-auto flex min-h-dvh w-full max-w-6xl px-3 md:px-5 xl:px-6">
		<LeftSidebar />

		<div
			class="grid min-w-0 flex-1 grid-cols-1 grid-rows-[4rem_1fr] xl:grid-cols-[minmax(0,1fr)_22rem]"
		>
			<div
				class="sticky top-0 z-30 col-start-1 row-start-1 border-r border-b border-border bg-background/95 backdrop-blur"
			>
				<div class="grid h-16 w-full grid-cols-2 px-5 sm:px-6">
					{#each timelineTabs as tab}
						<button
							class={`relative flex items-center justify-center text-sm font-semibold transition-colors ${
								tab.active
									? 'text-foreground'
									: 'text-muted-foreground hover:bg-muted/40 hover:text-foreground'
							}`}
						>
							{tab.label}
							{#if tab.active}
								<span class="absolute bottom-0 h-1 w-16 rounded-full bg-primary"></span>
							{/if}
						</button>
					{/each}
				</div>
			</div>

			<main class="col-start-1 row-start-2 min-w-0 border-r border-border" data-layout="main">
				{@render children()}
			</main>

			<div class="sticky top-0 z-30 col-start-2 row-start-1 hidden bg-background px-6 py-2 xl:flex">
				<div class="flex h-16 w-full items-center">
					<div class="relative w-full">
						<Icon
							src={MagnifyingGlass}
							solid
							size="16"
							class="pointer-events-none absolute top-1/2 left-4 -translate-y-1/2 text-muted-foreground"
						/>
						<input
							type="text"
							placeholder="Search"
							class="h-12 w-full rounded-full border border-border bg-transparent pr-4 pl-11 text-sm transition-colors outline-none placeholder:text-muted-foreground focus:bg-muted/20"
						/>
					</div>
				</div>
			</div>

			<aside class="hidden px-6 py-4 xl:block" data-layout="right_sidebar">
				<RightSidebar />
			</aside>
		</div>
	</div>
</div>
