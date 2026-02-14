import { fail, redirect } from '@sveltejs/kit';
import type { Actions } from './$types';
import type { AuthResponse } from '$lib/types';

export const actions = {
	default: async ({ request, cookies, fetch }) => {
		const formData = await request.formData();
		const username = formData.get('username')?.toString() ?? '';
		const password = formData.get('password')?.toString() ?? '';

		if (!username || !password) {
			return fail(400, { username, error: 'Preencha todos os campos.' });
		}

		const response = await fetch(`/api/user/login`, {
			method: 'POST',
			headers: { 'Content-Type': 'application/json' },
			body: JSON.stringify({ username, password })
		});

		if (!response.ok) {
			const data = await response.json().catch(() => null);
			return fail(400, {
				username,
				error: data?.error || 'Credenciais inválidas. Tente novamente.'
			});
		}

		const data = (await response.json()) as AuthResponse;

		cookies.set('sessionId', data.sessionId, {
			path: '/',
			maxAge: 60 * 60 * 24 * 7,
			sameSite: 'lax',
			httpOnly: false
		});

		redirect(303, '/');
	}
} satisfies Actions;
