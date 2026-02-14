<script lang="ts">
	import * as Table from '$lib/components/ui/table';
	import type { BuyOrderDto, MarketDto } from '$lib/types';

	interface Props {
		buyOrders: BuyOrderDto[];
		markets: MarketDto[];
	}

	let { buyOrders, markets }: Props = $props();

	const marketsById = $derived(new Map(markets.map((m) => [m.id, m])));

	function getOptionName(marketId: string, option: string): string {
		const market = marketsById.get(marketId);
		if (!market) return option;
		return option === 'optionA' ? market.optionAName : market.optionBName;
	}

	function getMarketName(marketId: string): string {
		return marketsById.get(marketId)?.displayName ?? 'Mercado desconhecido';
	}

	function formatEuros(cents: number): string {
		return (cents / 100).toFixed(2) + '€';
	}
</script>

<div class="mt-8">
	<h2 class="mb-3 text-lg font-semibold">Ordens de compra pendentes</h2>
	<div class="rounded-lg border">
		<Table.Root>
			<Table.Header>
				<Table.Row>
					<Table.Head>Mercado</Table.Head>
					<Table.Head>Opção</Table.Head>
					<Table.Head class="text-end">Ações</Table.Head>
					<Table.Head class="text-end">Preço/ação</Table.Head>
					<Table.Head class="text-end">Total</Table.Head>
				</Table.Row>
			</Table.Header>
			<Table.Body>
				{#each buyOrders as order (order.id)}
					<Table.Row>
						<Table.Cell class="font-medium">
							{getMarketName(order.marketId)}
						</Table.Cell>
						<Table.Cell>
							<span
								class="inline-flex items-center rounded-full px-2 py-0.5 text-xs font-semibold {order.option === 'optionA'
									? 'bg-primary/10 text-primary'
									: 'bg-destructive/10 text-destructive'}"
							>
								{getOptionName(order.marketId, order.option)}
							</span>
						</Table.Cell>
						<Table.Cell class="text-end font-medium">
							{order.shares}
						</Table.Cell>
						<Table.Cell class="text-end">
							{order.pricePerShare}¢
						</Table.Cell>
						<Table.Cell class="text-end font-medium">
							{formatEuros(order.shares * order.pricePerShare)}
						</Table.Cell>
					</Table.Row>
				{/each}
			</Table.Body>
		</Table.Root>
	</div>
</div>
