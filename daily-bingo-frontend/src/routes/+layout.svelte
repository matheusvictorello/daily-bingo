<script lang="ts">
	import favicon from '$lib/assets/favicon.svg';
	import { goto } from '$app/navigation';
	import { auth, setToken } from '$lib/auth.svelte';

	let { children } = $props();

	function logout() {
		setToken(null);
		goto('/login');
	}
</script>

<svelte:head>
	<link rel="icon" href={favicon} />
</svelte:head>

<nav>
	<a href="/">Daily Bingo</a>
	<span>
		{#if auth.token}
			<button onclick={logout}>Logout</button>
		{:else}
			<a href="/login">Login</a>
			<a href="/signup">Signup</a>
		{/if}
	</span>
</nav>

<main>
	{@render children()}
</main>

<style>
	nav { display: flex; justify-content: space-between; align-items: center; padding: 0.75rem 1rem; border-bottom: 1px solid #ddd; }
	nav span { display: flex; gap: 1rem; align-items: center; }
	main { padding: 1rem; }
</style>
