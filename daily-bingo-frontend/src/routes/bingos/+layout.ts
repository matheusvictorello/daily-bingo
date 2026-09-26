import { redirect } from '@sveltejs/kit';
import { auth } from '$lib/auth.svelte';

export const load = () => {
	if (!auth.token) redirect(307, '/login');
};
