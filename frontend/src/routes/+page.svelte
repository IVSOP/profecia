<script lang="ts">
	import MarketCard from '$lib/components/market-card.svelte';
	import type { PageData } from './$types';

	let { data }: { data: PageData } = $props();
</script>

<svelte:head>
	<title>Profecia</title>
</svelte:head>

{#if data.events.length > 0}
	<div class="grid grid-cols-1 gap-4 sm:grid-cols-2 lg:grid-cols-3">
		{#each data.events as event (event.id)}
			<MarketCard {event} percentages={data.allPercentages[event.id] ?? {}} />
		{/each}
	</div>
{:else}
	<div class="flex flex-col items-center justify-center py-16 text-muted-foreground">
		<p class="text-lg">Não há eventos disponíveis</p>
	</div>
{/if}
