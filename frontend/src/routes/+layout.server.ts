import type { BalanceResponse } from '$lib/types';
import type { LayoutServerLoad } from './$types';

export const load: LayoutServerLoad = async ({ locals, fetch }) => {
	let balanceCents: number | null = null;

	if (locals.user) {
		const res = await fetch('/api/user/balance');
		if (res.ok) {
			const data = (await res.json()) as BalanceResponse;
			balanceCents = data.balanceCents;
		}
	}

	return {
		user: locals.user,
		balanceCents
	};
};
