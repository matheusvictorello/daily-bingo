import { error } from '@sveltejs/kit';
import { goto } from '$app/navigation';

// ponytail: token in localStorage, move to httpOnly cookie if XSS exposure matters
export const auth = $state({ token: localStorage.getItem('token') });

export function setToken(token: string | null) {
	auth.token = token;
	if (token) localStorage.setItem('token', token);
	else localStorage.removeItem('token');
}

/** Calls the backend through the dev proxy; throws a SvelteKit HttpError on failure. */
export async function api<T>(path: string, init: RequestInit = {}): Promise<T> {
	const res = await fetch(`/api${path}`, {
		...init,
		headers: {
			'Content-Type': 'application/json',
			...(auth.token && { Authorization: `Bearer ${auth.token}` })
		}
	});
	const body = await res.json().catch(() => ({}));
	if (res.status === 401 && auth.token) {
		setToken(null);
		goto('/login');
	}
	if (!res.ok) error(res.status, body.error ?? `Request failed (${res.status})`);
	return body;
}

export async function authenticate(path: '/signup' | '/login', email: string, password: string) {
	const { token } = await api<{ token: string }>(path, {
		method: 'POST',
		body: JSON.stringify({ email, password })
	});
	setToken(token);
}
