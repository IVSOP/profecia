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
</script>

<div class="flex items-center gap-4 px-4 py-3">
	<div class="min-w-0 flex-1">
		<p class="truncate text-sm font-semibold">{market.displayName}</p>
		<p class="text-xs text-muted-foreground">Mercado encerrado</p>
	</div>
	<div
		class="flex shrink-0 items-center gap-2 rounded-full bg-emerald-100 px-3 py-1 dark:bg-emerald-900/40"
	>
		<Trophy class="h-4 w-4 text-emerald-600 dark:text-emerald-400" />
		<span class="text-sm font-semibold text-emerald-700 dark:text-emerald-300">{winnerName}</span>
	</div>
</div>
