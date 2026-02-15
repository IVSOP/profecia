<script lang="ts">
	import Trophy from '@lucide/svelte/icons/trophy';
	import type { MarketDto } from '$lib/types';

	interface Props {
		market: MarketDto;
	}

	let { market }: Props = $props();

	const winnerName = $derived(
		market.resolvedOption === 'optionA' ? market.optionAName : market.optionBName
	);
	const isOptionA = $derived(market.resolvedOption === 'optionA');
</script>

<div class="flex items-center gap-4 px-4 py-3">
	{#if market.imageUrl}
		<img
			src={market.imageUrl}
			alt={market.displayName}
			class="size-8 shrink-0 rounded object-cover"
		/>
	{/if}
	<div class="min-w-0 flex-1">
		<p class="truncate text-sm font-semibold">{market.displayName}</p>
		<p class="text-xs text-muted-foreground">Mercado encerrado</p>
	</div>
	<div
		class="flex shrink-0 items-center gap-2 rounded-full px-3 py-1 {isOptionA
			? 'bg-green-100 dark:bg-green-900/40'
			: 'bg-red-100 dark:bg-red-900/40'}"
	>
		<Trophy class="h-4 w-4 {isOptionA ? 'text-green-600 dark:text-green-400' : 'text-red-600 dark:text-red-400'}" />
		<span class="text-sm font-semibold {isOptionA ? 'text-green-700 dark:text-green-300' : 'text-red-700 dark:text-red-300'}">{winnerName}</span>
	</div>
</div>
