<script lang="ts">
	import { enhance } from '$app/forms';
	import { Button } from '$lib/components/ui/button';
	import { Input } from '$lib/components/ui/input';
	import { Label } from '$lib/components/ui/label';
	import * as Dialog from '$lib/components/ui/dialog';
	import type { MarketDto, MarketPercentagesDto, UserDto } from '$lib/types';

	interface Props {
		open: boolean;
		market: MarketDto | null;
		option: 'A' | 'B';
		percentages: MarketPercentagesDto | null;
		user: UserDto | null;
		balanceCents: number | null;
	}

	let {
		open = $bindable(false),
		market,
		option = $bindable('A'),
		percentages,
		user,
		balanceCents
	}: Props = $props();

	let shares = $state(1);
	let priceInput = $state('50');
	let submitting = $state(false);
	let errorMessage = $state('');

	function getPriceForOption(opt: 'A' | 'B'): string {
		if (!percentages) return '50';
		const pct = opt === 'A' ? percentages.optionAPercentage : percentages.optionBPercentage;
		return pct != null ? String(pct) : '50';
	}

	// When the dialog opens, set initial price based on the selected option
	$effect(() => {
		if (open) {
			priceInput = getPriceForOption(option);
		}
	});

	function switchOption(opt: 'A' | 'B') {
		option = opt;
		priceInput = getPriceForOption(opt);
	}

	const pricePerShare = $derived(parseInt(priceInput, 10));
	const priceValid = $derived(!isNaN(pricePerShare) && pricePerShare >= 1 && pricePerShare <= 99);

	const optionName = $derived(
		market ? (option === 'A' ? market.optionAName : market.optionBName) : ''
	);

	const totalCostCents = $derived(priceValid ? shares * pricePerShare : 0);
	const totalGainCents = $derived(shares * 100);
	const profitCents = $derived(totalGainCents - totalCostCents);

	function formatDollars(cents: number): string {
		return '$' + (cents / 100).toFixed(2);
	}

	const isValid = $derived(shares > 0 && priceValid);
	const hasEnoughBalance = $derived(balanceCents != null && totalCostCents <= balanceCents);
	const canSubmit = $derived(isValid && hasEnoughBalance);

	function handlePriceInput(e: Event) {
		const input = e.target as HTMLInputElement;
		const raw = input.value;

		// Allow empty field
		if (raw === '') {
			priceInput = '';
			return;
		}

		let val = parseInt(raw, 10);
		if (isNaN(val) || val < 0) {
			priceInput = '';
			input.value = '';
			return;
		}

		if (val > 99) val = 99;

		priceInput = String(val);
		input.value = String(val);
	}

	function resetForm() {
		shares = 1;
		priceInput = '50';
		errorMessage = '';
	}
</script>

<Dialog.Root
	bind:open
	onOpenChange={(v) => {
		if (!v) resetForm();
	}}
>
	<Dialog.Content class="sm:max-w-md">
		<Dialog.Header>
			<Dialog.Title>
				{market?.displayName ?? ''}
			</Dialog.Title>
			<Dialog.Description>Criar ordem de compra</Dialog.Description>
		</Dialog.Header>

		{#if user}
			<form
				method="POST"
				action="?/buyorder"
				class="space-y-4"
				use:enhance={() => {
					submitting = true;
					errorMessage = '';

					return async ({ result, update }) => {
						submitting = false;

						if (result.type === 'success') {
							open = false;
							resetForm();
							window.dispatchEvent(new CustomEvent('balance:refresh'));
							await update();
						} else if (result.type === 'failure') {
							const data = result.data as { error?: string } | undefined;
							errorMessage = data?.error ?? 'Erro ao criar a ordem de compra.';
						} else {
							errorMessage = 'Erro inesperado. Tenta novamente.';
						}
					};
				}}
			>
				<input type="hidden" name="marketId" value={market?.id ?? ''} />
				<input type="hidden" name="option" value={option === 'A' ? 'optionA' : 'optionB'} />

				<!-- Option toggle -->
				{#if market}
					<div class="grid grid-cols-2 gap-2">
						<button
							type="button"
							class="rounded-lg border-2 px-3 py-2.5 text-sm font-semibold transition-colors {option ===
							'A'
								? 'border-green-600 bg-green-600 text-white'
								: 'border-border bg-muted/50 text-muted-foreground hover:bg-muted'}"
							onclick={() => switchOption('A')}
						>
							{market.optionAName}
							{percentages?.optionAPercentage != null ? `${percentages.optionAPercentage}¢` : '—'}
						</button>
						<button
							type="button"
							class="rounded-lg border-2 px-3 py-2.5 text-sm font-semibold transition-colors {option ===
							'B'
								? 'border-red-600 bg-red-600 text-white'
								: 'border-border bg-muted/50 text-muted-foreground hover:bg-muted'}"
							onclick={() => switchOption('B')}
						>
							{market.optionBName}
							{percentages?.optionBPercentage != null ? `${percentages.optionBPercentage}¢` : '—'}
						</button>
					</div>
				{/if}

				<div class="space-y-2">
					<Label for="price">Preço por ação</Label>
					<div class="relative">
						<Input
							id="price"
							name="pricePerShare"
							type="number"
							min={1}
							max={99}
							step={1}
							placeholder="0"
							value={priceInput}
							oninput={handlePriceInput}
							disabled={submitting}
							class="pr-8"
						/>
						<span
							class="pointer-events-none absolute top-1/2 right-3 -translate-y-1/2 text-sm text-muted-foreground"
						>
							¢
						</span>
					</div>
				</div>

				<div class="space-y-2">
					<Label for="shares">Quantidade de ações</Label>
					<Input
						id="shares"
						name="shares"
						type="number"
						min="1"
						step="1"
						placeholder="1"
						bind:value={shares}
						disabled={submitting}
					/>
				</div>

				<div>
					<p class="mb-3 text-sm font-medium">
						Se <span class="font-bold">{optionName}</span> for a opção correta:
					</p>
					<div class="flex items-stretch gap-2">
						<div
							class="flex flex-1 flex-col items-center justify-center rounded-lg border bg-muted/50 p-3 text-center"
						>
							<p class="text-xs text-muted-foreground">Custo</p>
							<p class="text-lg font-bold">{formatDollars(totalCostCents)}</p>
						</div>
						<div class="flex shrink-0 items-center">
							<span class="text-xl text-muted-foreground">→</span>
						</div>
						<div
							class="flex flex-1 flex-col items-center justify-center rounded-lg border border-green-600/20 bg-green-600/10 p-3 text-center"
						>
							<p class="text-xs text-muted-foreground">Recebes</p>
							<p class="text-lg font-bold">{formatDollars(totalGainCents)}</p>
							<p class="text-xs font-medium text-green-600">
								+{formatDollars(profitCents)} de lucro
							</p>
						</div>
					</div>
				</div>

				{#if isValid && !hasEnoughBalance}
					<div
						class="rounded-md border border-amber-500/50 bg-amber-500/10 px-3 py-2 text-sm text-amber-700 dark:text-amber-400"
					>
						Saldo insuficiente. Precisas de {formatDollars(totalCostCents)} mas tens {formatDollars(
							balanceCents ?? 0
						)}.
					</div>
				{/if}

				{#if errorMessage}
					<div
						class="rounded-md border border-destructive/50 bg-destructive/10 px-3 py-2 text-sm text-destructive"
					>
						{errorMessage}
					</div>
				{/if}

				<div class="grid grid-cols-2 gap-2">
					<Button
						type="button"
						variant="outline"
						class="w-full"
						disabled={submitting}
						onclick={() => {
							open = false;
							resetForm();
						}}>Cancelar</Button
					>
					<Button
						type="submit"
						class="w-full {option === 'A'
							? 'bg-green-600 text-white hover:bg-green-700'
							: 'bg-red-600 text-white hover:bg-red-700'}"
						disabled={!canSubmit || submitting}
					>
						{#if submitting}
							A enviar...
						{:else}
							Confirmar compra
						{/if}
					</Button>
				</div>
			</form>
		{:else}
			<div class="space-y-4 py-4">
				<p class="text-center text-sm text-muted-foreground">
					Tens de ter sessão iniciada para criar ordens de compra.
				</p>
				<Button href="/login" class="w-full">Iniciar sessão</Button>
			</div>
		{/if}
	</Dialog.Content>
</Dialog.Root>
