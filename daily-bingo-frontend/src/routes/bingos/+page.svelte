<script lang="ts">
	import BingoGrid from '$lib/BingoGrid.svelte';

	let { data } = $props();
</script>

<h1>Your Bingos</h1>
<p><a class="button" href="/bingos/new">New Bingo</a></p>

{#if data.bingos.length === 0}
	<p>No bingos yet.</p>
{/if}

<ul>
	{#each data.bingos as bingo (bingo.id)}
		<li>
			<a href="/bingos/{bingo.id}"><BingoGrid {bingo} small /></a>
			<span>{bingo.cols}×{bingo.rows}</span>
			<span class="actions">
				<a class="button" href="/bingos/{bingo.id}/play">Play</a>
				<a class="button" href="/bingos/{bingo.id}/edit">Edit</a>
			</span>
		</li>
	{/each}
</ul>

<style>
	ul { list-style: none; padding: 0; display: grid; grid-template-columns: repeat(auto-fill, minmax(11rem, 1fr)); gap: 1rem; }
	li { display: flex; flex-direction: column; gap: 0.5rem; align-items: start; }
	.actions { display: flex; gap: 0.5rem; }
</style>
