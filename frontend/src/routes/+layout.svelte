<script lang="ts">
	import './layout.css';
	import favicon from '$lib/assets/favicon.svg';
	import { page } from '$app/state';
	import { Button } from '$lib/components/ui/button';
	import UserAvatar from '$lib/components/avatar.svelte';
	import DarkModeToggle from '$lib/components/dark-mode-toggle.svelte';
	import AirdropButton from '$lib/components/airdrop-button.svelte';
	import '@fontsource-variable/inter';
	import { ModeWatcher } from 'mode-watcher';
	import { Toaster } from '$lib/components/ui/sonner';
	import type { BalanceResponse } from '$lib/types';

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

	function formatBalance(cents: number): string {
		return '$' + (cents / 100).toFixed(2);
	}
</script>

<ModeWatcher />
<Toaster richColors closeButton />

<svelte:head><link rel="icon" href={favicon} /></svelte:head>

<div class="flex min-h-screen flex-col bg-background">
	<header class="w-full border-b">
		<div class="container mx-auto px-4 py-2">
			<div class="flex items-center justify-between">
				<a href="/" class="flex items-center gap-2 text-xl font-bold">
					<img src={favicon} alt="" class="h-7 w-7" aria-hidden="true" />
					Profecia
				</a>
				<div class="flex items-center gap-3">
					{#if user}
						<AirdropButton />
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
