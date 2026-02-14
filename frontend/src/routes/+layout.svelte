<script lang="ts">
	import './layout.css';
	import favicon from '$lib/assets/favicon.svg';
	import { page } from '$app/state';
	import { Button } from '$lib/components/ui/button';
	import UserAvatar from '$lib/components/avatar.svelte';
	import DarkModeToggle from '$lib/components/dark-mode-toggle.svelte';
	import '@fontsource-variable/inter';
	import { ModeWatcher } from 'mode-watcher';
	import { GiftIcon } from '@lucide/svelte';
	import type { AirdropStatusResponse, BalanceResponse } from '$lib/types';

	let { children } = $props();

	const user = $derived(page.data.user);

	// Reactive balance state: starts from server data, updated on airdrop
	let balanceOverride = $state<number | null>(null);
	const balanceCents = $derived(balanceOverride ?? page.data.balanceCents);

	// Reset override when page data changes (navigation)
	$effect(() => {
		page.data.balanceCents;
		balanceOverride = null;
	});

	// Listen for balance refresh events from child components
	$effect(() => {
		const handler = async () => {
			const res = await fetch('/api/user/balance');
			if (res.ok) {
				const data = (await res.json()) as BalanceResponse;
				balanceOverride = data.balanceCents;
			}
		};
		window.addEventListener('balance:refresh', handler);
		return () => window.removeEventListener('balance:refresh', handler);
	});

	// Airdrop cooldown state
	let secondsLeft = $state(0);
	let airdropAvailable = $state(false);
	let airdropLoading = $state(false);

	// Sync from server data on load/navigation
	$effect(() => {
		secondsLeft = page.data.airdropSecondsLeft;
		airdropAvailable = page.data.airdropAvailable;
	});

	// Live countdown timer
	$effect(() => {
		if (secondsLeft <= 0) {
			airdropAvailable = true;
			return;
		}

		airdropAvailable = false;
		const interval = setInterval(() => {
			secondsLeft--;
			if (secondsLeft <= 0) {
				secondsLeft = 0;
				airdropAvailable = true;
				clearInterval(interval);
			}
		}, 1000);

		return () => clearInterval(interval);
	});

	function formatBalance(cents: number): string {
		return '$' + (cents / 100).toFixed(2);
	}

	function formatCooldown(seconds: number): string {
		const m = Math.floor(seconds / 60);
		const s = seconds % 60;
		if (m > 0) return `${m}m ${s.toString().padStart(2, '0')}s`;
		return `${s}s`;
	}

	async function requestAirdrop() {
		if (!airdropAvailable || airdropLoading) return;
		airdropLoading = true;

		try {
			const res = await fetch('/api/user/airdrop', { method: 'POST' });
			if (!res.ok) return;

			// Fetch updated balance and airdrop status in parallel
			const [balanceRes, statusRes] = await Promise.all([
				fetch('/api/user/balance'),
				fetch('/api/user/airdrop')
			]);

			if (balanceRes.ok) {
				const data = (await balanceRes.json()) as BalanceResponse;
				balanceOverride = data.balanceCents;
			}

			if (statusRes.ok) {
				const data = (await statusRes.json()) as AirdropStatusResponse;
				secondsLeft = data.secondsUntilAvailable;
				airdropAvailable = data.available;
			}
		} finally {
			airdropLoading = false;
		}
	}
</script>

<ModeWatcher />

<svelte:head><link rel="icon" href={favicon} /></svelte:head>

<div class="flex min-h-screen flex-col bg-background">
	<header class="w-full border-b">
		<div class="container mx-auto px-4 py-2">
			<div class="flex items-center justify-between">
				<a href="/" class="text-xl font-bold">Profecia</a>
				<div class="flex items-center gap-2">
					{#if user}
						<div class="group relative">
						<Button
							variant="outline"
							size="icon"
							class={airdropAvailable && !airdropLoading
								? 'text-green-600 hover:text-green-700 dark:text-green-400 dark:hover:text-green-300 cursor-pointer'
								: 'text-muted-foreground/40 !opacity-50 !pointer-events-auto cursor-not-allowed'}
							onclick={requestAirdrop}
							aria-label={airdropAvailable ? 'Pedir airdrop' : `Airdrop disponível em ${formatCooldown(secondsLeft)}`}
						>
							<GiftIcon class="h-[1.2rem] w-[1.2rem]" />
						</Button>
						{#if !airdropAvailable && secondsLeft > 0}
								<div class="pointer-events-none absolute top-full left-1/2 z-50 mt-1.5 -translate-x-1/2 whitespace-nowrap rounded-md bg-popover px-2.5 py-1 text-xs font-medium text-popover-foreground shadow-md border opacity-0 group-hover:opacity-100 transition-opacity">
									{formatCooldown(secondsLeft)}
								</div>
							{/if}
						</div>
						{#if balanceCents != null}
							<span class="text-sm font-medium text-muted-foreground">
								{formatBalance(balanceCents)}
							</span>
						{/if}
						<UserAvatar username={user.username} />
					{:else}
						<Button variant="ghost" href="/login">Entrar</Button>
						<Button href="/register">Criar conta</Button>
					{/if}
					<DarkModeToggle />
				</div>
			</div>
		</div>
	</header>
	<main class="container mx-auto flex-1 px-4 py-5">
		{@render children?.()}
	</main>
</div>
