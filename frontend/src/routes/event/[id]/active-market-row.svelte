<script lang="ts">
	import { Button } from '$lib/components/ui/button';
	import * as Collapsible from '$lib/components/ui/collapsible';
	import ChevronDown from '@lucide/svelte/icons/chevron-down';
	import type { BuyOrderDto, MarketDto, MarketPercentagesDto } from '$lib/types';
	import MarketOrderBook from './market-order-book.svelte';

	interface Props {
		market: MarketDto;
		orders: BuyOrderDto[];
		percentages: MarketPercentagesDto | undefined;
		expanded: boolean;
		onExpandedChange: (expanded: boolean) => void;
		onBuy: (market: MarketDto, option: 'A' | 'B') => void;
	}

	let { market, orders, percentages, expanded, onExpandedChange, onBuy }: Props = $props();

	const orderCount = $derived(orders.length);
	const pct = $derived(percentages);
</script>

<Collapsible.Root open={expanded} onOpenChange={onExpandedChange}>
	<div class="flex items-center gap-4 px-4 py-3">
		<Collapsible.Trigger class="flex min-w-0 flex-1 cursor-pointer items-center gap-2 text-left">
			<ChevronDown
				class="h-4 w-4 shrink-0 text-muted-foreground transition-transform duration-200 {expanded ? 'rotate-180' : ''}"
			/>
			<div class="min-w-0">
				<p class="truncate text-sm font-semibold">{market.displayName}</p>
				<p class="text-xs text-muted-foreground">
					{orderCount}
					{orderCount === 1 ? 'ordem aberta' : 'ordens abertas'}
				</p>
			</div>
		</Collapsible.Trigger>

		<span class="shrink-0 text-lg font-bold"
			>{pct?.optionAPercentage != null ? `${pct.optionAPercentage}%` : '–'}</span
		>

		<div class="flex shrink-0 items-center gap-2">
			<Button
				size="sm"
				class="group relative h-8 min-w-[120px] bg-green-600 px-4 text-xs font-semibold text-white hover:bg-green-700"
				onclick={() => onBuy(market, 'A')}
			>
				<span class="transition-opacity group-hover:opacity-0"
					>Comprar {market.optionAName}</span
				>
				<span
					class="absolute inset-0 flex items-center justify-center opacity-0 transition-opacity group-hover:opacity-100"
				>
					{pct?.optionAPercentage != null ? `${pct.optionAPercentage}¢` : '–'}
				</span>
			</Button>
			<Button
				size="sm"
				class="group relative h-8 min-w-[120px] bg-red-600 px-4 text-xs font-semibold text-white hover:bg-red-700"
				onclick={() => onBuy(market, 'B')}
			>
				<span class="transition-opacity group-hover:opacity-0"
					>Comprar {market.optionBName}</span
				>
				<span
					class="absolute inset-0 flex items-center justify-center opacity-0 transition-opacity group-hover:opacity-100"
				>
					{pct?.optionBPercentage != null ? `${pct.optionBPercentage}¢` : '–'}
				</span>
			</Button>
		</div>
	</div>

	<Collapsible.Content>
		<MarketOrderBook {market} {orders} />
	</Collapsible.Content>
</Collapsible.Root>
