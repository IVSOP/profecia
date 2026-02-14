import type { AirdropStatusResponse, BalanceResponse } from '$lib/types';
import type { LayoutServerLoad } from './$types';

export const load: LayoutServerLoad = async ({ locals, fetch }) => {
	let balanceCents: number | null = null;
	let airdropSecondsLeft: number = 0;
	let airdropAvailable: boolean = false;

	if (locals.user) {
		const [balanceRes, airdropRes] = await Promise.all([
			fetch('/api/user/balance'),
			fetch('/api/user/airdrop')
		]);

		if (balanceRes.ok) {
			const data = (await balanceRes.json()) as BalanceResponse;
			balanceCents = data.balanceCents;
		}

		if (airdropRes.ok) {
			const data = (await airdropRes.json()) as AirdropStatusResponse;
			airdropSecondsLeft = data.secondsUntilAvailable;
			airdropAvailable = data.available;
		}
	}

	return {
		user: locals.user,
		balanceCents,
		airdropSecondsLeft,
		airdropAvailable
	};
};
