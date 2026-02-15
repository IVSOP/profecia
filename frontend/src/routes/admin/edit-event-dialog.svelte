<script lang="ts">
	import { Button } from '$lib/components/ui/button';
	import * as Dialog from '$lib/components/ui/dialog';
	import { Input } from '$lib/components/ui/input';
	import { Label } from '$lib/components/ui/label';
	import type { EventDto, UpdateEventRequest } from '$lib/types';
	import ErrorBanner from './error-banner.svelte';

	interface Props {
		open: boolean;
		event: EventDto | null;
		onupdated: (event: EventDto) => void;
	}

	let { open = $bindable(false), event, onupdated }: Props = $props();

	let loading = $state(false);
	let error = $state('');
	let title = $state('');
	let imageUrl = $state('');

	$effect(() => {
		if (event && open) {
			title = event.displayName;
			imageUrl = event.imageUrl ?? '';
		}
	});

	function reset() {
		title = '';
		imageUrl = '';
		error = '';
		loading = false;
	}

	async function handleSubmit(e: SubmitEvent) {
		e.preventDefault();
		if (!event) return;
		error = '';

		if (!title.trim()) {
			error = 'O título do evento é obrigatório.';
			return;
		}

		loading = true;

		const payload: UpdateEventRequest = {
			displayName: title.trim(),
			imageUrl: imageUrl.trim() || null
		};

		try {
			const response = await fetch(`/api/event/${event.id}`, {
				method: 'PATCH',
				headers: { 'Content-Type': 'application/json' },
				body: JSON.stringify(payload)
			});

			if (!response.ok) {
				const data = await response.json().catch(() => null);
				error = data?.error || 'Erro ao atualizar evento. Tente novamente.';
				loading = false;
				return;
			}

			const updated = (await response.json()) as EventDto;
			onupdated(updated);
			open = false;
			reset();
		} catch {
			error = 'Erro ao atualizar evento. Tente novamente.';
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
			<Dialog.Title>Editar Evento</Dialog.Title>
			<Dialog.Description>
				Atualize as informações do evento.
			</Dialog.Description>
		</Dialog.Header>
		<form onsubmit={handleSubmit}>
			<div class="grid gap-5">
				<ErrorBanner message={error} />

				<div class="grid gap-2">
					<Label for="edit-event-title">Título do Evento</Label>
					<Input
						id="edit-event-title"
						placeholder="Ex: Final da Champions League 2026"
						bind:value={title}
						disabled={loading}
						required
					/>
				</div>

				<div class="grid gap-2">
					<Label for="edit-event-image-url">Imagem do Evento (URL, opcional)</Label>
					<Input
						id="edit-event-image-url"
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
