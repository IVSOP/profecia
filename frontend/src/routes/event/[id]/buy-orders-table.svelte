<script lang="ts">
	import { enhance } from '$app/forms';
	import { Button } from '$lib/components/ui/button';
	import * as Table from '$lib/components/ui/table';
	import type { BuyOrderDto, MarketDto } from '$lib/types';

	interface Props {
		buyOrders: BuyOrderDto[];
		markets: MarketDto[];
	}

	let { buyOrders, markets }: Props = $props();

	const marketsById = $derived(new Map(markets.map((m) => [m.id, m])));

	let cancellingId = $state('');

	function getOptionName(marketId: string, option: string): string {
		const market = marketsById.get(marketId);
		if (!market) return option;
		return option === 'optionA' ? market.optionAName : market.optionBName;
	}

	function getMarketName(marketId: string): string {
		return marketsById.get(marketId)?.displayName ?? 'Mercado desconhecido';
	}

	function formatDollars(cents: number): string {
		return '$' + (cents / 100).toFixed(2);
	}
</script>

<div class="mt-8">
	<h2 class="mb-3 text-lg font-semibold">Ordens de compra pendentes</h2>
	<div class="rounded-lg border bg-card text-card-foreground">
		<Table.Root class="[&_td:first-child]:pl-4 [&_td:last-child]:pr-4 [&_th:first-child]:pl-4 [&_th:last-child]:pr-4">
			<Table.Header>
				<Table.Row>
					<Table.Head>Mercado</Table.Head>
					<Table.Head>Opção</Table.Head>
					<Table.Head class="text-end">Ações</Table.Head>
					<Table.Head class="text-end">Preço/ação</Table.Head>
					<Table.Head class="text-end">Total</Table.Head>
					<Table.Head class="w-[1%]"></Table.Head>
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
									? 'bg-green-600/15 text-green-700 dark:text-green-400'
									: 'bg-red-600/15 text-red-700 dark:text-red-400'}"
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
							{formatDollars(order.shares * order.pricePerShare)}
						</Table.Cell>
						<Table.Cell>
							<form
								method="POST"
								action="?/cancelorder"
								use:enhance={() => {
									cancellingId = order.id;
									return async ({ update }) => {
										cancellingId = '';
										await update();
									};
								}}
							>
								<input type="hidden" name="orderId" value={order.id} />
								<Button
									type="submit"
									variant="ghost"
									size="sm"
									class="h-7 px-2 text-xs text-destructive hover:text-destructive"
									disabled={cancellingId === order.id}
								>
									Cancelar
								</Button>
							</form>
						</Table.Cell>
					</Table.Row>
				{/each}
			</Table.Body>
		</Table.Root>
	</div>
</div>
