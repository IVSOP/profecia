import { error, fail } from '@sveltejs/kit';
import type { Actions, PageServerLoad } from './$types';
import type { BuyOrderDto, InfoResponse, MarketOption, PositionDto } from '$lib/types';

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

	if (locals.user) {
		const posResponse = await fetch(`/api/event/position/${params.id}`);
		if (posResponse.ok) {
			positions = (await posResponse.json()) as PositionDto[];
		}

		// Fetch buy orders for each market, filter by current user
		const orderPromises = payload.event.markets.map(async (market) => {
			const res = await fetch(`/api/event/buyorder/${market.id}`);
			if (!res.ok) return [];
			const orders = (await res.json()) as BuyOrderDto[];
			return orders.filter((o) => o.userId === locals.user!.id);
		});

		buyOrders = (await Promise.all(orderPromises)).flat();
	}

	return { event: payload.event, positions, buyOrders };
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
	}
} satisfies Actions;
