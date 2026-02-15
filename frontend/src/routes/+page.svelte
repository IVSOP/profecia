<script lang="ts">
	import MarketCard from './market-card.svelte';
	import { usePolling } from '$lib/hooks/use-polling.svelte';
	import type { PageData } from './$types';

	let { data }: { data: PageData } = $props();

	usePolling(30_000);

	const sortedEvents = $derived.by(() => {
		return [...data.events].sort((a, b) => {
			const aResolved = a.markets.length > 0 && a.markets.every((m) => m.resolvedOption !== null);
			const bResolved = b.markets.length > 0 && b.markets.every((m) => m.resolvedOption !== null);

			// Resolved events go to the end
			if (aResolved !== bResolved) return aResolved ? 1 : -1;

			// Among unresolved, sort by pending buy orders descending, then by shares
			const orderDiff = b.pendingBuyOrders - a.pendingBuyOrders;
			if (orderDiff !== 0) return orderDiff;
			return b.volume - a.volume;
		});
	});
</script>

<svelte:head>
	<title>Profecia</title>
</svelte:head>

{#if sortedEvents.length > 0}
	<div class="grid grid-cols-1 gap-4 sm:grid-cols-2 lg:grid-cols-3">
		{#each sortedEvents as event (event.id)}
			<MarketCard {event} percentages={data.allPercentages[event.id] ?? {}} />
		{/each}
	</div>
{:else}
	<div class="flex flex-col items-center justify-center py-16 text-muted-foreground">
		<p class="text-lg">Não há eventos disponíveis</p>
	</div>
{/if}
