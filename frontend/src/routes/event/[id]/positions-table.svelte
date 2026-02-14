<script lang="ts">
	import * as Table from '$lib/components/ui/table';
	import type { MarketDto, PositionDto } from '$lib/types';

	interface Props {
		positions: PositionDto[];
		markets: MarketDto[];
	}

	let { positions, markets }: Props = $props();

	const marketsById = $derived(new Map(markets.map((m) => [m.id, m])));

	function getOptionName(marketId: string, option: string): string {
		const market = marketsById.get(marketId);
		if (!market) return option;
		return option === 'optionA' ? market.optionAName : market.optionBName;
	}

	function getMarketName(marketId: string): string {
		return marketsById.get(marketId)?.displayName ?? 'Mercado desconhecido';
	}
</script>

<div class="mt-8">
	<h2 class="mb-3 text-lg font-semibold">As tuas posições</h2>
	<div class="rounded-lg border">
		<Table.Root class="[&_td:first-child]:pl-4 [&_td:last-child]:pr-4 [&_th:first-child]:pl-4 [&_th:last-child]:pr-4">
			<Table.Header>
				<Table.Row>
					<Table.Head>Mercado</Table.Head>
					<Table.Head>Opção</Table.Head>
					<Table.Head class="text-end">Ações</Table.Head>
				</Table.Row>
			</Table.Header>
			<Table.Body>
				{#each positions as position (position.id)}
					<Table.Row>
						<Table.Cell class="font-medium">
							{getMarketName(position.marketId)}
						</Table.Cell>
						<Table.Cell>
							<span
								class="inline-flex items-center rounded-full px-2 py-0.5 text-xs font-semibold {position.option === 'optionA'
									? 'bg-green-100 text-green-700 dark:bg-green-900/30 dark:text-green-400'
									: 'bg-red-100 text-red-700 dark:bg-red-900/30 dark:text-red-400'}"
							>
								{getOptionName(position.marketId, position.option)}
							</span>
						</Table.Cell>
						<Table.Cell class="text-end font-medium">
							{position.shares}
						</Table.Cell>
					</Table.Row>
				{/each}
			</Table.Body>
		</Table.Root>
	</div>
</div>
