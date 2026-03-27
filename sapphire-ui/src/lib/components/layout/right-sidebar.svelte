<script lang="ts">
	import {
		ArrowTopRightOnSquare,
		Banknotes,
		CalendarDays,
		ChatBubbleBottomCenterText,
		CodeBracket,
		GlobeAlt,
		Icon,
		Link,
		MapPin,
		PaperAirplane,
		Sparkles,
		UserGroup,
		Users,
		type IconSource
	} from 'svelte-hero-icons';

	const panelClass = 'rounded-2xl border border-border bg-card p-5 shadow-sm';

	interface DetailItem {
		icon: IconSource;
		label: string;
		value?: string;
		status?: string;
	}

	const keyFacts: DetailItem[] = [
		{ icon: Users, label: 'Employees', value: '500 - 1500 people' },
		{ icon: GlobeAlt, label: 'Country', value: 'South Korea' },
		{ icon: CalendarDays, label: 'Year established', value: '2021' },
		{ icon: MapPin, label: 'Presence', value: 'Seoul, Tokyo, Singapore, San Francisco' }
	];

	const collaboration: DetailItem[] = [
		{ icon: UserGroup, label: 'Partnership questions', status: 'Available' },
		{ icon: Sparkles, label: 'Joint launches', status: 'Open to proposals' },
		{ icon: Banknotes, label: 'Investments', status: 'Seed to Series A' }
	];

	const contacts: DetailItem[] = [
		{ icon: GlobeAlt, label: 'Website', value: 'sapphire.social' },
		{ icon: PaperAirplane, label: 'Telegram', value: '@sapphire_network' },
		{ icon: CodeBracket, label: 'GitHub', value: '@sapphire-ui' },
		{ icon: Link, label: 'Docs', value: 'creator-kit / partnership deck' }
	];
</script>

<div class="flex flex-col gap-4 pb-5">
	<section class={panelClass}>
		<div class="flex items-center justify-between gap-3">
			<h2 class="text-sm font-semibold">Key Facts</h2>
			<span
				class="rounded-full border border-border bg-muted/40 px-2.5 py-1 text-[11px] tracking-[0.22em] text-muted-foreground uppercase"
			>
				Verified
			</span>
		</div>

		<div class="mt-4 flex flex-col gap-3">
			{#each keyFacts as item}
				<div class="flex items-start gap-3 rounded-xl border border-border bg-muted/30 px-3 py-3">
					<div
						class="flex size-10 shrink-0 items-center justify-center rounded-lg border border-border bg-background text-foreground"
					>
						<Icon src={item.icon} solid size="18" />
					</div>
					<div class="min-w-0">
						<p class="text-[11px] tracking-[0.22em] text-muted-foreground uppercase">
							{item.label}
						</p>
						<p class="mt-1 text-sm leading-6">{item.value}</p>
					</div>
				</div>
			{/each}
		</div>
	</section>

	<section class={panelClass}>
		<h2 class="text-sm font-semibold">Collaboration</h2>
		<p class="mt-2 text-sm leading-6 text-muted-foreground">
			Share a campaign idea, creator activation, or product collaboration and the team will route it
			to the right lead.
		</p>

		<div class="mt-4 flex flex-col gap-3">
			{#each collaboration as item}
				<div class="flex items-center gap-3 rounded-xl border border-border bg-muted/30 px-3 py-3">
					<div
						class="flex size-10 shrink-0 items-center justify-center rounded-lg border border-border bg-background text-foreground"
					>
						<Icon src={item.icon} solid size="18" />
					</div>
					<div class="min-w-0 flex-1">
						<p class="truncate text-sm font-medium">{item.label}</p>
						<p class="mt-1 text-xs font-medium text-primary">{item.status}</p>
					</div>
				</div>
			{/each}
		</div>

		<button
			class="mt-4 flex h-10 w-full items-center justify-center gap-2 rounded-xl bg-primary text-sm font-semibold text-primary-foreground transition-opacity hover:opacity-90"
		>
			<Icon src={ChatBubbleBottomCenterText} solid size="18" />
			Propose collaboration
		</button>
	</section>

	<section class={panelClass}>
		<div class="flex items-center justify-between gap-3">
			<h2 class="text-sm font-semibold">Contacts</h2>
			<button
				class="flex size-9 items-center justify-center rounded-full border border-border bg-muted/40 text-muted-foreground transition-colors hover:text-foreground"
				aria-label="Open external links"
			>
				<Icon src={ArrowTopRightOnSquare} solid size="18" />
			</button>
		</div>

		<div class="mt-4 flex flex-col gap-3">
			{#each contacts as item}
				<div class="flex items-start gap-3 rounded-xl border border-border bg-muted/30 px-3 py-3">
					<div
						class="flex size-10 shrink-0 items-center justify-center rounded-lg border border-border bg-background text-foreground"
					>
						<Icon src={item.icon} solid size="18" />
					</div>
					<div class="min-w-0">
						<p class="text-[11px] tracking-[0.22em] text-muted-foreground uppercase">
							{item.label}
						</p>
						<p class="mt-1 text-sm">{item.value}</p>
					</div>
				</div>
			{/each}
		</div>
	</section>
</div>
