// ponytail: token in localStorage, move to httpOnly cookie if XSS exposure matters
export const auth = $state({ token: localStorage.getItem('token') });

export function setToken(token: string | null) {
	auth.token = token;
	if (token) localStorage.setItem('token', token);
	else localStorage.removeItem('token');
}

export async function authenticate(path: '/signup' | '/login', email: string, password: string) {
	const res = await fetch(`/api${path}`, {
		method: 'POST',
		headers: { 'Content-Type': 'application/json' },
		body: JSON.stringify({ email, password })
	});
	const body = await res.json().catch(() => ({}));
	if (!res.ok) throw new Error(body.error ?? `Request failed (${res.status})`);
	setToken(body.token);
}
