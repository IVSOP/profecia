<script lang="ts">
	import type { BuyOrderDto, MarketDto } from '$lib/types';

	interface Props {
		market: MarketDto;
		orders: BuyOrderDto[];
	}

	let { market, orders }: Props = $props();

	// Aggregate orders by price level for each option
	interface PriceLevel {
		pricePerShare: number;
		totalShares: number;
		orderCount: number;
	}

	function aggregateByPrice(filteredOrders: BuyOrderDto[]): PriceLevel[] {
		const map = new Map<number, { totalShares: number; orderCount: number }>();
		for (const o of filteredOrders) {
			const existing = map.get(o.pricePerShare);
			if (existing) {
				existing.totalShares += o.shares;
				existing.orderCount += 1;
			} else {
				map.set(o.pricePerShare, { totalShares: o.shares, orderCount: 1 });
			}
		}
		return Array.from(map.entries())
			.map(([pricePerShare, { totalShares, orderCount }]) => ({
				pricePerShare,
				totalShares,
				orderCount
			}))
			.sort((a, b) => b.pricePerShare - a.pricePerShare);
	}

	const optionAOrders = $derived(aggregateByPrice(orders.filter((o) => o.option === 'optionA')));
	const optionBOrders = $derived(aggregateByPrice(orders.filter((o) => o.option === 'optionB')));

	// Collect all unique price points from both sides, sorted high to low
	const allPrices = $derived(() => {
		const priceSet = new Set<number>();
		for (const o of optionAOrders) priceSet.add(o.pricePerShare);
		for (const o of optionBOrders) priceSet.add(100 - o.pricePerShare);
		return Array.from(priceSet).sort((a, b) => b - a);
	});

	// Lookup maps for quick access
	const optionAByPrice = $derived(
		new Map(optionAOrders.map((l) => [l.pricePerShare, l]))
	);
	const optionBByInversePrice = $derived(
		new Map(optionBOrders.map((l) => [100 - l.pricePerShare, l]))
	);

	const maxShares = $derived(
		Math.max(
			...optionAOrders.map((l) => l.totalShares),
			...optionBOrders.map((l) => l.totalShares),
			1
		)
	);

	const hasOrders = $derived(orders.length > 0);
</script>

<div class="border-t border-border/50 bg-muted/30 px-4 py-3">
	{#if !hasOrders}
		<p class="py-2 text-center text-xs text-muted-foreground">Sem ordens de compra neste mercado.</p>
	{:else}
		<!-- Column headers -->
		<div class="mb-2 grid grid-cols-[1fr_auto_auto_auto_1fr] items-center gap-0">
			<!-- Left: Option A -->
			<div class="flex items-center gap-1.5 pr-2">
				<span class="inline-block h-2 w-2 shrink-0 rounded-full bg-green-500"></span>
				<span class="text-xs font-semibold text-green-600 dark:text-green-400">{market.optionAName}</span>
			</div>
			<!-- Center labels -->
			<span class="px-1 text-center text-[11px] font-medium text-muted-foreground">Preço</span>
			<span class="px-2 text-center text-[10px] text-muted-foreground">⇄</span>
			<span class="px-1 text-center text-[11px] font-medium text-muted-foreground">Preço</span>
			<!-- Right: Option B -->
			<div class="flex items-center justify-end gap-1.5 pl-2">
				<span class="text-xs font-semibold text-red-600 dark:text-red-400">{market.optionBName}</span>
				<span class="inline-block h-2 w-2 shrink-0 rounded-full bg-red-500"></span>
			</div>
		</div>

		<!-- Order rows -->
		<div class="space-y-0.5">
			{#each allPrices() as price (price)}
				{@const aLevel = optionAByPrice.get(price)}
				{@const bLevel = optionBByInversePrice.get(price)}
				<div class="grid grid-cols-[1fr_auto_auto_auto_1fr] items-center gap-0">
					<!-- Option A bar (grows right-to-left) -->
					<div class="flex items-center justify-end">
						{#if aLevel}
							<div class="relative flex w-full items-center justify-end rounded px-2 py-1">
								<div
									class="absolute inset-y-0 right-0 rounded bg-green-500/10"
									style="width: {(aLevel.totalShares / maxShares) * 100}%"
								></div>
								<span class="relative mr-2 text-xs tabular-nums text-muted-foreground">
									{aLevel.totalShares}
								</span>
							</div>
						{/if}
					</div>

					<!-- Center: price A | divider | price B (inverse) -->
					<span class="min-w-10 px-1 text-center text-xs font-semibold tabular-nums text-green-600 dark:text-green-400">
						{price}¢
					</span>
					<div class="flex items-center justify-center px-1">
						<div class="h-4 w-px bg-border"></div>
					</div>
					<span class="min-w-10 px-1 text-center text-xs font-semibold tabular-nums text-red-600 dark:text-red-400">
						{100 - price}¢
					</span>

					<!-- Option B bar (grows left-to-right) -->
					<div class="flex items-center justify-start">
						{#if bLevel}
							<div class="relative flex w-full items-center justify-start rounded px-2 py-1">
								<div
									class="absolute inset-y-0 left-0 rounded bg-red-500/10"
									style="width: {(bLevel.totalShares / maxShares) * 100}%"
								></div>
								<span class="relative ml-2 text-xs tabular-nums text-muted-foreground">
									{bLevel.totalShares}
								</span>
							</div>
						{/if}
					</div>
				</div>
			{/each}
		</div>

		<!-- Explanation -->
		<p class="mt-3 text-center text-[11px] text-muted-foreground">
			<span class="font-medium text-green-600 dark:text-green-400">{market.optionAName}</span> e
			<span class="font-medium text-red-600 dark:text-red-400">{market.optionBName}</span> proporcionais (N¢ ⇄ 100−N¢) são convertidas em ações
		</p>
	{/if}
</div>
