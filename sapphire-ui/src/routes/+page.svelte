<script lang="ts">
	import {
		ArrowTopRightOnSquare,
		Briefcase,
		BuildingOffice2,
		CalendarDays,
		ChatBubbleOvalLeft,
		Check,
		CheckBadge,
		CodeBracket,
		EllipsisHorizontal,
		GlobeAlt,
		Icon,
		MapPin,
		PaintBrush,
		PaperAirplane,
		Users,
		type IconSource
	} from 'svelte-hero-icons';

	interface OpeningItem {
		icon: IconSource;
		title: string;
		open: string;
		description: string;
	}

	const metrics = [
		{ label: 'Reach', value: '128K creators', meta: 'Monthly active network' },
		{ label: 'Partnerships', value: '34 live tracks', meta: 'Brands, labels, studios' },
		{ label: 'Markets', value: '11 launch regions', meta: 'Asia + North America' }
	];

	const capabilities = [
		'Creator identity and verification across product surfaces.',
		'Fast-moving collaboration workflows for launches, campaigns, and experimental drops.',
		'Social-native content operations with design, engineering, and community in one loop.'
	];

	const openings: OpeningItem[] = [
		{
			icon: CodeBracket,
			title: 'Engineering',
			open: '12 roles',
			description: 'Platform, ranking, trust systems, realtime infra.'
		},
		{
			icon: PaintBrush,
			title: 'Design',
			open: '5 roles',
			description: 'Product systems, motion direction, creator tooling.'
		},
		{
			icon: Briefcase,
			title: 'Strategy',
			open: '3 roles',
			description: 'Partnerships, market development, creator relations.'
		}
	];

	const updates = [
		{
			title: 'Campaign OS beta opened to studio partners',
			time: '6m ago',
			description:
				'Invited teams can now launch creator onboarding, approvals, and distribution without leaving the workspace.',
			tags: ['Launch', 'B2B'],
			author: 'Product'
		},
		{
			title: 'Identity profile cards now support live proof blocks',
			time: '19m ago',
			description:
				'Verification, portfolio signals, and community trust markers now stack into a single profile module.',
			tags: ['Identity', 'Profiles'],
			author: 'Platform'
		},
		{
			title: 'Regional ambassador program expanded to Tokyo and Singapore',
			time: '54m ago',
			description:
				'The community team opened new local activation tracks for events, creator scouting, and education programs.',
			tags: ['Community', 'Expansion'],
			author: 'Network'
		}
	];

	const events = [
		{
			title: 'Creator Systems Summit',
			date: 'Apr 12',
			location: 'Seoul',
			description: 'Closed-room sessions on ranking, trust, and content collaboration flows.'
		},
		{
			title: 'Launch Week Live',
			date: 'Apr 27',
			location: 'Tokyo',
			description:
				'Multi-day partnership demos with design drops, product reveals, and operator panels.'
		}
	];
</script>

<div class="flex flex-col bg-card">
	<section class="overflow-hidden border-b border-border">
		<div class="relative h-[220px] bg-muted sm:h-[260px]">
			<div class="absolute right-4 bottom-4">
				<div
					class="rounded-full border border-border bg-card px-3 py-1 text-xs text-muted-foreground"
				>
					Studio profile
				</div>
			</div>
		</div>

		<div class="px-5 py-6 sm:px-6">
			<div class="flex flex-col gap-5 lg:flex-row lg:items-end lg:justify-between">
				<div class="flex flex-col gap-4 sm:flex-row sm:items-end">
					<div
						class="flex size-24 shrink-0 items-center justify-center rounded-[1.6rem] bg-primary text-3xl font-semibold text-primary-foreground"
					>
						S
					</div>

					<div>
						<div class="flex flex-wrap items-center gap-2">
							<h1 class="text-2xl font-semibold tracking-tight sm:text-3xl">Sapphire Labs</h1>
							<Icon src={CheckBadge} solid size="20" class="text-primary" />
						</div>
						<p class="mt-1 text-sm text-muted-foreground sm:text-base">
							Social infrastructure / creator network / realtime collaboration
						</p>
						<div class="mt-3 flex flex-wrap gap-2">
							<span class="rounded-full border border-border bg-muted/40 px-3 py-1 text-xs">
								Product-led studio
							</span>
							<span class="rounded-full border border-border bg-muted/40 px-3 py-1 text-xs">
								Seoul HQ
							</span>
							<span class="rounded-full border border-border bg-muted/40 px-3 py-1 text-xs">
								Global creator programs
							</span>
						</div>
					</div>
				</div>

				<div class="flex flex-wrap items-center gap-2">
					<button
						class="flex h-10 items-center justify-center rounded-xl bg-primary px-5 text-sm font-semibold text-primary-foreground transition-opacity hover:opacity-90"
					>
						Connect +
					</button>
					<button
						class="flex h-10 items-center justify-center gap-2 rounded-xl border border-border bg-muted/40 px-4 text-sm font-medium transition-colors hover:bg-muted"
					>
						<Icon src={PaperAirplane} solid size="18" />
						Message
					</button>
					<button
						class="flex size-10 items-center justify-center rounded-xl border border-border bg-muted/40 transition-colors hover:bg-muted"
						aria-label="More actions"
					>
						<Icon src={EllipsisHorizontal} solid size="18" />
					</button>
				</div>
			</div>

			<div
				class="mt-6 overflow-hidden rounded-xl bg-muted/25 md:grid md:grid-cols-3 md:divide-x md:divide-border"
			>
				{#each metrics as metric}
					<div class="border-t border-border px-4 py-4 first:border-t-0 md:border-t-0">
						<p class="text-[11px] tracking-[0.24em] text-muted-foreground uppercase">
							{metric.label}
						</p>
						<p class="mt-2 text-lg font-semibold">{metric.value}</p>
						<p class="mt-1 text-sm text-muted-foreground">{metric.meta}</p>
					</div>
				{/each}
			</div>
		</div>
	</section>

	<section class="border-b border-border px-5 py-5 sm:px-6">
		<div class="flex flex-wrap items-center justify-between gap-3">
			<div>
				<p class="text-[11px] tracking-[0.24em] text-muted-foreground uppercase">About</p>
				<h2 class="mt-2 text-xl font-semibold">What Sapphire is building</h2>
			</div>
			<div class="flex items-center gap-2 text-sm text-muted-foreground">
				<Icon src={BuildingOffice2} solid size="18" />
				Studio + platform company
			</div>
		</div>

		<p class="mt-4 text-sm leading-7 text-muted-foreground">
			Sapphire Labs builds the social layer for identity, collaboration, and creator growth. The
			product combines profile credibility, distribution tooling, and partnership workflows so that
			teams can move from discovery to launch without stitching together separate systems.
		</p>

		<div class="mt-5 grid gap-3 lg:grid-cols-3">
			{#each capabilities as capability}
				<div class="rounded-xl bg-muted/25 px-4 py-4">
					<div class="flex items-start gap-3">
						<div
							class="mt-0.5 flex size-8 shrink-0 items-center justify-center rounded-full bg-primary/10 text-primary"
						>
							<Icon src={Check} solid size="16" />
						</div>
						<p class="text-sm leading-6 text-muted-foreground">{capability}</p>
					</div>
				</div>
			{/each}
		</div>
	</section>

	<section class="border-b border-border px-5 py-5 sm:px-6">
		<div class="flex flex-wrap items-center justify-between gap-3">
			<div>
				<p class="text-[11px] tracking-[0.24em] text-muted-foreground uppercase">Open Roles</p>
				<h2 class="mt-2 text-xl font-semibold">Hiring across product lines</h2>
			</div>
			<button
				class="flex items-center gap-2 rounded-full border border-border bg-muted/40 px-4 py-2 text-sm font-medium transition-colors hover:bg-muted"
			>
				View careers
				<Icon src={ArrowTopRightOnSquare} solid size="16" />
			</button>
		</div>

		<div class="mt-5 grid gap-3 md:grid-cols-3">
			{#each openings as item}
				<div class="rounded-xl bg-muted/25 p-4">
					<div
						class="flex size-11 items-center justify-center rounded-lg bg-background text-foreground"
					>
						<Icon src={item.icon} solid size="20" />
					</div>
					<h3 class="mt-4 text-base font-semibold">{item.title}</h3>
					<p class="mt-1 text-sm text-muted-foreground">{item.open}</p>
					<p class="mt-3 text-sm leading-6 text-muted-foreground">{item.description}</p>
				</div>
			{/each}
		</div>
	</section>

	<section class="border-b border-border px-5 py-5 sm:px-6">
		<div class="flex flex-wrap items-center justify-between gap-3">
			<div>
				<p class="text-[11px] tracking-[0.24em] text-muted-foreground uppercase">Signal Feed</p>
				<h2 class="mt-2 text-xl font-semibold">Latest updates</h2>
			</div>
			<div class="flex items-center gap-2 text-sm text-muted-foreground">
				<Icon src={GlobeAlt} solid size="18" />
				Live operator notes
			</div>
		</div>

		<div class="mt-5 flex flex-col">
			{#each updates as update}
				<article class="border-t border-border py-4 first:border-t-0 first:pt-0 last:pb-0">
					<div class="flex flex-wrap items-center justify-between gap-3">
						<div class="flex min-w-0 items-center gap-3">
							<div
								class="flex size-10 shrink-0 items-center justify-center rounded-lg bg-primary text-sm font-semibold text-primary-foreground"
							>
								S
							</div>
							<div class="min-w-0">
								<p class="truncate text-sm font-semibold">{update.title}</p>
								<p class="mt-1 text-xs text-muted-foreground">{update.author} / {update.time}</p>
							</div>
						</div>
						<div class="flex flex-wrap gap-2">
							{#each update.tags as tag}
								<span class="rounded-full bg-muted px-3 py-1 text-xs text-muted-foreground">
									{tag}
								</span>
							{/each}
						</div>
					</div>

					<p class="mt-4 text-sm leading-7 text-muted-foreground">{update.description}</p>

					<div class="mt-4 flex flex-wrap gap-2 text-sm">
						<button
							class="flex items-center gap-2 rounded-full bg-muted px-3 py-2 text-muted-foreground transition-colors hover:bg-muted/80"
						>
							<Icon src={ChatBubbleOvalLeft} solid size="16" />
							Discuss
						</button>
						<button
							class="flex items-center gap-2 rounded-full bg-muted px-3 py-2 text-muted-foreground transition-colors hover:bg-muted/80"
						>
							<Icon src={Users} solid size="16" />
							Share
						</button>
						<button
							class="flex items-center gap-2 rounded-full bg-muted px-3 py-2 text-muted-foreground transition-colors hover:bg-muted/80"
						>
							<Icon src={ArrowTopRightOnSquare} solid size="16" />
							Open
						</button>
					</div>
				</article>
			{/each}
		</div>
	</section>

	<section class="px-5 py-5 sm:px-6">
		<div class="flex flex-wrap items-center justify-between gap-3">
			<div>
				<p class="text-[11px] tracking-[0.24em] text-muted-foreground uppercase">Events</p>
				<h2 class="mt-2 text-xl font-semibold">Upcoming programs</h2>
			</div>
			<div class="flex items-center gap-2 text-sm text-muted-foreground">
				<Icon src={Users} solid size="18" />
				Partner-facing sessions
			</div>
		</div>

		<div class="mt-5 grid gap-3 md:grid-cols-2">
			{#each events as event}
				<div class="overflow-hidden rounded-xl bg-muted/25">
					<div class="h-28 bg-muted/60"></div>
					<div class="p-4">
						<div class="flex items-center gap-2 text-xs font-medium text-muted-foreground">
							<Icon src={CalendarDays} solid size="16" />
							{event.date}
							<span class="text-border">/</span>
							<Icon src={MapPin} solid size="16" />
							{event.location}
						</div>
						<h3 class="mt-3 text-base font-semibold">{event.title}</h3>
						<p class="mt-2 text-sm leading-6 text-muted-foreground">{event.description}</p>
					</div>
				</div>
			{/each}
		</div>
	</section>
</div>
