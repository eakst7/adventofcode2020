import * as fs from 'fs'

function readInputData() : string[] {
  const data: string = fs.readFileSync('input.txt', 'utf8')
  var lines: string[] = data.split('\n')

  return lines
}

class Grid {
  private _grid: string[][]
  readonly y_size: number
  readonly x_size: number

  constructor(lines: string[]) {
    this.y_size = lines.length
    this.x_size = lines[0].length

    this._grid = new Array(this.x_size);
    for (let i = 0; i < this.x_size; i++) {
      this._grid[i] = new Array(this.y_size)
    }

    for (let y = 0; y < lines.length; y++) {
      for (let x = 0; x < lines[y].length; x++) {
        this._grid[x][y] = lines[y].charAt(x)
      }
    }
  }

  public getat(x: number, y: number) : string {
    x = x % this._grid.length;
    return this._grid[x][y]
  }

}

function countHits(grid: Grid, slope_x: number, slope_y: number) : number {
  let tree_count = 0
  let y_pos = 0
  let x_pos = 0

  while (y_pos < grid.y_size) {
    y_pos += slope_y
    x_pos += slope_x

    if (grid.getat(x_pos,y_pos) === '#') {
      tree_count++
    }
  }

  return tree_count
}

function findSolutionPart1(lines: string[]) {
  let grid: Grid = new Grid(lines)

  console.log(`Total trees hit: ${countHits(grid, 3, 1)}`)
}

function findSolutionPart2(lines: string[]) {
  let grid: Grid = new Grid(lines)

  let product = 
    countHits(grid, 1, 1) *
    countHits(grid, 3, 1) *
    countHits(grid, 5, 1) *
    countHits(grid, 7, 1) *
    countHits(grid, 1, 2)

  console.log(`Product: ${product}`)
}

let lines: string[] = readInputData()
findSolutionPart1(lines)
findSolutionPart2(lines)
