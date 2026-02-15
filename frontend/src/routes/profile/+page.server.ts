import { redirect } from '@sveltejs/kit';
import type { PageServerLoad } from './$types';
import type {
	BalanceResponse,
	EventDto,
	ListResponse,
	MarketPercentagesDto,
	PositionDto,
	AllPercentagesResponse
} from '$lib/types';

export const load: PageServerLoad = async ({ locals, fetch }) => {
	if (!locals.user) {
		redirect(302, '/login');
	}

	const [positionsRes, eventsRes, balanceRes, pctRes] = await Promise.all([
		fetch('/api/user/positions'),
		fetch('/api/event'),
		fetch('/api/user/balance'),
		fetch('/api/event/percentages')
	]);

	let positions: PositionDto[] = [];
	if (positionsRes.ok) {
		positions = (await positionsRes.json()) as PositionDto[];
	}

	let events: EventDto[] = [];
	if (eventsRes.ok) {
		const payload = (await eventsRes.json()) as ListResponse;
		events = payload.events;
	}

	let balanceCents = 0;
	if (balanceRes.ok) {
		const data = (await balanceRes.json()) as BalanceResponse;
		balanceCents = data.balanceCents;
	}

	const allPercentages: Record<string, Record<string, MarketPercentagesDto>> = {};
	if (pctRes.ok) {
		const response = (await pctRes.json()) as AllPercentagesResponse;
		for (const [eventId, eventPct] of Object.entries(response.percentages)) {
			allPercentages[eventId] = eventPct.percentages;
		}
	}

	return { positions, events, balanceCents, allPercentages };
};
