<script lang="ts">
	import { untrack } from 'svelte';
	import { isHttpError } from '@sveltejs/kit';
	import type { BingoInfo, Cell } from '$lib/bingo';

	const MAX = 10;

	let {
		initial = { cols: 5, rows: 5, values: Array(25).fill('Empty') },
		submitLabel,
		onsave
	}: {
		initial?: BingoInfo;
		submitLabel: string;
		onsave: (info: BingoInfo) => Promise<void>;
	} = $props();

	// Editor state starts from `initial` and is owned here afterwards.
	const start = untrack(() => initial);
	let cols = $state(start.cols);
	let rows = $state(start.rows);
	let values = $state<Cell[]>([...start.values]);
	let error = $state('');
	let saving = $state(false);

	/** Keeps each cell at its (row, col) position; new cells start Empty. */
	function resize(c: number, r: number) {
		if (!(c >= 1 && r >= 1)) return;
		c = Math.min(c, MAX);
		r = Math.min(r, MAX);
		values = Array.from({ length: r * c }, (_, i) => {
			const y = Math.floor(i / c);
			const x = i % c;
			return y < rows && x < cols ? values[y * cols + x] : 'Empty';
		});
		cols = c;
		rows = r;
	}

	function setText(i: number, text: string) {
		values[i] = text.trim() ? { Filled: text } : 'Empty';
	}

	async function submit(e: SubmitEvent) {
		e.preventDefault();
		error = '';
		saving = true;
		try {
			await onsave({ cols, rows, values });
		} catch (err) {
			error = isHttpError(err) ? err.body.message : 'Could not reach the server';
		} finally {
			saving = false;
		}
	}
</script>

<form onsubmit={submit}>
	<div class="size">
		<label>
			Columns
			<input type="number" min="1" max={MAX} value={cols} oninput={(e) => resize(e.currentTarget.valueAsNumber, rows)} />
		</label>
		<label>
			Rows
			<input type="number" min="1" max={MAX} value={rows} oninput={(e) => resize(cols, e.currentTarget.valueAsNumber)} />
		</label>
	</div>

	<p class="hint muted">Blank cells are free spaces. ✕ turns a cell into a gap.</p>

	<div class="grid-scroll">
		<div class="grid" style:grid-template-columns="repeat({cols}, var(--cell))">
			{#each values as cell, i (i)}
				{#if cell === 'Gap'}
					<button type="button" class="cell gap" aria-label="Restore cell {i + 1}" onclick={() => (values[i] = 'Empty')}>+</button>
				{:else}
					<div class="cell">
						<textarea
							aria-label="Cell {i + 1}"
							placeholder="Free"
							value={cell === 'Empty' ? '' : cell.Filled}
							oninput={(e) => setText(i, e.currentTarget.value)}
						></textarea>
						<button type="button" class="remove" aria-label="Make cell {i + 1} a gap" onclick={() => (values[i] = 'Gap')}>✕</button>
					</div>
				{/if}
			{/each}
		</div>
	</div>

	{#if error}<p role="alert">{error}</p>{/if}
	<div class="actions"><button disabled={saving}>{submitLabel}</button></div>
</form>

<style>
	.size { display: flex; gap: 1rem; }
	.size input { width: 5rem; }
	.hint { margin: 1rem 0; font-size: 0.9rem; }
	.grid { display: grid; gap: var(--cell-gap); width: max-content; }
	.cell { position: relative; width: var(--cell); height: var(--cell); }
	.cell textarea {
		width: 100%; height: 100%; resize: none; padding: 0.35rem;
		font-size: 0.85rem; line-height: 1.2; text-align: center;
	}
	.cell textarea:placeholder-shown { background: var(--surface-muted); }
	.cell textarea:focus { border-color: var(--primary); outline: none; }
	.remove {
		position: absolute; top: 2px; right: 2px; padding: 0 0.3rem; border: none;
		background: none; color: var(--text-muted); font-size: 0.7rem; font-weight: 400;
	}
	.remove:hover { background: none; color: var(--danger); }
	.gap {
		width: var(--cell); height: var(--cell); border: 1px dashed var(--border);
		background: none; color: var(--text-muted); font-size: 1.25rem; font-weight: 400;
	}
	.gap:hover { background: var(--surface-muted); border-color: var(--primary); color: var(--primary); }
</style>
