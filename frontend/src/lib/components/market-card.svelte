<script lang="ts">
	import * as Card from '$lib/components/ui/card';
	import Trophy from '@lucide/svelte/icons/trophy';
	import BarChart3 from '@lucide/svelte/icons/bar-chart-3';
	import type { EventDto, MarketPercentagesDto } from '$lib/types';

	interface Props {
		event: EventDto;
		percentages: Record<string, MarketPercentagesDto>;
	}

	let { event, percentages }: Props = $props();

	const MAX_VISIBLE_MARKETS = 2;

	const sortedMarkets = $derived.by(() => {
		return [...event.markets].sort((a, b) => {
			const pctA = percentages[a.id]?.optionAPercentage ?? -1;
			const pctB = percentages[b.id]?.optionAPercentage ?? -1;
			return pctB - pctA;
		});
	});

	const isSingleMarket = $derived(event.markets.length === 1);
	const visibleMarkets = $derived(sortedMarkets.slice(0, MAX_VISIBLE_MARKETS));
	const hiddenCount = $derived(Math.max(0, sortedMarkets.length - MAX_VISIBLE_MARKETS));

	function formatVolume(vol: number): string {
		if (vol >= 1_000_000) return `${(vol / 1_000_000).toFixed(1)}M`;
		if (vol >= 1_000) return `${(vol / 1_000).toFixed(1)}K`;
		return vol.toString();
	}
</script>

<Card.Root class="flex h-44 w-full flex-col gap-0 overflow-hidden p-4">
	<!-- Header -->
	<div class="flex items-center gap-3">
		{#if event.imageUrl}
			<img
				src={event.imageUrl}
				alt={event.displayName}
				class="size-12 shrink-0 rounded-lg object-cover"
			/>
		{/if}
		<div class="min-w-0 flex-1 text-base leading-snug font-semibold">
			<span class="flex items-center gap-1.5">
				{#if event.pendingBuyOrders > 0}
					<span class="relative flex h-2 w-2 shrink-0">
						<span
							class="absolute inline-flex h-full w-full animate-ping rounded-full bg-red-500 opacity-75"
						></span>
						<span class="relative inline-flex h-2 w-2 rounded-full bg-red-500"></span>
					</span>
				{/if}
				<a href="/event/{event.id}" class="hover:underline">{event.displayName}</a>
			</span>
			<span class="mt-0.5 flex items-center gap-1 text-xs font-normal text-muted-foreground">
				<BarChart3 class="h-3 w-3" />
				{formatVolume(event.volume)} Posições
			</span>
		</div>
		{#if isSingleMarket}
			{@const pct = percentages[event.markets[0].id]}
			{#if event.markets[0].resolvedOption != null}
				<span
					class="flex shrink-0 items-center gap-1.5 rounded-full bg-yellow-500 px-2.5 py-1 text-xs font-semibold text-white"
				>
					<Trophy class="h-3.5 w-3.5" />
					Encerrado
				</span>
			{:else}
				<span class="shrink-0 text-xl font-bold">
					{pct?.optionAPercentage != null ? `${pct.optionAPercentage}%` : '–'}
				</span>
			{/if}
		{/if}
	</div>

	<!-- Content -->
	<div class="mt-3 flex flex-1 flex-col">
		{#if isSingleMarket}
			<!-- Single market: prominent side-by-side buttons -->
			{@const market = event.markets[0]}
			{@const pct = percentages[market.id]}
			<div class="flex flex-1 items-end">
				{#if market.resolvedOption != null}
					<!-- Resolved single market -->
					<div class="grid w-full grid-cols-2 gap-2">
						<a
							href="/event/{event.id}"
							class="flex h-10 items-center justify-center gap-1.5 rounded-md text-sm font-semibold transition-colors {market.resolvedOption ===
							'optionA'
								? 'bg-green-600 text-white hover:bg-green-700'
								: 'bg-muted text-muted-foreground'}"
						>
							{#if market.resolvedOption === 'optionA'}
								<Trophy class="h-4 w-4" />
							{/if}
							{market.optionAName}
						</a>
						<a
							href="/event/{event.id}"
							class="flex h-10 items-center justify-center gap-1.5 rounded-md text-sm font-semibold transition-colors {market.resolvedOption ===
							'optionB'
								? 'bg-green-600 text-white hover:bg-green-700'
								: 'bg-muted text-muted-foreground'}"
						>
							{#if market.resolvedOption === 'optionB'}
								<Trophy class="h-4 w-4" />
							{/if}
							{market.optionBName}
						</a>
					</div>
				{:else}
					<!-- Active single market -->
					<div class="grid w-full grid-cols-2 gap-2">
						<a
							href="/event/{event.id}"
							class="flex h-10 items-center justify-center rounded-md bg-green-600 text-sm font-semibold text-white transition-colors hover:bg-green-700"
						>
							{market.optionAName}
							{#if pct?.optionAPercentage != null}
								<span class="ml-2 text-xs font-normal opacity-80">{pct.optionAPercentage}¢</span>
							{/if}
						</a>
						<a
							href="/event/{event.id}"
							class="flex h-10 items-center justify-center rounded-md bg-red-600 text-sm font-semibold text-white transition-colors hover:bg-red-700"
						>
							{market.optionBName}
							{#if pct?.optionBPercentage != null}
								<span class="ml-2 text-xs font-normal opacity-80">{pct.optionBPercentage}¢</span>
							{/if}
						</a>
					</div>
				{/if}
			</div>
		{:else}
			<!-- Multiple markets -->
			<div class="flex flex-col gap-2">
				{#each visibleMarkets as market (market.id)}
					{@const pct = percentages[market.id]}
					<div class="flex items-center gap-3">
						<div class="flex min-w-0 flex-1 items-center gap-2">
							{#if market.imageUrl}
								<img
									src={market.imageUrl}
									alt={market.displayName}
									class="size-7 shrink-0 rounded object-cover"
								/>
							{/if}
							<a
								href="/event/{event.id}"
								class="min-w-0 truncate text-sm font-medium hover:underline"
							>
								{market.displayName}
							</a>
						</div>
						{#if market.resolvedOption != null}
							<!-- Resolved market row -->
							<div class="flex shrink-0 items-center gap-1.5 rounded-full bg-green-600 px-2.5 py-1">
								<Trophy class="h-3.5 w-3.5 text-white" />
								<span class="text-xs font-semibold text-white">
									{market.resolvedOption === 'optionA' ? market.optionAName : market.optionBName}
								</span>
							</div>
						{:else}
							<!-- Active market row -->
							<span class="shrink-0 text-sm font-bold">
								{pct?.optionAPercentage != null ? `${pct.optionAPercentage}%` : '–'}
							</span>
							<div class="flex shrink-0 items-center gap-1.5">
								<a
									href="/event/{event.id}"
									class="flex h-7 items-center justify-center rounded-md bg-green-600 px-3 text-xs font-semibold text-white transition-colors hover:bg-green-700"
								>
									{market.optionAName}
								</a>
								<a
									href="/event/{event.id}"
									class="flex h-7 items-center justify-center rounded-md bg-red-600 px-3 text-xs font-semibold text-white transition-colors hover:bg-red-700"
								>
									{market.optionBName}
								</a>
							</div>
						{/if}
					</div>
				{/each}
			</div>
			{#if hiddenCount > 0}
				<a
					href="/event/{event.id}"
					class="mt-2 text-xs text-muted-foreground hover:text-foreground hover:underline"
				>
					Mais {hiddenCount}
					{hiddenCount === 1 ? 'mercado' : 'mercados'}
				</a>
			{/if}
		{/if}
	</div>
</Card.Root>
