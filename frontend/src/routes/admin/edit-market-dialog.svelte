<script lang="ts">
	import { Button } from '$lib/components/ui/button';
	import * as Dialog from '$lib/components/ui/dialog';
	import { Input } from '$lib/components/ui/input';
	import { Label } from '$lib/components/ui/label';
	import type { MarketDto, UpdateMarketRequest } from '$lib/types';
	import ErrorBanner from './error-banner.svelte';

	interface Props {
		open: boolean;
		market: MarketDto | null;
		onupdated: (market: MarketDto) => void;
	}

	let { open = $bindable(false), market, onupdated }: Props = $props();

	let loading = $state(false);
	let error = $state('');
	let displayName = $state('');
	let imageUrl = $state('');
	let optionAName = $state('');
	let optionBName = $state('');
	let rules = $state('');

	$effect(() => {
		if (market && open) {
			displayName = market.displayName;
			imageUrl = market.imageUrl ?? '';
			optionAName = market.optionAName;
			optionBName = market.optionBName;
			rules = market.rules;
		}
	});

	function reset() {
		displayName = '';
		imageUrl = '';
		optionAName = '';
		optionBName = '';
		rules = '';
		error = '';
		loading = false;
	}

	async function handleSubmit(e: SubmitEvent) {
		e.preventDefault();
		if (!market) return;
		error = '';

		if (!displayName.trim() || !optionAName.trim() || !optionBName.trim()) {
			error = 'Preencha todos os campos obrigatórios.';
			return;
		}

		loading = true;

		const payload: UpdateMarketRequest = {
			displayName: displayName.trim(),
			imageUrl: imageUrl.trim() || null,
			optionAName: optionAName.trim(),
			optionBName: optionBName.trim(),
			rules: rules.trim()
		};

		try {
			const response = await fetch(`/api/event/market/${market.id}`, {
				method: 'PATCH',
				headers: { 'Content-Type': 'application/json' },
				body: JSON.stringify(payload)
			});

			if (!response.ok) {
				const data = await response.json().catch(() => null);
				error = data?.error || 'Erro ao atualizar mercado. Tente novamente.';
				loading = false;
				return;
			}

			const updated = (await response.json()) as MarketDto;
			onupdated(updated);
			open = false;
			reset();
		} catch {
			error = 'Erro ao atualizar mercado. Tente novamente.';
		} finally {
			loading = false;
		}
	}
</script>

<Dialog.Root
	bind:open
	onOpenChange={(v) => {
		if (!v) reset();
	}}
>
	<Dialog.Content class="max-h-[85vh] overflow-y-auto sm:max-w-lg">
		<Dialog.Header>
			<Dialog.Title>Editar Mercado</Dialog.Title>
			<Dialog.Description>
				{#if market}
					Atualize as informações de <strong>{market.displayName}</strong>.
				{/if}
			</Dialog.Description>
		</Dialog.Header>
		<form onsubmit={handleSubmit}>
			<div class="grid gap-4">
				<ErrorBanner message={error} />

				<div class="grid gap-2">
					<Label for="edit-market-name">Nome do Mercado</Label>
					<Input
						id="edit-market-name"
						placeholder="Ex: Vencedor da partida"
						bind:value={displayName}
						disabled={loading}
						required
					/>
				</div>

				<div class="grid grid-cols-2 gap-3">
					<div class="grid gap-2">
						<Label for="edit-market-option-a">Opção A</Label>
						<Input
							id="edit-market-option-a"
							placeholder="Ex: Sim"
							bind:value={optionAName}
							disabled={loading}
							required
						/>
					</div>
					<div class="grid gap-2">
						<Label for="edit-market-option-b">Opção B</Label>
						<Input
							id="edit-market-option-b"
							placeholder="Ex: Não"
							bind:value={optionBName}
							disabled={loading}
							required
						/>
					</div>
				</div>

				<div class="grid gap-2">
					<Label for="edit-market-rules">Regras</Label>
					<Input
						id="edit-market-rules"
						placeholder="Ex: Resultado no tempo regulamentar"
						bind:value={rules}
						disabled={loading}
					/>
				</div>

				<div class="grid gap-2">
					<Label for="edit-market-image-url">Imagem (URL, opcional)</Label>
					<Input
						id="edit-market-image-url"
						placeholder="https://exemplo.com/imagem.jpg"
						bind:value={imageUrl}
						disabled={loading}
					/>
				</div>
			</div>

			<Dialog.Footer class="mt-5">
				<Dialog.Close>
					<Button type="button" variant="outline" disabled={loading}>Cancelar</Button>
				</Dialog.Close>
				<Button type="submit" disabled={loading}>
					{#if loading}
						A guardar...
					{:else}
						Guardar
					{/if}
				</Button>
			</Dialog.Footer>
		</form>
	</Dialog.Content>
</Dialog.Root>
