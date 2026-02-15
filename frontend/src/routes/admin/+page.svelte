<script lang="ts">
	import { Button } from '$lib/components/ui/button';
	import * as ButtonGroup from '$lib/components/ui/button-group';
	import type { EventDto, MarketDto, MarketOption } from '$lib/types';
	import type { PageProps } from './$types';
	import BulkCreateEventDialog from './bulk-create-event-dialog.svelte';
	import CreateEventDialog from './create-event-dialog.svelte';
	import CreateMarketDialog from './create-market-dialog.svelte';
	import EditEventDialog from './edit-event-dialog.svelte';
	import EditMarketDialog from './edit-market-dialog.svelte';
	import EventsTable from './events-table.svelte';
	import ResolveMarketDialog from './resolve-market-dialog.svelte';

	let { data }: PageProps = $props();

	let createdEvents = $state<EventDto[]>([]);
	let resolvedMarkets = $state<Map<string, MarketOption>>(new Map());
	let updatedEvents = $state<Map<string, EventDto>>(new Map());
	let updatedMarkets = $state<Map<string, MarketDto>>(new Map());
	let addedMarkets = $state<Map<string, MarketDto[]>>(new Map());

	let events = $derived<EventDto[]>([
		...createdEvents,
		...data.events.map((event) => {
			const updatedEvent = updatedEvents.get(event.id);
			const base = updatedEvent ?? event;
			const extra = addedMarkets.get(event.id) ?? [];
			return {
				...base,
				markets: [
					...base.markets.map((m) => {
						const updatedMarket = updatedMarkets.get(m.id);
						const resolved = resolvedMarkets.get(m.id);
						let market = updatedMarket ?? m;
						if (resolved !== undefined) {
							market = { ...market, resolvedOption: resolved };
						}
						return market;
					}),
					...extra
				]
			};
		})
	]);

	// Create Event
	let createEventOpen = $state(false);
	let bulkCreateEventOpen = $state(false);

	function handleEventCreated(event: EventDto) {
		createdEvents = [event, ...createdEvents];
	}

	function handleBulkEventsCreated(events: EventDto[]) {
		createdEvents = [...events, ...createdEvents];
	}

	// Resolve Market
	let resolveDialogOpen = $state(false);
	let marketToResolve = $state<MarketDto | null>(null);

	function handleMarketResolved(marketId: string, option: MarketOption) {
		resolvedMarkets.set(marketId, option);
		resolvedMarkets = new Map(resolvedMarkets);
	}

	function openResolveDialog(market: MarketDto) {
		marketToResolve = market;
		resolveDialogOpen = true;
	}

	// Edit Event
	let editEventOpen = $state(false);
	let eventToEdit = $state<EventDto | null>(null);

	function openEditEventDialog(event: EventDto) {
		eventToEdit = event;
		editEventOpen = true;
	}

	function handleEventUpdated(event: EventDto) {
		updatedEvents.set(event.id, event);
		updatedEvents = new Map(updatedEvents);
	}

	// Edit Market
	let editMarketOpen = $state(false);
	let marketToEdit = $state<MarketDto | null>(null);

	function openEditMarketDialog(market: MarketDto) {
		marketToEdit = market;
		editMarketOpen = true;
	}

	function handleMarketUpdated(market: MarketDto) {
		updatedMarkets.set(market.id, market);
		updatedMarkets = new Map(updatedMarkets);
	}

	// Create Market
	let createMarketOpen = $state(false);
	let eventForNewMarket = $state<EventDto | null>(null);

	function openCreateMarketDialog(event: EventDto) {
		eventForNewMarket = event;
		createMarketOpen = true;
	}

	function handleMarketCreated(eventId: string, market: MarketDto) {
		const existing = addedMarkets.get(eventId) ?? [];
		addedMarkets.set(eventId, [...existing, market]);
		addedMarkets = new Map(addedMarkets);
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
		<ButtonGroup.Root>
			<Button onclick={() => (createEventOpen = true)}>Criar Evento</Button>
			<Button variant="outline" onclick={() => (bulkCreateEventOpen = true)}>Criar em Massa</Button>
		</ButtonGroup.Root>
	</div>

	<EventsTable
		{events}
		onresolve={openResolveDialog}
		oneditevent={openEditEventDialog}
		oneditmarket={openEditMarketDialog}
		oncreatemarket={openCreateMarketDialog}
	/>
</div>

<CreateEventDialog bind:open={createEventOpen} oncreated={handleEventCreated} />
<BulkCreateEventDialog bind:open={bulkCreateEventOpen} oncreated={handleBulkEventsCreated} />
<ResolveMarketDialog bind:open={resolveDialogOpen} market={marketToResolve} onresolved={handleMarketResolved} />
<EditEventDialog bind:open={editEventOpen} event={eventToEdit} onupdated={handleEventUpdated} />
<EditMarketDialog bind:open={editMarketOpen} market={marketToEdit} onupdated={handleMarketUpdated} />
<CreateMarketDialog bind:open={createMarketOpen} event={eventForNewMarket} oncreated={handleMarketCreated} />
