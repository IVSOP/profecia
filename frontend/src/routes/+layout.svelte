<script lang="ts">
	import './layout.css';
	import favicon from '$lib/assets/favicon.svg';
	import { enhance } from '$app/forms';
	import { page } from '$app/stores';
	import { Button } from '$lib/components/ui/button';
	import * as DropdownMenu from '$lib/components/ui/dropdown-menu';

	let { children } = $props();

	const user = $derived($page.data.user);
</script>

<svelte:head><link rel="icon" href={favicon} /></svelte:head>

<div class="flex min-h-screen flex-col bg-background">
	<header class="w-full border-b">
		<div class="container mx-auto px-4 py-2">
			<div class="flex items-center justify-between">
				<a href="/" class="text-xl font-bold">Profecia</a>
				{#if user}
					<DropdownMenu.Root>
						<DropdownMenu.Trigger>
							<Button variant="ghost" class="gap-2">
								{user.username}
							</Button>
						</DropdownMenu.Trigger>
						<DropdownMenu.Content align="end">
							<form method="POST" action="/logout" use:enhance>
								<DropdownMenu.Item>
									<button type="submit" class="w-full text-left">Sair</button>
								</DropdownMenu.Item>
							</form>
						</DropdownMenu.Content>
					</DropdownMenu.Root>
				{:else}
					<div class="flex items-center gap-2">
						<Button variant="ghost" href="/login">Entrar</Button>
						<Button href="/register">Criar conta</Button>
					</div>
				{/if}
			</div>
		</div>
	</header>
	<main class="container mx-auto flex-1 px-4 py-5">
		{@render children()}
	</main>
</div>
