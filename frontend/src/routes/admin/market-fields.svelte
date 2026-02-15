<script lang="ts">
	import { Label } from '$lib/components/ui/label';
	import { Input } from '$lib/components/ui/input';
	import { Button } from '$lib/components/ui/button';
	import type { CreateMarketRequest } from '$lib/types';
	import { Trash2 } from '@lucide/svelte';

	interface Props {
		market: CreateMarketRequest & { imageUrl: string };
		index: number;
		total: number;
		disabled: boolean;
		onremove: () => void;
	}

	let { market = $bindable(), index, total, disabled, onremove }: Props = $props();
</script>

<div class="rounded-lg border p-4">
	<div class="mb-3 flex items-center justify-between">
		<span class="text-sm font-medium text-muted-foreground">Mercado {index + 1}</span>
		{#if total > 1}
			<Button
				type="button"
				variant="ghost"
				size="sm"
				class="h-7 px-2 text-xs text-destructive hover:text-destructive"
				onclick={onremove}
				{disabled}
			>
				<Trash2 size={12} />
				Remover
			</Button>
		{/if}
	</div>
	<div class="grid gap-3">
		<div class="grid gap-2">
			<Label for="market-name-{index}">Nome do Mercado</Label>
			<Input
				id="market-name-{index}"
				placeholder="Ex: Vencedor da partida"
				bind:value={market.displayName}
				{disabled}
				required
			/>
		</div>
		<div class="grid grid-cols-2 gap-3">
			<div class="grid gap-2">
				<Label for="market-option-a-{index}">Opção A</Label>
				<Input
					id="market-option-a-{index}"
					placeholder="Ex: Sim"
					bind:value={market.optionAName}
					{disabled}
					required
				/>
			</div>
			<div class="grid gap-2">
				<Label for="market-option-b-{index}">Opção B</Label>
				<Input
					id="market-option-b-{index}"
					placeholder="Ex: Não"
					bind:value={market.optionBName}
					{disabled}
					required
				/>
			</div>
		</div>
		<div class="grid gap-2">
			<Label for="market-rules-{index}">Regras</Label>
			<Input
				id="market-rules-{index}"
				placeholder="Ex: Resultado no tempo regulamentar"
				bind:value={market.rules}
				{disabled}
			/>
		</div>
		<div class="grid gap-2">
			<Label for="market-image-url-{index}">Imagem do Mercado (URL, opcional)</Label>
			<Input
				id="market-image-url-{index}"
				placeholder="https://exemplo.com/imagem.jpg"
				bind:value={market.imageUrl}
				{disabled}
			/>
		</div>
	</div>
</div>
