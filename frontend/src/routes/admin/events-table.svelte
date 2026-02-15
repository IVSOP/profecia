<script lang="ts">
	import { ChevronDown, ChevronRight, Gavel } from '@lucide/svelte';
	import { Button } from '$lib/components/ui/button';
	import * as Table from '$lib/components/ui/table';
	import type { EventDto, MarketDto } from '$lib/types';

	interface Props {
		events: EventDto[];
		onresolve: (market: MarketDto) => void;
	}

	let { events, onresolve }: Props = $props();

	let expandedEvents = $state<Set<string>>(new Set());

	function toggleEvent(eventId: string) {
		if (expandedEvents.has(eventId)) {
			expandedEvents.delete(eventId);
		} else {
			expandedEvents.add(eventId);
		}
		expandedEvents = new Set(expandedEvents);
	}

	function getEventStatus(event: EventDto): 'resolved' | 'partial' | 'active' {
		const allResolved = event.markets.every((m) => m.resolvedOption !== null);
		const someResolved = event.markets.some((m) => m.resolvedOption !== null);
		if (allResolved) return 'resolved';
		if (someResolved) return 'partial';
		return 'active';
	}
</script>

<div class="rounded-lg border bg-card text-card-foreground">
	<Table.Root
		class="[&_td]:py-4 [&_th]:py-4 [&_td:first-child]:pl-4 [&_td:last-child]:pr-4 [&_th:first-child]:pl-4 [&_th:last-child]:pr-4"
	>
		<Table.Header>
			<Table.Row>
				<Table.Head class="w-10"></Table.Head>
				<Table.Head>Nome</Table.Head>
				<Table.Head class="w-35">Estado</Table.Head>
				<Table.Head class="w-24"></Table.Head>
			</Table.Row>
		</Table.Header>
		<Table.Body>
			{#if events.length === 0}
				<Table.Row>
					<Table.Cell colspan={4} class="py-8 text-center text-muted-foreground">
						Nenhum evento encontrado.
					</Table.Cell>
				</Table.Row>
			{:else}
				{#each events as event (event.id)}
					{@const status = getEventStatus(event)}
					{@const isExpanded = expandedEvents.has(event.id)}
					<Table.Row
						class="cursor-pointer transition-colors hover:bg-muted/50"
						onclick={() => toggleEvent(event.id)}
					>
						<Table.Cell class="w-10 pr-0">
							{#if isExpanded}
								<ChevronDown class="text-muted-foreground" size={16} />
							{:else}
								<ChevronRight class="text-muted-foreground" size={16} />
							{/if}
						</Table.Cell>
						<Table.Cell class="font-medium">
							{event.displayName}
							<span class="ml-2 text-xs text-muted-foreground">
								({event.markets.length}
								{event.markets.length === 1 ? 'mercado' : 'mercados'})
							</span>
						</Table.Cell>
						<Table.Cell>
							{#if status === 'resolved'}
								<span
									class="inline-flex items-center rounded-full bg-green-600/15 px-2.5 py-0.5 text-xs font-medium text-green-700 dark:text-green-400"
								>
									Resolvido
								</span>
							{:else if status === 'partial'}
								<span
									class="inline-flex items-center rounded-full bg-yellow-600/15 px-2.5 py-0.5 text-xs font-medium text-yellow-700 dark:text-yellow-400"
								>
									Parcial
								</span>
							{:else}
								<span
									class="inline-flex items-center rounded-full bg-blue-600/15 px-2.5 py-0.5 text-xs font-medium text-blue-700 dark:text-blue-400"
								>
									Ativo
								</span>
							{/if}
						</Table.Cell>
						<Table.Cell></Table.Cell>
					</Table.Row>

					{#if isExpanded}
						{#each event.markets as market (market.id)}
							<Table.Row class="bg-muted/30">
								<Table.Cell class="w-10"></Table.Cell>
								<Table.Cell>
									<div class="flex flex-col">
										<span class="text-sm">{market.displayName}</span>
										<span class="text-xs text-muted-foreground">
											{market.optionAName} vs {market.optionBName}
										</span>
									</div>
								</Table.Cell>
								<Table.Cell>
									{#if market.resolvedOption}
										<span
											class="inline-flex items-center rounded-full bg-green-600/15 px-2.5 py-0.5 text-xs font-medium text-green-700 dark:text-green-400"
										>
											{market.resolvedOption === 'optionA'
												? market.optionAName
												: market.optionBName}
										</span>
									{:else}
										<span
											class="inline-flex items-center rounded-full bg-blue-600/15 px-2.5 py-0.5 text-xs font-medium text-blue-700 dark:text-blue-400"
										>
											Ativo
										</span>
									{/if}
								</Table.Cell>
								<Table.Cell class="text-right">
									{#if !market.resolvedOption}
										<Button
											variant="outline"
											size="sm"
											onclick={(e) => {
												e.stopPropagation();
												onresolve(market);
											}}
										>
											<Gavel size={14} />
											Resolver
										</Button>
									{/if}
								</Table.Cell>
							</Table.Row>
						{/each}
					{/if}
				{/each}
			{/if}
		</Table.Body>
	</Table.Root>
</div>
