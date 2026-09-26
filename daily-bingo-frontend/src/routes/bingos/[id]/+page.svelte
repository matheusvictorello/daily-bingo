<script lang="ts">
	import BingoGrid from '$lib/BingoGrid.svelte';

	let { data } = $props();
</script>

<p><a href="/bingos">← All bingos</a></p>
<h1>{data.bingo.cols}×{data.bingo.rows} Bingo</h1>
<BingoGrid bingo={data.bingo} />
<p><a class="button" href="/bingos/{data.bingo.id}/play">Play</a></p>
