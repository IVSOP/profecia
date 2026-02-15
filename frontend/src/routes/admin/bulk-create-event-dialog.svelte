<script lang="ts">
	import { Button } from '$lib/components/ui/button';
	import * as Dialog from '$lib/components/ui/dialog';
	import { Label } from '$lib/components/ui/label';
	import { Textarea } from '$lib/components/ui/textarea';
	import type { CreateEventRequest, EventDto } from '$lib/types';
	import ErrorBanner from './error-banner.svelte';

	interface Props {
		open: boolean;
		oncreated: (events: EventDto[]) => void;
	}

	let { open = $bindable(false), oncreated }: Props = $props();

	let loading = $state(false);
	let error = $state('');
	let jsonText = $state('');

	const placeholder = `[
  {
    "displayName": "Final da Champions League 2026",
    "imageUrl": "https://exemplo.com/imagem.jpg",
    "markets": [
      {
        "displayName": "Vencedor da partida",
        "optionAName": "Real Madrid",
        "optionBName": "Liverpool",
        "rules": "Mercado resolve com base no resultado após 90 minutos.",
        "imageUrl": ""
      }
    ]
  }
]`;

	function resetForm() {
		jsonText = '';
		error = '';
		loading = false;
	}

	function validatePayload(data: unknown): data is CreateEventRequest[] {
		if (!Array.isArray(data)) {
			error = 'O JSON deve ser um array de eventos.';
			return false;
		}

		for (let i = 0; i < data.length; i++) {
			const event = data[i];
			if (!event.displayName || typeof event.displayName !== 'string') {
				error = `Evento ${i + 1}: "displayName" é obrigatório.`;
				return false;
			}
			if (!Array.isArray(event.markets) || event.markets.length === 0) {
				error = `Evento ${i + 1}: deve ter ao menos um mercado.`;
				return false;
			}
			for (let j = 0; j < event.markets.length; j++) {
				const m = event.markets[j];
				if (!m.displayName || !m.optionAName || !m.optionBName) {
					error = `Evento ${i + 1}, Mercado ${j + 1}: "displayName", "optionAName" e "optionBName" são obrigatórios.`;
					return false;
				}
				if (typeof m.rules !== 'string') {
					error = `Evento ${i + 1}, Mercado ${j + 1}: "rules" deve ser uma string.`;
					return false;
				}
			}
		}

		return true;
	}

	async function handleSubmit(e: SubmitEvent) {
		e.preventDefault();
		error = '';

		if (!jsonText.trim()) {
			error = 'Cole o JSON com os eventos.';
			return;
		}

		let parsed: unknown;
		try {
			parsed = JSON.parse(jsonText);
		} catch {
			error = 'JSON inválido. Verifique a sintaxe.';
			return;
		}

		if (!validatePayload(parsed)) {
			return;
		}

		loading = true;

		try {
			const response = await fetch('/api/event/bulk', {
				method: 'POST',
				headers: { 'Content-Type': 'application/json' },
				body: JSON.stringify(parsed)
			});

			if (!response.ok) {
				const data = await response.json().catch(() => null);
				error = data?.error || 'Erro ao criar eventos. Tente novamente.';
				loading = false;
				return;
			}

			const created = (await response.json()) as EventDto[];
			oncreated(created);
			open = false;
			resetForm();
		} catch {
			error = 'Erro ao criar eventos. Tente novamente.';
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
			<Dialog.Title>Criar Eventos em Massa</Dialog.Title>
			<Dialog.Description>
				Cole um JSON com um array de eventos e seus mercados para criar vários eventos de uma vez.
			</Dialog.Description>
		</Dialog.Header>
		<form onsubmit={handleSubmit}>
			<div class="grid gap-4">
				<ErrorBanner message={error} />

				<div class="grid gap-2">
					<Label for="bulk-json">JSON dos Eventos</Label>
					<Textarea
						id="bulk-json"
						{placeholder}
						bind:value={jsonText}
						disabled={loading}
						class="font-mono text-xs min-h-[300px]"
					/>
				</div>
			</div>

			<Dialog.Footer class="mt-5">
				<Dialog.Close>
					<Button type="button" variant="outline" disabled={loading}>Cancelar</Button>
				</Dialog.Close>
				<Button type="submit" disabled={loading}>
					{#if loading}
						A criar...
					{:else}
						Criar Eventos
					{/if}
				</Button>
			</Dialog.Footer>
		</form>
	</Dialog.Content>
</Dialog.Root>
