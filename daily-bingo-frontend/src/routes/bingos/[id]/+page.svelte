<script lang="ts">
	import BingoGrid from '$lib/BingoGrid.svelte';

	let { data } = $props();
</script>

<a class="back" href="/bingos">← All bingos</a>
<h1>{data.bingo.cols}×{data.bingo.rows} Bingo</h1>
<BingoGrid bingo={data.bingo} />
<div class="actions">
	<a class="button" href="/bingos/{data.bingo.id}/play">Play</a>
	<a class="button secondary" href="/bingos/{data.bingo.id}/edit">Edit</a>
</div>
