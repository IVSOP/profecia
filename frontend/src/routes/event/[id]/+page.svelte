<script lang="ts">
	import { page } from '$app/state';
	import { Button } from '$lib/components/ui/button';
	import { Separator } from '$lib/components/ui/separator';
	import * as Select from '$lib/components/ui/select';
	import * as Collapsible from '$lib/components/ui/collapsible';
	import ChevronDown from '@lucide/svelte/icons/chevron-down';
	import type { MarketDto } from '$lib/types';
	import type { PageProps } from './$types';
	import BuyOrderDialog from './buy-order-dialog.svelte';
	import BuyOrdersTable from './buy-orders-table.svelte';
	import PositionsTable from './positions-table.svelte';
	import MarketOrderBook from './market-order-book.svelte';

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

	function openBuyDialog(market: MarketDto, option: 'A' | 'B') {
		buyDialogMarket = market;
		buyDialogOption = option;
		buyDialogOpen = true;
	}

	// Track which markets are expanded
	let expandedMarkets = $state<Record<string, boolean>>({});
</script>

<div class="mx-auto max-w-3xl">
	<!-- Event Title -->
	<h1 class="mb-6 text-2xl font-bold">{data.event.displayName}</h1>

	<div class="rounded-lg border bg-card text-card-foreground">
		{#each data.event.markets as market, i (market.id)}
			{#if i > 0}
				<Separator />
			{/if}
			{@const orderCount = data.allMarketOrders[market.id]?.length ?? 0}
			<Collapsible.Root
				open={expandedMarkets[market.id] ?? false}
				onOpenChange={(v) => (expandedMarkets[market.id] = v)}
			>
				<div class="flex items-center gap-4 px-4 py-3">
					<Collapsible.Trigger
						class="flex min-w-0 flex-1 cursor-pointer items-center gap-2 text-left"
					>
						<ChevronDown
							class="h-4 w-4 shrink-0 text-muted-foreground transition-transform duration-200 {expandedMarkets[market.id] ? 'rotate-180' : ''}"
						/>
						<div class="min-w-0">
							<p class="truncate text-sm font-semibold">{market.displayName}</p>
							<p class="text-xs text-muted-foreground">
								{orderCount} {orderCount === 1 ? 'ordem aberta' : 'ordens abertas'}
							</p>
						</div>
					</Collapsible.Trigger>

					<div class="flex shrink-0 items-center gap-1.5">
						<span class="text-lg font-bold">50%</span>
					</div>

					<div class="flex shrink-0 items-center gap-2">
						<Button
							size="sm"
							class="h-8 bg-green-600 px-4 text-xs font-semibold text-white hover:bg-green-700"
							onclick={() => openBuyDialog(market, 'A')}
						>
							Comprar {market.optionAName}
						</Button>
						<Button
							size="sm"
							class="h-8 bg-red-600 px-4 text-xs font-semibold text-white hover:bg-red-700"
							onclick={() => openBuyDialog(market, 'B')}
						>
							Comprar {market.optionBName}
						</Button>
					</div>
				</div>

				<Collapsible.Content>
					<MarketOrderBook market={market} orders={data.allMarketOrders[market.id] ?? []} />
				</Collapsible.Content>
			</Collapsible.Root>
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
	user={user ?? null}
/>
