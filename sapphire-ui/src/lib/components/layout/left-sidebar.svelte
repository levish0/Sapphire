<script lang="ts">
	import {
		Home,
		MagnifyingGlass,
		Bell,
		EnvelopeOpen,
		User,
		BookmarkSquare,
		PencilSquare,
		Cog6Tooth,
		Icon,
		type IconSource
	} from 'svelte-hero-icons';

	let expanded = $state(false);

	interface NavItem {
		icon: IconSource;
		label: string;
		href: string;
		active?: boolean;
	}

	const navItems: NavItem[] = [
		{ icon: Home, label: 'Home', href: '/', active: true },
		{ icon: MagnifyingGlass, label: 'Explore', href: '/explore' },
		{ icon: Bell, label: 'Notifications', href: '/notifications' },
		{ icon: EnvelopeOpen, label: 'Messages', href: '/messages' },
		{ icon: BookmarkSquare, label: 'Bookmarks', href: '/bookmarks' },
		{ icon: User, label: 'Profile', href: '/profile' },
		{ icon: Cog6Tooth, label: 'Settings', href: '/settings' }
	];
</script>

<aside
	class="group fixed left-0 top-0 z-40 flex h-dvh flex-col border-r bg-card transition-all duration-300 ease-in-out"
	class:w-[68px]={!expanded}
	class:w-[240px]={expanded}
	role="navigation"
	aria-label="Main navigation"
	onmouseenter={() => (expanded = true)}
	onmouseleave={() => (expanded = false)}
>
	<!-- Logo -->
	<div class="flex h-14 items-center px-5">
		<div class="flex size-8 shrink-0 items-center justify-center rounded-lg bg-primary">
			<span class="text-sm font-bold text-primary-foreground">S</span>
		</div>
		<span
			class="ml-3 whitespace-nowrap text-lg font-bold tracking-tight opacity-0 transition-opacity duration-200"
			class:opacity-100={expanded}
		>
			Sapphire
		</span>
	</div>

	<!-- Navigation Items -->
	<nav class="mt-2 flex flex-1 flex-col gap-0.5 px-3">
		{#each navItems as item}
			<a
				href={item.href}
				class="group/item flex items-center rounded-xl px-3 py-2.5 transition-colors duration-150"
				class:bg-muted={item.active}
				class:text-foreground={item.active}
				class:text-muted-foreground={!item.active}
				class:hover:bg-muted={!item.active}
				class:hover:text-foreground={!item.active}
			>
				<div class="flex size-7 shrink-0 items-center justify-center">
					<Icon src={item.icon} solid={item.active} size="22" />
				</div>
				<span
					class="ml-3 whitespace-nowrap text-[15px] font-medium opacity-0 transition-opacity duration-200"
					class:opacity-100={expanded}
					class:font-semibold={item.active}
				>
					{item.label}
				</span>
			</a>
		{/each}
	</nav>

	<!-- Post Button -->
	<div class="p-3">
		<button
			class="flex w-full items-center justify-center gap-2 rounded-xl bg-primary py-2.5 text-sm font-semibold text-primary-foreground transition-all hover:opacity-90"
		>
			<Icon src={PencilSquare} solid size="20" />
			<span
				class="whitespace-nowrap opacity-0 transition-opacity duration-200"
				class:opacity-100={expanded}
			>
				Post
			</span>
		</button>
	</div>

	<!-- User Section -->
	<div class="border-t p-3">
		<div class="flex items-center rounded-xl px-3 py-2 transition-colors hover:bg-muted">
			<div
				class="flex size-8 shrink-0 items-center justify-center rounded-full bg-gradient-to-br from-violet-500 to-indigo-600"
			>
				<span class="text-xs font-bold text-white">L</span>
			</div>
			<div
				class="ml-3 min-w-0 opacity-0 transition-opacity duration-200"
				class:opacity-100={expanded}
			>
				<p class="truncate text-sm font-semibold">levish</p>
				<p class="truncate text-xs text-muted-foreground">@evil_sh0t</p>
			</div>
		</div>
	</div>
</aside>
