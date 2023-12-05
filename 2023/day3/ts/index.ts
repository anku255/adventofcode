import { readFile } from "node:fs/promises";
import { argv } from "node:process";

async function getLines() {
  if(!argv[2]) throw new Error('No input file specified! Usage: npm start <input_file>');
  return (await readFile(argv[2], "utf8")).toString().split("\n");
}

type GridItem = { x: number; y: number; value: string };

function parseGrid(lines: string[]) {
  const nums: Array<GridItem> = [];
  const symbols: Array<GridItem> = [];
  let numXIndex = -1;
  let num = "";
  for (let y = 0; y < lines.length; y++) {
    for (let x = 0; x <= lines[y].length - 1; x++) {
      const char = lines[y][x];
      if (Number.isInteger(Number(char))) {
        num += char;
        numXIndex = numXIndex === -1 ? x : numXIndex;
        if (x < lines[y].length - 1) {
          continue;
        }
      } else if (char === ".") {
        // do nothing
      } else {
        symbols.push({ x, y, value: char });
      }

      if (numXIndex !== -1) {
        nums.push({ x: numXIndex, y, value: num });
        numXIndex = -1;
        num = "";
      }
    }
  }
  return { nums, symbols };
}

function findPartsAndGears(lines: string[]) {
  const { nums, symbols } = parseGrid(lines);

  const parts: Array<GridItem> = [];
  const gears: Array<GridItem & { gearRatio: number }> = [];
  for (const symbol of symbols) {
    let adjacentPartsCount = 0;
    let gearRatio = 1;
    for (const num of nums) {
      const boundary = {
        min: { x: num.x - 1, y: num.y - 1 },
        max: { x: num.x + num.value.length, y: num.y + 1 },
      };

      const isSymbolWithinBoundary =
        symbol.x >= boundary.min.x &&
        symbol.y >= boundary.min.y &&
        symbol.x <= boundary.max.x &&
        symbol.y <= boundary.max.y;

      if (isSymbolWithinBoundary) {
        parts.push(num);
        adjacentPartsCount++;
        gearRatio *= Number(num.value);
      }
    }

    if(adjacentPartsCount === 2) {
      gears.push({ ...symbol, gearRatio });
    }
  }

  return {parts, gears };
}

export async function part1(lines: string[]) {
  const { parts } = findPartsAndGears(lines);
  const result = parts
    .reduce((acc, curr) => acc + Number(curr.value), 0);
  return result;
}

export async function part2(lines: string[]) {
  const { gears } = findPartsAndGears(lines);
  const result = gears.reduce((acc, curr) => acc + curr.gearRatio, 0);
  return result;
}

async function main() {
  const lines = await getLines();
  const part1Result = await part1(lines);
  const part2Result = await part2(lines);

  console.log({
    part1Result,
    part2Result,
  });
}

main();
