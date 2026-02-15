import type { PageServerLoad } from './$types';
import type { LeaderboardResponse } from '$lib/types';

export const load: PageServerLoad = async ({ fetch }) => {
	const res = await fetch('/api/user/leaderboard');

	let entries: LeaderboardResponse['entries'] = [];
	if (res.ok) {
		const data = (await res.json()) as LeaderboardResponse;
		entries = data.entries;
	}

	return { entries };
};
