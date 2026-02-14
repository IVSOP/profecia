<script lang="ts">
	import * as Card from '$lib/components/ui/card';
	import { Button } from '$lib/components/ui/button';
	import type { EventDto, MarketPercentagesDto } from '$lib/types';

	interface Props {
		event: EventDto;
		percentages: Record<string, MarketPercentagesDto>;
	}

	let { event, percentages }: Props = $props();
</script>

<Card.Root class="w-full gap-0 overflow-hidden py-0">
	<Card.Header class="flex flex-row items-center gap-3 p-4">
		{#if event.imageUrl}
			<img
				src={event.imageUrl}
				alt={event.displayName}
				class="size-12 shrink-0 rounded-lg object-cover"
			/>
		{/if}
		<Card.Title class="text-base leading-snug">
			<a href="/event/{event.id}" class="hover:underline">{event.displayName}</a>
		</Card.Title>
	</Card.Header>
	<Card.Content class="flex flex-col gap-3 px-4 pt-0 pb-4">
		{#each event.markets as market (market.id)}
			{@const pct = percentages[market.id]}
			<div class="flex items-center justify-between gap-3">
				<div class="flex min-w-0 items-center gap-2">
					{#if market.imageUrl}
						<img
							src={market.imageUrl}
							alt={market.displayName}
							class="size-8 shrink-0 rounded object-cover"
						/>
					{/if}
					<a href="/event/{event.id}" class="min-w-0 truncate text-sm font-medium hover:underline"
						>{market.displayName}</a
					>
				</div>
				<div class="flex shrink-0 items-center gap-2">
					<span class="text-sm font-semibold">{pct?.optionAPercentage != null ? `${pct.optionAPercentage}%` : '–'}</span>
					<Button size="sm" variant="outline" class="h-7 border-green-600 px-3 text-xs text-green-600 hover:bg-green-600 hover:text-white">Sim</Button>
					<Button size="sm" variant="outline" class="h-7 border-red-600 px-3 text-xs text-red-600 hover:bg-red-600 hover:text-white">Não</Button>
				</div>
			</div>
		{/each}
	</Card.Content>
</Card.Root>
