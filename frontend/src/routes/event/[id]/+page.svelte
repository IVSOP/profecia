<script lang="ts">
	import { Button } from '$lib/components/ui/button';
	import { Separator } from '$lib/components/ui/separator';
	import * as Select from '$lib/components/ui/select';
	import * as Dialog from '$lib/components/ui/dialog';
	import type { MarketDto } from '$lib/types';
	import type { PageData } from './$types';

	let { data }: { data: PageData } = $props();

	let selectedRuleMarketId = $state('');

	// Initialize selected market on first render
	$effect(() => {
		if (!selectedRuleMarketId && data.event.markets.length > 0) {
			selectedRuleMarketId = data.event.markets[0].id;
		}
	});

	const selectedMarketRules = $derived(
		data.event.markets.find((m) => m.id === selectedRuleMarketId)?.rules ?? ''
	);

	const rulesSelectLabel = $derived(
		data.event.markets.find((m) => m.id === selectedRuleMarketId)?.displayName ??
			'Selecione um mercado'
	);

	// Buy dialog state
	let buyDialogOpen = $state(false);
	let buyDialogMarket = $state<MarketDto | null>(null);
	let buyDialogOption = $state<'A' | 'B'>('A');

	function openBuyDialog(market: MarketDto, option: 'A' | 'B') {
		buyDialogMarket = market;
		buyDialogOption = option;
		buyDialogOpen = true;
	}
</script>

<div class="mx-auto max-w-3xl">
	<!-- Event Title -->
	<h1 class="mb-6 text-2xl font-bold">{data.event.displayName}</h1>

	<div class="rounded-lg border">
		{#each data.event.markets as market, i (market.id)}
			{#if i > 0}
				<Separator />
			{/if}
			<div class="flex items-center gap-4 px-4 py-3">
				<div class="min-w-0 flex-1">
					<p class="truncate text-sm font-semibold">{market.displayName}</p>
				</div>

				<div class="flex shrink-0 items-center gap-1.5">
					<span class="text-lg font-bold">50%</span>
				</div>

				<div class="flex shrink-0 items-center gap-2">
					<Button
						size="sm"
						class="h-8 px-4 text-xs font-semibold"
						onclick={() => openBuyDialog(market, 'A')}
					>
						Comprar {market.optionAName}
					</Button>
					<Button
						size="sm"
						variant="destructive"
						class="h-8 px-4 text-xs font-semibold"
						onclick={() => openBuyDialog(market, 'B')}
					>
						Comprar {market.optionBName}
					</Button>
				</div>
			</div>
		{/each}
	</div>

	<div class="mt-8">
		<div class="mb-3 flex items-center justify-between gap-4">
			<h2 class="text-lg font-semibold">Regras</h2>
			{#if data.event.markets.length > 1}
				<Select.Root type="single" bind:value={selectedRuleMarketId}>
					<Select.Trigger class="w-55">
						{rulesSelectLabel}
					</Select.Trigger>
					<Select.Content>
						{#each data.event.markets as market (market.id)}
							<Select.Item value={market.id} label={market.displayName}>
								{market.displayName}
							</Select.Item>
						{/each}
					</Select.Content>
				</Select.Root>
			{/if}
		</div>
		<div class="rounded-lg border p-4">
			{#if selectedMarketRules}
				<p class="text-sm leading-relaxed whitespace-pre-wrap text-muted-foreground">
					{selectedMarketRules}
				</p>
			{:else}
				<p class="text-sm text-muted-foreground">Nenhuma regra definida para este mercado.</p>
			{/if}
		</div>
	</div>
</div>

<!-- Buy Dialog -->
<Dialog.Root bind:open={buyDialogOpen}>
	<Dialog.Content>
		<Dialog.Header>
			<Dialog.Title>
				{#if buyDialogMarket}
					Comprar {buyDialogOption === 'A'
						? buyDialogMarket.optionAName
						: buyDialogMarket.optionBName}
				{/if}
			</Dialog.Title>
			<Dialog.Description>
				{buyDialogMarket?.displayName ?? ''}
			</Dialog.Description>
		</Dialog.Header>
		<div class="flex items-center justify-center py-8 text-muted-foreground">
			<p class="text-sm">Em breve...</p>
		</div>
	</Dialog.Content>
</Dialog.Root>
