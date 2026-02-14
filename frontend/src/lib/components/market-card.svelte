<script lang="ts">
	import * as Card from '$lib/components/ui/card';
	import { Button } from '$lib/components/ui/button';
	import type { EventDto } from '$lib/types';

	interface Props {
		event: EventDto;
	}

	let { event }: Props = $props();
</script>

<Card.Root class="w-full gap-0 overflow-hidden py-0">
	<Card.Header class="flex flex-row items-center gap-3 p-4">
		<img
			src="https://placehold.co/48x48"
			alt={event.display_name}
			class="size-12 shrink-0 rounded-lg object-cover"
		/>
		<Card.Title class="text-base leading-snug">{event.display_name}</Card.Title>
	</Card.Header>
	<Card.Content class="flex flex-col gap-3 px-4 pt-0 pb-4">
		{#each event.markets as market (market.id)}
			<div class="flex items-center justify-between gap-3">
				<span class="min-w-0 truncate text-sm font-medium">{market.display_name}</span>
				<div class="flex shrink-0 items-center gap-2">
					<span class="text-sm font-semibold">50%</span>
					<Button size="sm" variant="outline" class="h-7 px-3 text-xs">Sim</Button>
					<Button size="sm" variant="outline" class="h-7 px-3 text-xs">Não</Button>
				</div>
			</div>
		{/each}
	</Card.Content>
</Card.Root>
