<script lang="ts">
	import { page } from '$app/state';
	import { Separator } from '$lib/components/ui/separator';
	import * as Select from '$lib/components/ui/select';
	import { ExternalLinkIcon } from '@lucide/svelte';
	import type { MarketDto, MarketPercentagesDto } from '$lib/types';
	import type { PageProps } from './$types';
	import BuyOrderDialog from './buy-order-dialog.svelte';
	import BuyOrdersTable from './buy-orders-table.svelte';
	import EventChart from './event-chart.svelte';
	import PositionsTable from './positions-table.svelte';
	import ResolvedMarketRow from './resolved-market-row.svelte';
	import ActiveMarketRow from './active-market-row.svelte';

	let { data }: PageProps = $props();

	const user = $derived(page.data.user);

	let selectedRuleMarketId = $state('');

	// Initialize selected market on first render
	$effect(() => {
		if (!selectedRuleMarketId && data.event.markets.length > 0) {
			selectedRuleMarketId = data.event.markets[0].id;
		}
	});

	const selectedMarketRules = $derived(
		data.event.markets.find((m) => m.id === selectedRuleMarketId)?.rules ?? ''
	);

	const rulesSelectLabel = $derived(
		data.event.markets.find((m) => m.id === selectedRuleMarketId)?.displayName ??
			'Selecionar mercado'
	);

	// Buy dialog state
	let buyDialogOpen = $state(false);
	let buyDialogMarket = $state<MarketDto | null>(null);
	let buyDialogOption = $state<'A' | 'B'>('A');
	let buyDialogPercentages = $state<MarketPercentagesDto | null>(null);

	function openBuyDialog(market: MarketDto, option: 'A' | 'B') {
		buyDialogMarket = market;
		buyDialogOption = option;
		buyDialogPercentages = data.marketPercentages[market.id] ?? null;
		buyDialogOpen = true;
	}

	// Track which markets are expanded
	let expandedMarkets = $state<Record<string, boolean>>({});
</script>

<div class="mx-auto max-w-3xl">
	<!-- Event Title -->
	<div class="mb-1 flex items-center gap-4">
		{#if data.event.imageUrl}
			<img
				src={data.event.imageUrl}
				alt={data.event.displayName}
				class="size-14 shrink-0 rounded-lg object-cover"
			/>
		{/if}
		<h1 class="text-2xl font-bold">{data.event.displayName}</h1>
	</div>
	{#if data.event.url}
		<a
			href={data.event.url}
			target="_blank"
			rel="noopener noreferrer"
			class="mb-6 inline-flex items-center gap-1.5 rounded-full border bg-muted/50 px-3 py-1 text-xs font-medium text-muted-foreground transition-colors hover:bg-muted hover:text-foreground"
		>
			<span class="inline-block h-2 w-2 rounded-full bg-green-500"></span>
			{data.event.pubkey.slice(0, 6)}...{data.event.pubkey.slice(-6)}
			<ExternalLinkIcon class="h-3 w-3" />
		</a>
	{:else}
		<div class="mb-6"></div>
	{/if}

	<!-- Price Chart -->
	<EventChart markets={data.event.markets} chartData={data.chartData} />

	<div class="rounded-lg border bg-card text-card-foreground">
		{#each data.event.markets as market, i (market.id)}
			{#if i > 0}
				<Separator />
			{/if}
			{#if market.resolvedOption != null}
				<ResolvedMarketRow {market} />
			{:else}
				<ActiveMarketRow
					{market}
					orders={data.allMarketOrders[market.id] ?? []}
					percentages={data.marketPercentages[market.id]}
					expanded={expandedMarkets[market.id] ?? false}
					onExpandedChange={(v) => (expandedMarkets[market.id] = v)}
					onBuy={openBuyDialog}
				/>
			{/if}
		{/each}
	</div>

	<!-- Positions -->
	{#if user && data.positions.length > 0}
		<PositionsTable positions={data.positions} markets={data.event.markets} />
	{/if}

	<!-- Pending Buy Orders -->
	{#if user && data.buyOrders.length > 0}
		<BuyOrdersTable buyOrders={data.buyOrders} markets={data.event.markets} />
	{/if}

	<div class="mt-8">
		<div class="mb-3 flex items-center justify-between gap-4">
			<h2 class="text-lg font-semibold">Regras</h2>
			{#if data.event.markets.length > 1}
				<Select.Root type="single" bind:value={selectedRuleMarketId}>
					<Select.Trigger class="w-55">
						{rulesSelectLabel}
					</Select.Trigger>
					<Select.Content>
						{#each data.event.markets as market (market.id)}
							<Select.Item value={market.id} label={market.displayName}>
								{market.displayName}
							</Select.Item>
						{/each}
					</Select.Content>
				</Select.Root>
			{/if}
		</div>
		<div class="rounded-lg border bg-card text-card-foreground p-4">
			{#if selectedMarketRules}
				<p class="text-sm leading-relaxed whitespace-pre-wrap text-muted-foreground">
					{selectedMarketRules}
				</p>
			{:else}
				<p class="text-sm text-muted-foreground">Sem regras definidas para este mercado.</p>
			{/if}
		</div>
	</div>
</div>

<!-- Buy Order Dialog -->
<BuyOrderDialog
	bind:open={buyDialogOpen}
	market={buyDialogMarket}
	bind:option={buyDialogOption}
	percentages={buyDialogPercentages}
	user={user ?? null}
	balanceCents={page.data.balanceCents}
/>
