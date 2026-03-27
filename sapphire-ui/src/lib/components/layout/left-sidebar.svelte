<script lang="ts">
	import {
		Bell,
		BookmarkSquare,
		CodeBracket,
		Cog6Tooth,
		EnvelopeOpen,
		Home,
		Icon,
		MagnifyingGlass,
		RocketLaunch,
		User,
		type IconSource
	} from 'svelte-hero-icons';
	import * as Avatar from '$lib/components/ui/avatar/index.js';

	interface NavItem {
		icon: IconSource;
		label: string;
		href: string;
		active: boolean;
	}

	const navItems: NavItem[] = [
		{ icon: Home, label: 'Home', href: '/', active: true },
		{ icon: MagnifyingGlass, label: 'Explore', href: '/explore', active: false },
		{ icon: Bell, label: 'Notifications', href: '/notifications', active: false },
		{ icon: EnvelopeOpen, label: 'Messages', href: '/messages', active: false },
		{ icon: BookmarkSquare, label: 'Bookmarks', href: '/bookmarks', active: false },
		{ icon: CodeBracket, label: 'Creator Studio', href: '/developer', active: false },
		{ icon: RocketLaunch, label: 'Launchpad', href: '/programs', active: false },
		{ icon: User, label: 'Profile', href: '/profile', active: false }
	];

	const profile = {
		name: 'Levi Park',
		handle: '@levi.sapphire'
	};
</script>

<nav
	class="group/sidebar sticky top-0 z-20 hidden h-dvh w-[5.25rem] shrink-0 flex-col overflow-hidden border-r border-sidebar-border bg-background text-foreground transition-[width] duration-200 ease-out hover:w-[17rem] md:flex"
	aria-label="Main navigation"
>
	<div class="flex h-full flex-col px-2 py-4">
		<div class="flex flex-1 flex-col gap-1 pt-2">
			{#each navItems as item}
				<a
					href={item.href}
					aria-current={item.active ? 'page' : undefined}
					class={`flex min-h-14 items-center gap-4 rounded-full px-4 text-base transition-colors ${
						item.active
							? 'font-semibold text-foreground'
							: 'text-foreground/80 hover:bg-muted/70 hover:text-foreground'
					}`}
				>
					<div class="flex size-7 shrink-0 items-center justify-center">
						<Icon src={item.icon} solid size="24" />
					</div>
					<span class="hidden truncate text-[1.05rem] group-hover/sidebar:block">
						{item.label}
					</span>
				</a>
			{/each}
		</div>

		<div class="mt-auto flex flex-col gap-2 border-t border-border/70 pt-4">
			<a
				href="/settings"
				class="flex min-h-12 items-center gap-4 rounded-full px-4 text-foreground/80 transition-colors hover:bg-muted/70 hover:text-foreground"
			>
				<div class="flex size-7 shrink-0 items-center justify-center">
					<Icon src={Cog6Tooth} solid size="22" />
				</div>
				<span class="hidden truncate text-sm font-medium group-hover/sidebar:block">Settings</span>
			</a>

			<a
				href="/profile"
				class="flex items-center gap-3 rounded-full px-3 py-3 transition-colors hover:bg-muted/70"
			>
				<Avatar.Root class="size-10 shrink-0">
					<Avatar.Image src="" alt="User avatar" />
					<Avatar.Fallback class="bg-primary text-xs font-semibold text-primary-foreground">
						LP
					</Avatar.Fallback>
				</Avatar.Root>
				<div class="hidden min-w-0 flex-1 group-hover/sidebar:block">
					<p class="truncate text-sm font-semibold text-foreground">{profile.name}</p>
					<p class="truncate text-sm text-muted-foreground">{profile.handle}</p>
				</div>
			</a>
		</div>
	</div>
</nav>
