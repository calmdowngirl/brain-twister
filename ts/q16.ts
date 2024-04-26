// 20 April 2024|New Scientist|45
// arrange the digits 1-9 in a line so that each pair of adjacent digits differs by either 2 or 3

const COLLECTION = [1, 2, 3, 4, 5, 6, 7, 8, 9] as const

const result: string[] = []

interface Tree {
  root: number
  children?: Tree[]
}

export function solve(): void {
  COLLECTION.forEach((elem) => traverse(elem))
  console.log(`there r ${result.length} ways to arrange the digits`)
  result.forEach((s) => console.log(s))
}

function traverse(node: number, visited: number[] = []): Tree {
  visited = [...visited, node]
  if (visited.length === 9) result.push(visited.join(","))
  return {
    root: node,
    children: getChildrenValues(node)
      ?.filter((elem) => !visited.includes(elem))
      .map((elem) => traverse(elem, visited)),
  }
}

function getChildrenValues(n: number): number[] | null {
  const values: number[] = []
  if (n - 2 >= 1) values.push(n - 2)
  if (n - 3 >= 1) values.push(n - 3)
  if (n + 2 <= 9) values.push(n + 2)
  if (n + 3 <= 9) values.push(n + 3)
  return values.length ? values : null
}
