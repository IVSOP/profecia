<script lang="ts">
	import { Button } from '$lib/components/ui/button';
	import * as Dialog from '$lib/components/ui/dialog';
	import { Input } from '$lib/components/ui/input';
	import { Label } from '$lib/components/ui/label';
	import { Separator } from '$lib/components/ui/separator';
	import type { CreateEventRequest, CreateMarketRequest, EventDto } from '$lib/types';
	import ErrorBanner from './error-banner.svelte';
	import MarketFields from './market-fields.svelte';

	interface Props {
		open: boolean;
		oncreated: (event: EventDto) => void;
	}

	let { open = $bindable(false), oncreated }: Props = $props();

	let loading = $state(false);
	let error = $state('');
	let title = $state('');
	let eventImageUrl = $state('');
	let markets = $state<(CreateMarketRequest & { imageUrl: string })[]>([
		{ displayName: '', imageUrl: '', optionAName: 'Sim', optionBName: 'Não', rules: '' }
	]);

	function addMarket() {
		markets.push({ displayName: '', imageUrl: '', optionAName: 'Sim', optionBName: 'Não', rules: '' });
	}

	function removeMarket(index: number) {
		if (markets.length > 1) {
			markets.splice(index, 1);
		}
	}

	function resetForm() {
		title = '';
		eventImageUrl = '';
		markets = [{ displayName: '', imageUrl: '', optionAName: 'Sim', optionBName: 'Não', rules: '' }];
		error = '';
		loading = false;
	}

	async function handleSubmit(e: SubmitEvent) {
		e.preventDefault();
		error = '';

		if (!title.trim()) {
			error = 'O título do evento é obrigatório.';
			return;
		}

		for (let i = 0; i < markets.length; i++) {
			const m = markets[i];
			if (!m.displayName.trim() || !m.optionAName.trim() || !m.optionBName.trim()) {
				error = `Preencha todos os campos do mercado ${i + 1}.`;
				return;
			}
		}

		loading = true;

		const payload: CreateEventRequest = {
			displayName: title.trim(),
			imageUrl: eventImageUrl.trim() || undefined,
			markets: markets.map((m) => ({
				displayName: m.displayName.trim(),
				imageUrl: m.imageUrl.trim() || undefined,
				optionAName: m.optionAName.trim(),
				optionBName: m.optionBName.trim(),
				rules: m.rules.trim()
			}))
		};

		try {
			const response = await fetch('/api/event', {
				method: 'POST',
				headers: { 'Content-Type': 'application/json' },
				body: JSON.stringify(payload)
			});

			if (!response.ok) {
				const data = await response.json().catch(() => null);
				error = data?.error || 'Erro ao criar evento. Tente novamente.';
				loading = false;
				return;
			}

			const created = (await response.json()) as EventDto;
			oncreated(created);
			open = false;
			resetForm();
		} catch {
			error = 'Erro ao criar evento. Tente novamente.';
		} finally {
			loading = false;
		}
	}
</script>

<Dialog.Root
	bind:open
	onOpenChange={(v) => {
		if (!v) resetForm();
	}}
>
	<Dialog.Content class="max-h-[85vh] overflow-y-auto sm:max-w-2xl">
		<Dialog.Header>
			<Dialog.Title>Criar Evento</Dialog.Title>
			<Dialog.Description>
				Preencha os dados do evento e adicione os mercados desejados.
			</Dialog.Description>
		</Dialog.Header>
		<form onsubmit={handleSubmit}>
			<div class="grid gap-5">
				<ErrorBanner message={error} />

				<div class="grid gap-2">
					<Label for="event-title">Título do Evento</Label>
					<Input
						id="event-title"
						placeholder="Ex: Final da Champions League 2026"
						bind:value={title}
						disabled={loading}
						required
					/>
				</div>

				<div class="grid gap-2">
					<Label for="event-image-url">Imagem do Evento (URL, opcional)</Label>
					<Input
						id="event-image-url"
						placeholder="https://exemplo.com/imagem.jpg"
						bind:value={eventImageUrl}
						disabled={loading}
					/>
				</div>

				<Separator />

				<div class="flex items-center justify-between">
					<h4 class="text-sm font-medium">Mercados</h4>
					<Button type="button" variant="outline" size="sm" onclick={addMarket} disabled={loading}>
						+ Adicionar Mercado
					</Button>
				</div>

				{#each markets as market, i}
					<MarketFields
						bind:market={markets[i]}
						index={i}
						total={markets.length}
						disabled={loading}
						onremove={() => removeMarket(i)}
					/>
				{/each}
			</div>

			<Dialog.Footer class="mt-5">
				<Dialog.Close>
					<Button type="button" variant="outline" disabled={loading}>Cancelar</Button>
				</Dialog.Close>
				<Button type="submit" disabled={loading}>
					{#if loading}
						A criar...
					{:else}
						Criar Evento
					{/if}
				</Button>
			</Dialog.Footer>
		</form>
	</Dialog.Content>
</Dialog.Root>
