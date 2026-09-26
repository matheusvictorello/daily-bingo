// Run: npx tsx src/lib/bingo.check.ts
import { winningLines, type Cell } from './bingo';

const f: Cell = { Filled: 'x' };
// x x x
// x F x   (F = Empty/free)
// x _ x   (_ = Gap)
const b = { cols: 3, rows: 3, values: [f, f, f, f, 'Empty', f, f, 'Gap', f] as Cell[] };
const m = (...idx: number[]) => Array.from({ length: 9 }, (_, i) => idx.includes(i));
const eq = (a: unknown, e: unknown) => {
	if (JSON.stringify(a) !== JSON.stringify(e)) throw new Error(`${JSON.stringify(a)} !== ${JSON.stringify(e)}`);
};

eq(winningLines(b, m()), []);
eq(winningLines(b, m(1)), [[1, 4, 7]]); // column through free space + gap
eq(winningLines(b, m(0, 8)), [[0, 4, 8]]); // diagonal through free space
eq(winningLines(b, m(3, 5)), [[3, 4, 5]]); // row through free space
eq(winningLines(b, m(0, 3, 6)), [[0, 3, 6]]); // plain column
eq(winningLines(b, m(0, 1, 2, 3, 5, 6, 8)).length, 8); // blackout
console.log('ok');
