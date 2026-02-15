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

	function formatDollars(cents: number): string {
		return '$' + (cents / 100).toFixed(2);
	}

	function isResolved(marketId: string): boolean {
		return marketsById.get(marketId)?.resolvedOption != null;
	}

	function isWinner(position: PositionDto): boolean {
		const market = marketsById.get(position.marketId);
		return market?.resolvedOption === position.option;
	}

	/** Expected return: shares × 100¢ (payout if this option wins) */
	function expectedReturn(position: PositionDto): number {
		return position.shares * 100;
	}

	/** Actual result for a resolved position: payout minus cost (negative = loss) */
	function resolvedResult(position: PositionDto): number {
		const won = isWinner(position);
		const payout = won ? position.shares * 100 : 0;
		return payout - position.shares * position.pricePerShare;
	}

	const hasAnyResolved = $derived(positions.some((p) => isResolved(p.marketId)));
	const hasAnyOpen = $derived(positions.some((p) => !isResolved(p.marketId)));
	const returnColumnTitle = $derived(
		hasAnyResolved && hasAnyOpen ? 'Retorno' : hasAnyResolved ? 'Retorno' : 'Retorno Esperado'
	);
</script>

<div class="mt-8">
	<h2 class="mb-3 text-lg font-semibold">As tuas posições</h2>
	<div class="rounded-lg border bg-card text-card-foreground">
		<Table.Root
			class="[&_td]:px-5 [&_th]:px-5"
		>
			<Table.Header>
				<Table.Row>
					<Table.Head>Mercado</Table.Head>
					<Table.Head>Opção</Table.Head>
					<Table.Head class="text-end">Ações</Table.Head>
					<Table.Head class="text-end">Preço</Table.Head>
					<Table.Head class="text-end">Total</Table.Head>
					<Table.Head class="text-end">{returnColumnTitle}</Table.Head>
				</Table.Row>
			</Table.Header>
			<Table.Body>
				{#each positions as position (position.id)}
					{@const resolved = isResolved(position.marketId)}
					<Table.Row>
						<Table.Cell class="font-medium">
							{getMarketName(position.marketId)}
						</Table.Cell>
						<Table.Cell>
							<span
								class="inline-flex items-center rounded-full px-2 py-0.5 text-xs font-semibold {position.option ===
								'optionA'
									? 'bg-green-600/15 text-green-700 dark:text-green-400'
									: 'bg-red-600/15 text-red-700 dark:text-red-400'}"
							>
								{getOptionName(position.marketId, position.option)}
							</span>
						</Table.Cell>
						<Table.Cell class="text-end font-medium tabular-nums">
							{position.shares}
						</Table.Cell>
						<Table.Cell class="text-end text-muted-foreground tabular-nums">
							{position.pricePerShare}¢
						</Table.Cell>
						<Table.Cell class="text-end font-semibold tabular-nums">
							{formatDollars(position.shares * position.pricePerShare)}
						</Table.Cell>
						<Table.Cell class="text-end font-semibold tabular-nums">
							{#if resolved}
								{@const result = resolvedResult(position)}
								<span
									class={result >= 0
										? 'text-green-600 dark:text-green-400'
										: 'text-red-600 dark:text-red-400'}
								>
									{result >= 0 ? '+' : '-'}{formatDollars(Math.abs(result))}
								</span>
							{:else}
								<span>
									{formatDollars(expectedReturn(position))}
								</span>
							{/if}
						</Table.Cell>
					</Table.Row>
				{/each}
			</Table.Body>
		</Table.Root>
	</div>
</div>
