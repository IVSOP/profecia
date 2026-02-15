import { fail } from '@sveltejs/kit';
import type { Actions, PageServerLoad } from './$types';
import type { MarketOption } from '$lib/types';
import { fetchEventData } from '$lib/fetchers/event';

export const load: PageServerLoad = async ({ fetch, params, locals }) => {
	return fetchEventData(fetch, params.id, locals.user?.id);
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

		const data = await response.json().catch(() => null);
		const transactionUrls: string[] = data?.transactionUrls ?? [];

		return { success: true, transactionUrls };
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

		const data = await response.json().catch(() => null);
		const transactionUrls: string[] = data?.transactionUrls ?? [];

		return { success: true, transactionUrls };
	}
} satisfies Actions;
