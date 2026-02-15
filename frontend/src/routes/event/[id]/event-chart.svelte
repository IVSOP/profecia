<script lang="ts">
	import * as Chart from '$lib/components/ui/chart/index.js';
	import { LineChart } from 'layerchart';
	import { scaleTime } from 'd3-scale';
	import { curveNatural } from 'd3-shape';
	import type { MarketDto, EventChartDto } from '$lib/types';

	interface Props {
		markets: MarketDto[];
		chartData: EventChartDto;
	}

	let { markets, chartData }: Props = $props();

	const MARKET_COLORS = [
		'#2563eb', // blue
		'#dc2626', // red
		'#16a34a', // green
		'#d97706', // amber
		'#7c3aed', // violet
		'#db2777', // pink
		'#0891b2', // cyan
		'#65a30d' // lime
	];

	// Transform backend data into chart-friendly format
	function buildChartData(
		marketList: MarketDto[],
		chart: EventChartDto
	): Record<string, unknown>[] {
		return chart.points.map((point) => {
			const row: Record<string, unknown> = {
				time: new Date(point.recordedAt)
			};
			for (const market of marketList) {
				row[market.id] = point.percentages[market.id] ?? null;
			}
			return row;
		});
	}

	const transformedData = $derived(buildChartData(markets, chartData));

	const chartConfig = $derived.by(() => {
		const config: Chart.ChartConfig = {};
		for (let i = 0; i < markets.length; i++) {
			const market = markets[i];
			config[market.id] = {
				label: market.displayName,
				color: MARKET_COLORS[i % MARKET_COLORS.length]
			};
		}
		return config;
	});

	const series = $derived(
		markets.map((market, i) => ({
			key: market.id,
			label: chartConfig[market.id]?.label ?? market.displayName,
			color: chartConfig[market.id]?.color ?? MARKET_COLORS[i % MARKET_COLORS.length]
		}))
	);

	const latestValues = $derived.by(() => {
		const last = transformedData[transformedData.length - 1];
		if (!last) return {};
		const values: Record<string, number> = {};
		for (const market of markets) {
			values[market.id] = last[market.id] as number;
		}
		return values;
	});
</script>

<div class="mt-6 mb-10">
	{#if markets.length > 1}
		<div class="mb-1 flex items-center gap-4">
			{#each series as s (s.key)}
				<div class="flex items-center gap-1.5">
					<span class="inline-block size-2.5 rounded-[2px]" style="background-color: {s.color};">
					</span>
					<span class="text-xs text-muted-foreground">
						{s.label}
						<span class="font-medium text-foreground"
							>{latestValues[s.key]?.toFixed(1) ?? '—'}%</span
						>
					</span>
				</div>
			{/each}
		</div>
	{/if}
	<Chart.Container config={chartConfig} class="h-[300px] w-full">
		<LineChart
			data={transformedData}
			x="time"
			xScale={scaleTime()}
			yDomain={[0, 100]}
			yNice={false}
			padding={{ left: 0, right: 40, top: 0, bottom: 0 }}
			axis={true}
			{series}
			props={{
				grid: {
					y: true,
					yTicks: [0, 25, 50, 75, 100]
				},
				spline: {
					curve: curveNatural,
					style: 'stroke-width: 3px;'
				},
				yAxis: {
					ticks: [0, 25, 50, 75, 100],
					format: (d: number) => `${d}%`,
					placement: 'right'
				},
				xAxis: {
					format: (d: Date | number) => {
						const date = d instanceof Date ? d : new Date(d);
						return `${date.getHours().toString().padStart(2, '0')}:${date.getMinutes().toString().padStart(2, '0')}`;
					},
					ticks: 6
				}
			}}
		>
			{#snippet tooltip()}
				<Chart.Tooltip
					labelFormatter={(value: unknown) => {
						if (value instanceof Date) {
							return `${value.getHours().toString().padStart(2, '0')}:${value.getMinutes().toString().padStart(2, '0')}`;
						}
						return `${value}`;
					}}
					formatter={tooltipFormatter}
				/>
			{/snippet}
		</LineChart>
	</Chart.Container>
</div>

{#snippet tooltipFormatter({
	value,
	name,
	item,
	index,
	payload
}: {
	value: unknown;
	name: string;
	item: import('$lib/components/ui/chart/chart-utils.js').TooltipPayload;
	index: number;
	payload: import('$lib/components/ui/chart/chart-utils.js').TooltipPayload[];
})}
	{@const indicatorColor = item.color}
	<div class="flex w-full flex-wrap items-center gap-2">
		<div
			style="--color-bg: {indicatorColor}; --color-border: {indicatorColor};"
			class="size-2.5 shrink-0 rounded-[2px] border-(--color-border) bg-(--color-bg)"
		></div>
		<div class="flex flex-1 items-center justify-between leading-none">
			<span class="text-muted-foreground">
				{chartConfig[name]?.label || name}
			</span>
			<span class="text-foreground tabular-nums">
				{value != null ? `${value}%` : '—'}
			</span>
		</div>
	</div>
{/snippet}
