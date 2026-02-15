<script lang="ts">
	import { page } from '$app/state';
	import * as Table from '$lib/components/ui/table';
	import * as Avatar from '$lib/components/ui/avatar/index.js';
	import { Trophy, MedalIcon } from '@lucide/svelte';
	import type { PageProps } from './$types';

	let { data }: PageProps = $props();

	const currentUser = $derived(page.data.user);

	function formatDollars(cents: number): string {
		return '$' + (Math.abs(cents) / 100).toFixed(2);
	}

	function formatProfit(cents: number): string {
		if (cents > 0) return '+' + formatDollars(cents);
		if (cents < 0) return '-' + formatDollars(cents);
		return formatDollars(0);
	}
</script>

<svelte:head>
	<title>Leaderboard - Profecia</title>
</svelte:head>

<div>
	<div class="mb-6 flex items-center gap-3">
		<Trophy class="h-7 w-7 text-yellow-500" />
		<h1 class="text-2xl font-bold">Leaderboard</h1>
	</div>

	{#if data.entries.length > 0}
		<div class="rounded-lg border bg-card text-card-foreground">
			<Table.Root
				class="[&_td]:px-5 [&_th]:px-5"
			>
				<Table.Header>
					<Table.Row>
						<Table.Head class="w-12">#</Table.Head>
						<Table.Head>Utilizador</Table.Head>
						<Table.Head class="text-end">Lucro</Table.Head>
					</Table.Row>
				</Table.Header>
				<Table.Body>
					{#each data.entries as entry, i (entry.userId)}
						{@const rank = i + 1}
						{@const isCurrentUser = currentUser?.id === entry.userId}
						<Table.Row
							class={isCurrentUser
								? 'bg-primary/5 font-medium'
								: ''}
						>
							<Table.Cell class="tabular-nums">
								{#if rank === 1}
									<span class="text-lg">🥇</span>
								{:else if rank === 2}
									<span class="text-lg">🥈</span>
								{:else if rank === 3}
									<span class="text-lg">🥉</span>
								{:else}
									<span class="text-muted-foreground">{rank}</span>
								{/if}
							</Table.Cell>
							<Table.Cell>
								<div class="flex items-center gap-2.5">
									<Avatar.Root class="h-7 w-7 ring-1 ring-border">
										<Avatar.Fallback
											class="justify-center bg-muted text-xs text-muted-foreground"
										>
											{entry.username.slice(0, 2).toUpperCase()}
										</Avatar.Fallback>
									</Avatar.Root>
									<span class={isCurrentUser ? 'font-semibold' : 'font-medium'}>
										{entry.username}
										{#if isCurrentUser}
											<span class="ml-1 text-xs text-muted-foreground">(tu)</span>
										{/if}
									</span>
								</div>
							</Table.Cell>
							<Table.Cell class="text-end font-semibold tabular-nums">
								<span
									class={entry.realizedProfitCents > 0
										? 'text-green-600 dark:text-green-400'
										: entry.realizedProfitCents < 0
											? 'text-red-600 dark:text-red-400'
											: ''}
								>
									{formatProfit(entry.realizedProfitCents)}
								</span>
							</Table.Cell>
						</Table.Row>
					{/each}
				</Table.Body>
			</Table.Root>
		</div>
	{:else}
		<div class="rounded-lg border bg-card p-8 text-center text-muted-foreground">
			<p class="text-lg font-medium">Sem dados no leaderboard</p>
			<p class="mt-1 text-sm">Ainda ninguém fez apostas.</p>
		</div>
	{/if}
</div>
