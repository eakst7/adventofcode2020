import * as fs from 'fs'

function readInputData() : string[] {
  const data: string = fs.readFileSync('input.txt', 'utf8')
  var lines: string[] = data.split('\n')

  return lines
}

function findSolutionPart1(lines: string[]) {
  for (let i = 0; i < lines.length; i++) {
    let ni = Number(lines[i])
    for (let j = i+1; j < lines.length; j++) {
      let nj: number  = Number(lines[j])
      let sum: number  = ni + nj
      if (sum === 2020) {
        let product = ni * nj
        console.log(ni + " + " + nj + " = " + sum)
        console.log(ni + " * " + nj + " = " + product)
        return
      }
    }
  }
}

function findSolutionPart2(lines: string[]) {
  for (let i = 0; i < lines.length; i++) {
    let ni = Number(lines[i])
    for (let j = i+1; j < lines.length; j++) {
      let nj: number  = Number(lines[j])
      for (let k = j+1; k < lines.length; k++) {
        let nk: number = Number(lines[k])
        let sum: number  = ni + nj + nk
        if (sum === 2020) {
          let product = ni * nj * nk
          console.log(ni + " + " + nj + " + " + nk + " = " + sum)
          console.log(ni + " * " + nj + " * " + nk + " = " + product)
          return
        }
  
      }
    }
  }
}


let lines: string[] = readInputData()
findSolutionPart1(lines)
findSolutionPart2(lines)
