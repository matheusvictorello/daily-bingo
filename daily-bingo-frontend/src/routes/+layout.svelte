<script lang="ts">
	import '../app.css';
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
	<title>Daily Bingo</title>
	<link rel="icon" href={favicon} />
</svelte:head>

<nav>
	<a class="brand" href="/bingos">Daily Bingo</a>
	<span>
		{#if auth.token}
			<a href="/bingos">Bingos</a>
			<button class="secondary" onclick={logout}>Logout</button>
		{:else}
			<a href="/login">Login</a>
			<a class="button" href="/signup">Signup</a>
		{/if}
	</span>
</nav>

<main>
	{@render children()}
</main>

<style>
	nav {
		position: sticky;
		top: 0;
		z-index: 1;
		display: flex;
		justify-content: space-between;
		align-items: center;
		padding: 0.75rem 1rem;
		border-bottom: 1px solid var(--border);
		background: var(--surface);
	}
	nav span { display: flex; gap: 1rem; align-items: center; }
	nav a:not(:global(.button)) { font-weight: 500; text-decoration: none; }
	.brand { font-size: 1.15rem; font-weight: 800; letter-spacing: -0.02em; }
	main { max-width: 64rem; margin: 0 auto; padding: 1.5rem 1rem 3rem; }
</style>
