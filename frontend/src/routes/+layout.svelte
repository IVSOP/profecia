<script lang="ts">
	import './layout.css';
	import favicon from '$lib/assets/favicon.svg';
	import { page } from '$app/state';
	import { Button } from '$lib/components/ui/button';
	import UserAvatar from '$lib/components/avatar.svelte';
	import DarkModeToggle from '$lib/components/dark-mode-toggle.svelte';
	import '@fontsource-variable/inter';
	import { ModeWatcher } from 'mode-watcher';

	let { children } = $props();

	const user = $derived(page.data.user);
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
