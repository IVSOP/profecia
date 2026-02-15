<script lang="ts">
	import { Button } from '$lib/components/ui/button';
	import type { EventDto, MarketDto, MarketOption } from '$lib/types';
	import type { PageProps } from './$types';
	import CreateEventDialog from './create-event-dialog.svelte';
	import EventsTable from './events-table.svelte';
	import ResolveMarketDialog from './resolve-market-dialog.svelte';

	let { data }: PageProps = $props();

	let createdEvents = $state<EventDto[]>([]);
	let events = $derived<EventDto[]>([...createdEvents, ...data.events]);
	let dialogOpen = $state(false);

	let resolveDialogOpen = $state(false);
	let marketToResolve = $state<MarketDto | null>(null);

	function handleEventCreated(event: EventDto) {
		createdEvents = [event, ...createdEvents];
	}

	function handleMarketResolved(marketId: string, option: MarketOption) {
		function updateInList(list: EventDto[]): boolean {
			for (const event of list) {
				const market = event.markets.find((m) => m.id === marketId);
				if (market) {
					market.resolvedOption = option;
					return true;
				}
			}
			return false;
		}

		if (updateInList(createdEvents)) {
			createdEvents = [...createdEvents];
		} else if (updateInList(data.events)) {
			data = { ...data, events: [...data.events] };
		}
	}

	function openResolveDialog(market: MarketDto) {
		marketToResolve = market;
		resolveDialogOpen = true;
	}
</script>

<svelte:head>
	<title>Administração - Profecia</title>
</svelte:head>

<div class="space-y-6">
	<div class="flex items-center justify-between">
		<div>
			<h1 class="text-2xl font-bold tracking-tight">Administração</h1>
			<p class="text-sm text-muted-foreground">Gerir eventos e mercados da plataforma.</p>
		</div>
		<Button onclick={() => (dialogOpen = true)}>Criar Evento</Button>
	</div>

	<EventsTable {events} onresolve={openResolveDialog} />
</div>

<CreateEventDialog bind:open={dialogOpen} oncreated={handleEventCreated} />
<ResolveMarketDialog bind:open={resolveDialogOpen} market={marketToResolve} onresolved={handleMarketResolved} />
