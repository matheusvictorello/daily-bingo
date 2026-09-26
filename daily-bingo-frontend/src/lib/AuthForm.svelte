<script lang="ts">
	import { isHttpError } from '@sveltejs/kit';
	import { goto } from '$app/navigation';
	import { authenticate } from '$lib/auth.svelte';

	let { path, title }: { path: '/signup' | '/login'; title: string } = $props();

	let email = $state('');
	let password = $state('');
	let error = $state('');
	let loading = $state(false);

	async function submit(e: SubmitEvent) {
		e.preventDefault();
		error = '';
		loading = true;
		try {
			await authenticate(path, email, password);
			await goto('/bingos');
		} catch (err) {
			error = isHttpError(err) ? err.body.message : 'Could not reach the server';
		} finally {
			loading = false;
		}
	}
</script>

<form class="card" onsubmit={submit}>
	<h1>{title}</h1>
	<label>Email <input type="email" bind:value={email} required autocomplete="email" /></label>
	<label>
		Password
		<input
			type="password"
			bind:value={password}
			required
			autocomplete={path === '/signup' ? 'new-password' : 'current-password'}
		/>
	</label>
	{#if error}<p role="alert">{error}</p>{/if}
	<button disabled={loading}>{title}</button>
</form>

<style>
	form { display: flex; flex-direction: column; gap: 1rem; max-width: 24rem; margin: 2rem auto; }
	h1 { margin: 0; }
</style>
