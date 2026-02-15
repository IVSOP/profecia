<script lang="ts">
	import { Button } from '$lib/components/ui/button';
	import * as Dialog from '$lib/components/ui/dialog';
	import type { CreateMarketRequest, EventDto, MarketDto } from '$lib/types';
	import ErrorBanner from './error-banner.svelte';
	import MarketFields from './market-fields.svelte';

	interface Props {
		open: boolean;
		event: EventDto | null;
		oncreated: (eventId: string, market: MarketDto) => void;
	}

	let { open = $bindable(false), event, oncreated }: Props = $props();

	let loading = $state(false);
	let error = $state('');
	let market = $state<CreateMarketRequest & { imageUrl: string }>({
		displayName: '',
		imageUrl: '',
		optionAName: 'Sim',
		optionBName: 'Não',
		rules: ''
	});

	function reset() {
		market = { displayName: '', imageUrl: '', optionAName: 'Sim', optionBName: 'Não', rules: '' };
		error = '';
		loading = false;
	}

	async function handleSubmit(e: SubmitEvent) {
		e.preventDefault();
		if (!event) return;
		error = '';

		if (!market.displayName.trim() || !market.optionAName.trim() || !market.optionBName.trim()) {
			error = 'Preencha todos os campos obrigatórios.';
			return;
		}

		loading = true;

		const payload: CreateMarketRequest = {
			displayName: market.displayName.trim(),
			imageUrl: market.imageUrl.trim() || undefined,
			optionAName: market.optionAName.trim(),
			optionBName: market.optionBName.trim(),
			rules: market.rules.trim()
		};

		try {
			const response = await fetch(`/api/event/${event.id}/market`, {
				method: 'POST',
				headers: { 'Content-Type': 'application/json' },
				body: JSON.stringify(payload)
			});

			if (!response.ok) {
				const data = await response.json().catch(() => null);
				error = data?.error || 'Erro ao criar mercado. Tente novamente.';
				loading = false;
				return;
			}

			const created = (await response.json()) as MarketDto;
			oncreated(event.id, created);
			open = false;
			reset();
		} catch {
			error = 'Erro ao criar mercado. Tente novamente.';
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
			<Dialog.Title>Adicionar Mercado</Dialog.Title>
			<Dialog.Description>
				{#if event}
					Adicione um novo mercado ao evento <strong>{event.displayName}</strong>.
				{/if}
			</Dialog.Description>
		</Dialog.Header>
		<form onsubmit={handleSubmit}>
			<div class="grid gap-4">
				<ErrorBanner message={error} />

				<MarketFields
					bind:market
					index={0}
					total={1}
					disabled={loading}
					onremove={() => {}}
				/>
			</div>

			<Dialog.Footer class="mt-5">
				<Dialog.Close>
					<Button type="button" variant="outline" disabled={loading}>Cancelar</Button>
				</Dialog.Close>
				<Button type="submit" disabled={loading}>
					{#if loading}
						A criar...
					{:else}
						Criar Mercado
					{/if}
				</Button>
			</Dialog.Footer>
		</form>
	</Dialog.Content>
</Dialog.Root>
