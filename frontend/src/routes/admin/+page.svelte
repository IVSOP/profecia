<script lang="ts">
	import { Button } from '$lib/components/ui/button';
	import * as Table from '$lib/components/ui/table';
	import type { EventDto } from '$lib/types';
	import type { PageData } from './$types';
	import CreateEventDialog from './create-event-dialog.svelte';

	let { data }: { data: PageData } = $props();

	let createdEvents = $state<EventDto[]>([]);
	let events = $derived<EventDto[]>([...createdEvents, ...data.events]);
	let dialogOpen = $state(false);

	function handleEventCreated(event: EventDto) {
		createdEvents = [event, ...createdEvents];
	}
</script>

<div class="space-y-6">
	<div class="flex items-center justify-between">
		<div>
			<h1 class="text-2xl font-bold tracking-tight">Administração</h1>
			<p class="text-sm text-muted-foreground">Gerir eventos e mercados da plataforma.</p>
		</div>
		<Button onclick={() => (dialogOpen = true)}>Criar Evento</Button>
	</div>

	<div class="rounded-lg border bg-card text-card-foreground">
		<Table.Root class="[&_td:first-child]:pl-4 [&_td:last-child]:pr-4 [&_th:first-child]:pl-4 [&_th:last-child]:pr-4">
			<Table.Header>
				<Table.Row>
					<Table.Head>Evento</Table.Head>
					<Table.Head>Mercados</Table.Head>
					<Table.Head class="w-35">Estado</Table.Head>
				</Table.Row>
			</Table.Header>
			<Table.Body>
				{#if events.length === 0}
					<Table.Row>
						<Table.Cell colspan={3} class="py-8 text-center text-muted-foreground">
							Nenhum evento encontrado.
						</Table.Cell>
					</Table.Row>
				{:else}
					{#each events as event (event.id)}
						<Table.Row>
							<Table.Cell class="font-medium">{event.displayName}</Table.Cell>
							<Table.Cell>
								<div class="flex flex-col gap-1">
									{#each event.markets as market (market.id)}
										<span class="text-sm">
											{market.displayName}
											<span class="text-muted-foreground">
												({market.optionAName} vs {market.optionBName})
											</span>
										</span>
									{/each}
								</div>
							</Table.Cell>
							<Table.Cell>
								{#if event.markets.some((m) => m.resolvedOption !== null)}
									<span
										class="inline-flex items-center rounded-full bg-green-600/15 px-2.5 py-0.5 text-xs font-medium text-green-700 dark:text-green-400"
									>
										Resolvido
									</span>
								{:else}
									<span
										class="inline-flex items-center rounded-full bg-blue-600/15 px-2.5 py-0.5 text-xs font-medium text-blue-700 dark:text-blue-400"
									>
										Ativo
									</span>
								{/if}
							</Table.Cell>
						</Table.Row>
					{/each}
				{/if}
			</Table.Body>
		</Table.Root>
	</div>
</div>

<CreateEventDialog bind:open={dialogOpen} oncreated={handleEventCreated} />
