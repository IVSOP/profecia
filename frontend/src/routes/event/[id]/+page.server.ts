import { error, fail } from '@sveltejs/kit';
import type { Actions, PageServerLoad } from './$types';
import type { BuyOrderDto, EventPercentagesResponse, InfoResponse, MarketOption, MarketPercentagesDto, PositionDto } from '$lib/types';

export const load: PageServerLoad = async ({ fetch, params, locals }) => {
	const response = await fetch(`/api/event/${params.id}`);

	if (!response.ok) {
		error(404, 'Evento não encontrado');
	}

	const payload = (await response.json()) as InfoResponse;

	if (!payload.event) {
		error(404, 'Evento não encontrado');
	}

	let positions: PositionDto[] = [];
	let buyOrders: BuyOrderDto[] = [];

	// Fetch percentages for all markets in this event
	let marketPercentages: Record<string, MarketPercentagesDto> = {};
	const pctResponse = await fetch(`/api/event/percentages/${params.id}`);
	if (pctResponse.ok) {
		const response = (await pctResponse.json()) as EventPercentagesResponse;
		marketPercentages = response.percentages;
	}

	// Fetch ALL buy orders for each market (for the order book)
	const allOrderPromises = payload.event.markets.map(async (market) => {
		const res = await fetch(`/api/event/buyorder/${market.id}`);
		if (!res.ok) return { marketId: market.id, orders: [] as BuyOrderDto[] };
		const orders = (await res.json()) as BuyOrderDto[];
		return { marketId: market.id, orders };
	});
	const allOrderResults = await Promise.all(allOrderPromises);
	const allMarketOrders: Record<string, BuyOrderDto[]> = {};
	for (const { marketId, orders } of allOrderResults) {
		allMarketOrders[marketId] = orders;
	}

	if (locals.user) {
		const posResponse = await fetch(`/api/event/position/${params.id}`);
		if (posResponse.ok) {
			positions = (await posResponse.json()) as PositionDto[];
		}

		// Filter user's buy orders from the already-fetched data
		buyOrders = Object.values(allMarketOrders)
			.flat()
			.filter((o) => o.userId === locals.user!.id);
	}

	return { event: payload.event, positions, buyOrders, allMarketOrders, marketPercentages };
};

export const actions = {
	buyorder: async ({ request, fetch, locals }) => {
		if (!locals.user) {
			return fail(401, { error: 'Tens de ter sessão iniciada.' });
		}

		const formData = await request.formData();
		const marketId = formData.get('marketId')?.toString() ?? '';
		const shares = parseInt(formData.get('shares')?.toString() ?? '0', 10);
		const pricePerShare = parseInt(formData.get('pricePerShare')?.toString() ?? '0', 10);
		const optionRaw = formData.get('option')?.toString() ?? '';

		if (!marketId || shares < 1 || pricePerShare < 1 || pricePerShare > 99) {
			return fail(400, { error: 'Dados inválidos.' });
		}

		if (optionRaw !== 'optionA' && optionRaw !== 'optionB') {
			return fail(400, { error: 'Opção inválida.' });
		}

		const option: MarketOption = optionRaw;

		const response = await fetch('/api/event/buyorder', {
			method: 'POST',
			headers: { 'Content-Type': 'application/json' },
			body: JSON.stringify({
				marketId,
				userId: locals.user.id,
				shares,
				pricePerShare,
				option
			})
		});

		if (!response.ok) {
			const text = await response.text();
			return fail(response.status, { error: text || 'Erro ao criar a ordem de compra.' });
		}

		return { success: true };
	},

	cancelorder: async ({ request, fetch, locals }) => {
		if (!locals.user) {
			return fail(401, { error: 'Tens de ter sessão iniciada.' });
		}

		const formData = await request.formData();
		const orderId = formData.get('orderId')?.toString() ?? '';

		if (!orderId) {
			return fail(400, { error: 'Ordem inválida.' });
		}

		const response = await fetch(`/api/event/buyorder/cancel/${orderId}`, {
			method: 'POST'
		});

		if (!response.ok) {
			const text = await response.text();
			return fail(response.status, { error: text || 'Erro ao cancelar a ordem.' });
		}

		return { success: true };
	}
} satisfies Actions;
