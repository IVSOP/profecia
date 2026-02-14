import { redirect } from '@sveltejs/kit';
import type { Actions } from './$types';

export const actions = {
	default: async ({ cookies, fetch }) => {
		await fetch('/api/user/logout', { method: 'POST' }).catch(() => {});

		cookies.delete('sessionId', { path: '/' });

		redirect(303, '/login');
	}
} satisfies Actions;
