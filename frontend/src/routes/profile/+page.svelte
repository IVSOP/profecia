<script lang="ts">
	import { page } from '$app/state';
	import * as Card from '$lib/components/ui/card';
	import * as Table from '$lib/components/ui/table';
	import { Separator } from '$lib/components/ui/separator';
	import * as Avatar from '$lib/components/ui/avatar/index.js';
	import {
		TrendingUpIcon,
		TrendingDownIcon,
		WalletIcon,
		BarChart3Icon,
		Trophy,
		ExternalLinkIcon
	} from '@lucide/svelte';
	import type { MarketDto, MarketOption, PositionDto } from '$lib/types';
	import type { PageProps } from './$types';

	let { data }: PageProps = $props();

	const user = $derived(page.data.user);

	// Build lookup maps
	const marketsById = $derived.by(() => {
		const map = new Map<string, MarketDto>();
		for (const event of data.events) {
			for (const market of event.markets) {
				map.set(market.id, market);
			}
		}
		return map;
	});

	const eventByMarketId = $derived.by(() => {
		const map = new Map<string, (typeof data.events)[0]>();
		for (const event of data.events) {
			for (const market of event.markets) {
				map.set(market.id, event);
			}
		}
		return map;
	});

	function getOptionName(marketId: string, option: string): string {
		const market = marketsById.get(marketId);
		if (!market) return option;
		return option === 'optionA' ? market.optionAName : market.optionBName;
	}

	function getMarketName(marketId: string): string {
		return marketsById.get(marketId)?.displayName ?? 'Mercado desconhecido';
	}

	function getEventName(marketId: string): string {
		return eventByMarketId.get(marketId)?.displayName ?? '';
	}

	function getEventId(marketId: string): string {
		return eventByMarketId.get(marketId)?.id ?? '';
	}

	function formatDollars(cents: number): string {
		return '$' + (cents / 100).toFixed(2);
	}

	function isResolved(marketId: string): boolean {
		return marketsById.get(marketId)?.resolvedOption != null;
	}

	function isWinner(position: PositionDto): boolean {
		const market = marketsById.get(position.marketId);
		return market?.resolvedOption === position.option;
	}

	function resolvedResult(position: PositionDto): number {
		const won = isWinner(position);
		const payout = won ? position.shares * 100 : 0;
		return payout - position.shares * position.pricePerShare;
	}

	function unrealizedGain(position: PositionDto): number {
		// Expected return if option wins minus cost
		return position.shares * 100 - position.shares * position.pricePerShare;
	}

	// Compute totals
	const resolvedPositions = $derived(data.positions.filter((p) => isResolved(p.marketId)));
	const openPositions = $derived(data.positions.filter((p) => !isResolved(p.marketId)));

	const totalRealizedProfit = $derived(
		resolvedPositions.reduce((sum, p) => sum + resolvedResult(p), 0)
	);

	const totalUnrealizedGain = $derived(
		openPositions.reduce((sum, p) => sum + unrealizedGain(p), 0)
	);

	const totalInvested = $derived(
		openPositions.reduce((sum, p) => sum + p.shares * p.pricePerShare, 0)
	);

	// Sort: open positions first, then resolved
	const sortedPositions = $derived.by(() => {
		return [...data.positions].sort((a, b) => {
			const aResolved = isResolved(a.marketId);
			const bResolved = isResolved(b.marketId);
			if (aResolved !== bResolved) return aResolved ? 1 : -1;
			return 0;
		});
	});
</script>

<svelte:head>
	<title>Perfil - Profecia</title>
</svelte:head>

<div>
	<!-- Profile Header -->
	<div class="mb-8 flex items-center gap-4">
		<Avatar.Root class="h-16 w-16 ring-2 ring-border">
			<Avatar.Fallback class="justify-center bg-muted text-xl text-muted-foreground">
				{user?.username.slice(0, 2).toUpperCase()}
			</Avatar.Fallback>
		</Avatar.Root>
		<div class="flex items-center gap-4">
			<div>
				<h1 class="text-2xl font-bold">{user?.username}</h1>
				<p class="text-sm text-muted-foreground">O teu perfil</p>
			</div>
			{#if user?.solanaUrl}
				<a
					href={user.solanaUrl}
					target="_blank"
					rel="noopener noreferrer"
					class="inline-flex items-center gap-1.5 rounded-full border bg-muted/50 px-3 py-1 text-xs font-medium text-muted-foreground transition-colors hover:bg-muted hover:text-foreground"
				>
					<span class="inline-block h-2 w-2 rounded-full bg-green-500"></span>
					{user.pubkey.slice(0, 6)}...{user.pubkey.slice(-6)}
					<ExternalLinkIcon class="h-3 w-3" />
				</a>
			{/if}
		</div>
	</div>

	<!-- Stats Cards -->
	<div class="mb-8 grid grid-cols-1 gap-3 sm:grid-cols-2 lg:grid-cols-4">
		<Card.Root class="gap-0 py-3">
			<Card.Header class="flex flex-row items-center justify-between px-4 py-0 pb-1">
				<Card.Title class="text-sm font-medium text-muted-foreground">Saldo</Card.Title>
				<WalletIcon class="h-4 w-4 text-muted-foreground" />
			</Card.Header>
			<Card.Content class="px-4 py-0">
				<div class="text-xl font-bold">{formatDollars(data.balanceCents)}</div>
			</Card.Content>
		</Card.Root>

		<Card.Root class="gap-0 py-3">
			<Card.Header class="flex flex-row items-center justify-between px-4 py-0 pb-1">
				<Card.Title class="text-sm font-medium text-muted-foreground"
					>Lucro Realizado</Card.Title
				>
				{#if totalRealizedProfit > 0}
					<TrendingUpIcon class="h-4 w-4 text-green-600 dark:text-green-400" />
				{:else if totalRealizedProfit < 0}
					<TrendingDownIcon class="h-4 w-4 text-red-600 dark:text-red-400" />
				{/if}
			</Card.Header>
			<Card.Content class="px-4 py-0">
				<div
					class="text-xl font-bold {totalRealizedProfit > 0
						? 'text-green-600 dark:text-green-400'
						: totalRealizedProfit < 0
							? 'text-red-600 dark:text-red-400'
							: ''}"
				>
					{totalRealizedProfit > 0 ? '+' : totalRealizedProfit < 0 ? '-' : ''}{formatDollars(Math.abs(totalRealizedProfit))}
				</div>
				<p class="text-xs text-muted-foreground">
					{resolvedPositions.length} {resolvedPositions.length === 1
						? 'posição encerrada'
						: 'posições encerradas'}
				</p>
			</Card.Content>
		</Card.Root>

		<Card.Root class="gap-0 py-3">
			<Card.Header class="flex flex-row items-center justify-between px-4 py-0 pb-1">
				<Card.Title class="text-sm font-medium text-muted-foreground"
					>Não Realizado</Card.Title
				>
				{#if totalUnrealizedGain > 0}
					<TrendingUpIcon class="h-4 w-4 text-green-600 dark:text-green-400" />
				{:else if totalUnrealizedGain < 0}
					<TrendingDownIcon class="h-4 w-4 text-red-600 dark:text-red-400" />
				{/if}
			</Card.Header>
			<Card.Content class="px-4 py-0">
				<div
					class="text-xl font-bold {totalUnrealizedGain > 0
						? 'text-green-600 dark:text-green-400'
						: totalUnrealizedGain < 0
							? 'text-red-600 dark:text-red-400'
							: ''}"
				>
					{totalUnrealizedGain > 0 ? '+' : totalUnrealizedGain < 0 ? '-' : ''}{formatDollars(Math.abs(totalUnrealizedGain))}
				</div>
				<p class="text-xs text-muted-foreground">
					{openPositions.length} {openPositions.length === 1
						? 'posição aberta'
						: 'posições abertas'}
				</p>
			</Card.Content>
		</Card.Root>

		<Card.Root class="gap-0 py-3">
			<Card.Header class="flex flex-row items-center justify-between px-4 py-0 pb-1">
				<Card.Title class="text-sm font-medium text-muted-foreground"
					>Investido</Card.Title
				>
				<BarChart3Icon class="h-4 w-4 text-muted-foreground" />
			</Card.Header>
			<Card.Content class="px-4 py-0">
				<div class="text-xl font-bold">{formatDollars(totalInvested)}</div>
				<p class="text-xs text-muted-foreground">em posições abertas</p>
			</Card.Content>
		</Card.Root>
	</div>

	<!-- Positions Table -->
	{#if sortedPositions.length > 0}
		<div>
			<h2 class="mb-3 text-lg font-semibold">As tuas posições</h2>
			<div class="rounded-lg border bg-card text-card-foreground">
				<Table.Root
					class="[&_td]:px-5 [&_th]:px-5"
				>
					<Table.Header>
						<Table.Row>
							<Table.Head>Evento</Table.Head>
							<Table.Head>Mercado</Table.Head>
							<Table.Head>Opção</Table.Head>
							<Table.Head class="text-end">Ações</Table.Head>
							<Table.Head class="text-end">Preço</Table.Head>
							<Table.Head class="text-end">Total</Table.Head>
							<Table.Head class="text-end">Retorno</Table.Head>
						</Table.Row>
					</Table.Header>
					<Table.Body>
						{#each sortedPositions as position (position.id)}
							{@const resolved = isResolved(position.marketId)}
							{@const eventId = getEventId(position.marketId)}
							<Table.Row class={resolved ? 'opacity-70' : ''}>
								<Table.Cell class="max-w-[140px] truncate font-medium">
									{#if eventId}
										<a
											href="/event/{eventId}"
											class="hover:underline"
										>
											{getEventName(position.marketId)}
										</a>
									{:else}
										{getEventName(position.marketId)}
									{/if}
								</Table.Cell>
								<Table.Cell class="max-w-[140px] truncate">
									{getMarketName(position.marketId)}
								</Table.Cell>
								<Table.Cell>
									<span
										class="inline-flex items-center rounded-full px-2 py-0.5 text-xs font-semibold {position.option ===
										'optionA'
											? 'bg-green-600/15 text-green-700 dark:text-green-400'
											: 'bg-red-600/15 text-red-700 dark:text-red-400'}"
									>
										{getOptionName(position.marketId, position.option)}
									</span>
								</Table.Cell>
								<Table.Cell class="text-end font-medium tabular-nums">
									{position.shares}
								</Table.Cell>
								<Table.Cell class="text-end text-muted-foreground tabular-nums">
									{position.pricePerShare}¢
								</Table.Cell>
								<Table.Cell class="text-end font-semibold tabular-nums">
									{formatDollars(position.shares * position.pricePerShare)}
								</Table.Cell>
								<Table.Cell class="text-end font-semibold tabular-nums">
									{#if resolved}
										{@const result = resolvedResult(position)}
										<span
											class="inline-flex items-center gap-1 {result > 0
												? 'text-green-600 dark:text-green-400'
												: result < 0
													? 'text-red-600 dark:text-red-400'
													: ''}"
										>
											{#if result > 0}
												<Trophy class="h-3.5 w-3.5" />
											{/if}
											{result > 0 ? '+' : result < 0 ? '-' : ''}{formatDollars(Math.abs(result))}
										</span>
									{:else}
										{@const gain = unrealizedGain(position)}
										<span class="text-muted-foreground">
											{gain > 0 ? '+' : gain < 0 ? '-' : ''}{formatDollars(Math.abs(gain))}
										</span>
									{/if}
								</Table.Cell>
							</Table.Row>
						{/each}
					</Table.Body>
				</Table.Root>
			</div>
		</div>
	{:else}
		<div class="rounded-lg border bg-card p-8 text-center text-muted-foreground">
			<p class="text-lg font-medium">Ainda não tens posições</p>
			<p class="mt-1 text-sm">Começa a investir nos eventos disponíveis!</p>
			<a
				href="/"
				class="mt-4 inline-flex items-center rounded-md bg-primary px-4 py-2 text-sm font-medium text-primary-foreground transition-colors hover:bg-primary/90"
			>
				Ver eventos
			</a>
		</div>
	{/if}
</div>
