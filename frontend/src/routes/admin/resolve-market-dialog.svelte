<script lang="ts">
	import * as AlertDialog from '$lib/components/ui/alert-dialog';
	import * as Select from '$lib/components/ui/select';
	import { Label } from '$lib/components/ui/label';
	import { toast } from 'svelte-sonner';
	import type { MarketDto, MarketOption } from '$lib/types';
	import ErrorBanner from './error-banner.svelte';

	interface Props {
		open: boolean;
		market: MarketDto | null;
		onresolved: (marketId: string, option: MarketOption) => void;
	}

	let { open = $bindable(false), market, onresolved }: Props = $props();

	let selectedOption = $state<MarketOption | undefined>(undefined);
	let loading = $state(false);
	let error = $state('');

	function reset() {
		selectedOption = undefined;
		loading = false;
		error = '';
	}

	async function handleSubmit(e: SubmitEvent) {
		e.preventDefault();
		if (!market || !selectedOption) return;

		loading = true;
		error = '';

		try {
			const response = await fetch(`/api/event/resolve/${market.id}`, {
				method: 'POST',
				headers: { 'Content-Type': 'application/json' },
				body: JSON.stringify({ option: selectedOption })
			});

			if (!response.ok) {
				const data = await response.json().catch(() => null);
				error = data?.error || 'Erro ao resolver mercado. Tente novamente.';
				loading = false;
				return;
			}

			const data = await response.json().catch(() => null);
			const urls: string[] = data?.transactionUrls ?? [];
			for (const url of urls) {
				toast.success('Transação na blockchain', {
					description: 'Clica para ver no explorador',
					action: {
						label: 'Abrir',
						onClick: () => window.open(url, '_blank')
					}
				});
			}

			onresolved(market.id, selectedOption);
			open = false;
			reset();
		} catch {
			error = 'Erro ao resolver mercado. Tente novamente.';
		} finally {
			loading = false;
		}
	}
</script>

<AlertDialog.Root
	bind:open
	onOpenChange={(v) => {
		if (!v) reset();
	}}
>
	<AlertDialog.Content>
		<form onsubmit={handleSubmit}>
			<AlertDialog.Header>
				<AlertDialog.Title>Resolver Mercado</AlertDialog.Title>
				<AlertDialog.Description>
					{#if market}
						Selecione o resultado correto para <strong>{market.displayName}</strong>. Esta ação
						é irreversível.
					{/if}
				</AlertDialog.Description>
			</AlertDialog.Header>

			{#if error}
				<ErrorBanner message={error} />
			{/if}

			{#if market}
				<div class="grid gap-2 py-2">
					<Label>Resultado</Label>
					<Select.Root
						type="single"
						onValueChange={(v) => {
							selectedOption = v as MarketOption;
						}}
					>
						<Select.Trigger class="w-full">
							{#if selectedOption === 'optionA'}
								{market.optionAName}
							{:else if selectedOption === 'optionB'}
								{market.optionBName}
							{:else}
								<span class="text-muted-foreground">Selecione o resultado...</span>
							{/if}
						</Select.Trigger>
						<Select.Content>
							<Select.Item value="optionA">{market.optionAName}</Select.Item>
							<Select.Item value="optionB">{market.optionBName}</Select.Item>
						</Select.Content>
					</Select.Root>
				</div>
			{/if}

			<AlertDialog.Footer>
				<AlertDialog.Cancel type="button" disabled={loading}>Cancelar</AlertDialog.Cancel>
				<button
					type="submit"
					disabled={!selectedOption || loading}
					class="inline-flex h-9 items-center justify-center gap-2 whitespace-nowrap rounded-md bg-primary px-4 py-2 text-sm font-medium text-primary-foreground shadow-xs transition-all hover:bg-primary/90 focus-visible:ring-[3px] focus-visible:ring-ring/50 focus-visible:outline-1 focus-visible:outline-ring disabled:pointer-events-none disabled:opacity-50"
				>
					{#if loading}
						A resolver...
					{:else}
						Confirmar
					{/if}
				</button>
			</AlertDialog.Footer>
		</form>
	</AlertDialog.Content>
</AlertDialog.Root>
