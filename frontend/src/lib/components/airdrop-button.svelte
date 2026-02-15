<script lang="ts">
	import { page } from '$app/state';
	import { Button } from '$lib/components/ui/button';
	import { GiftIcon } from '@lucide/svelte';
	import * as Tooltip from '$lib/components/ui/tooltip';
	import type { AirdropStatusResponse } from '$lib/types';

	// The target date when airdrop becomes available (null = available now)
	let availableAt = $state<string | null>(page.data.airdropAvailableAt);
	let loading = $state(false);

	// Sync from server data on navigation
	$effect(() => {
		availableAt = page.data.airdropAvailableAt;
	});

	// Compute seconds left from the target date
	let now = $state(Date.now());

	$effect(() => {
		if (!availableAt) return;

		const tick = setInterval(() => {
			now = Date.now();
		}, 1000);

		return () => clearInterval(tick);
	});

	const secondsLeft = $derived.by(() => {
		if (!availableAt) return 0;
		const diff = Math.ceil((new Date(availableAt).getTime() - now) / 1000);
		return Math.max(0, diff);
	});

	const available = $derived(!availableAt || secondsLeft === 0);

	function formatCooldown(seconds: number): string {
		const m = Math.floor(seconds / 60);
		const s = seconds % 60;
		return `${m.toString().padStart(2, '0')}m${s.toString().padStart(2, '0')}s`;
	}

	async function requestAirdrop() {
		if (!available || loading) return;
		loading = true;

		try {
			const res = await fetch('/api/user/airdrop', { method: 'POST' });
			if (!res.ok) return;

			// Trigger balance refresh in parent layout
			window.dispatchEvent(new CustomEvent('balance:refresh'));

			const statusRes = await fetch('/api/user/airdrop');

			if (statusRes.ok) {
				const data = (await statusRes.json()) as AirdropStatusResponse;
				availableAt = data.nextAirdropAt ?? null;
				now = Date.now();
			}
		} finally {
			loading = false;
		}
	}
</script>

<Tooltip.Provider>
	<Tooltip.Root>
		<Tooltip.Trigger>
			{#snippet child({ props })}
				<Button
					{...props}
					variant="outline"
					size="icon"
					class={available && !loading
						? 'text-primary hover:text-primary/80 cursor-pointer'
						: 'text-muted-foreground/40 !opacity-50 !pointer-events-auto cursor-not-allowed'}
					onclick={requestAirdrop}
					aria-label={available ? 'Pedir airdrop' : `Airdrop disponível em ${formatCooldown(secondsLeft)}`}
				>
					<GiftIcon class="h-[1.2rem] w-[1.2rem]" />
				</Button>
			{/snippet}
		</Tooltip.Trigger>
		{#if !available && secondsLeft > 0}
			<Tooltip.Portal>
				<Tooltip.Content>
					{formatCooldown(secondsLeft)}
				</Tooltip.Content>
			</Tooltip.Portal>
		{/if}
	</Tooltip.Root>
</Tooltip.Provider>
